from dataclasses import dataclass
from typing import List, Dict, Iterable, Any, Tuple, TYPE_CHECKING, Union, Optional

import numpy as np

from drepr.models import Alignment, defaultdict, RangeAlignment, ClassNode

from drepr.outputs.array_backend.array_attr import Attribute, ArrayAttr, ScalarAttr
from drepr.outputs.array_backend.array_predicate import ArrayObjectPredicate, ArrayDataPredicate, ArrayPredicate
from drepr.outputs.array_backend.subset_array_class import SubsetArrayClass
from drepr.outputs.base_output_class import BaseOutputClass
from drepr.outputs.base_output_predicate import BaseOutputPredicate
from drepr.outputs.base_output_sm import FConditionOp, FCondition
from drepr.outputs.record_id import RecordID, URIRecordID, BlankRecordID

if TYPE_CHECKING:
    from drepr.outputs.array_backend.array_backend import ArrayBackend
from drepr.outputs.array_backend.array_record import ArrayRecord
from drepr.outputs.array_backend.index_map_func import O2ORange0Func, X2OFunc, IdentityFunc, X2MFunc


class ArrayClass(BaseOutputClass):
    def __init__(self, backend: 'ArrayBackend', node_id: str):
        self.backend = backend
        # the original semantic model
        self.sm = backend.sm
        # @inherit: id of the node in the semantic model
        self.id = node_id
        # uri of the class
        self.uri = backend.sm.nodes[node_id].label
        # id of the attribute which is selected as the subject of the class
        self.pk_attr_id: str
        # the attribute which is selected as the subject of the class
        self.pk_attr: Attribute
        # if of the attribute which contains URIs of the class
        self.uri_attr_id: Optional[str] = None
        # the wrapper of the attribute which contains URIs of the class.
        self.uri_attr: PolymorphismAttribute
        # a mapping from predicate uris to the list of predicates of this class
        self.predicates: Dict[str, Union[ArrayDataPredicate, ArrayObjectPredicate]] = {}
        # the list of wrapped attributes which contains data of predicates of this class
        self.attrs: List[PolymorphismAttribute] = []
        # a mapping from predicate uris to the index of the wrapped attributes
        self.pred2attrs: Dict[str, List[int]] = defaultdict(list)

    def _init_schema(self):
        uri2edges = defaultdict(list)
        uri2type = {}
        for e in self.sm.iter_outgoing_edges(self.id):
            if isinstance(self.sm.nodes[e.target_id], ClassNode):
                if e.label in uri2type:
                    assert uri2type[e.label] == "object", "Violate the assumption"
                else:
                    uri2type[e.label] = "object"
                uri2edges[e.label].append(e)
            else:
                if e.label == 'drepr:uri':
                    self.uri_attr_id = e.target_id
                else:
                    if e.label in uri2type:
                        assert uri2type[e.label] == "data", "Violate the assumption"
                    else:
                        uri2type[e.label] = "data"
                    uri2edges[e.label].append(e)

            if e.is_subject:
                # this requires us to analyze the d-repr output first.
                self.pk_attr_id = e.target_id

        for uri, edges in uri2edges.items():
            if uri2type[uri] == "object":
                self.predicates[uri] = ArrayObjectPredicate(self.backend, edges)
            else:
                self.predicates[uri] = ArrayDataPredicate(self.backend, edges)

    def _init_data(self):
        self.pk_attr = self.backend.attrs[self.pk_attr_id]

        if self.uri_attr_id is None:
            # blank class
            self.uri_attr = PolymorphismAttribute(self.pk_attr, IdentityFunc(), True, True, self.id)
        else:
            if self.pk_attr_id == self.uri_attr_id:
                imfunc = IdentityFunc()
            else:
                imfunc = self._get_imfunc(self.backend.alignments[self.pk_attr.id, self.uri_attr_id])
            self.uri_attr = PolymorphismAttribute(self.backend.attrs[self.uri_attr_id],
                                                  imfunc,
                                                  True, False, self.id)

        # contains both values of data property and object property.
        self.attrs = []
        self.pred2attrs: Dict[str, List[int]] = defaultdict(list)
        for p in self.predicates.values():
            if isinstance(p, ArrayDataPredicate):
                for i, e in enumerate(p.edges):
                    self.pred2attrs[p.uri].append(len(self.attrs))

                    if self.pk_attr.id == p.attr(i).id:
                        imfunc = IdentityFunc()
                    else:
                        imfunc = self._get_imfunc(self.backend.alignments[self.pk_attr.id, p.attr(i).id])
                    self.attrs.append(PolymorphismAttribute(
                        p.attr(i), imfunc, True
                    ))
            else:
                p._init(self.backend)
                for i, e in enumerate(p.edges):
                    self.pred2attrs[p.uri].append(len(self.attrs))
                    imfunc = self._get_imfunc(self.backend.alignments[self.pk_attr.id, p.attr(i).id])
                    self.attrs.append(PolymorphismAttribute(
                        p.attr(i), imfunc,
                        imfunc.is_x2o, p.targets[i].is_blank(), p.targets[i].id,
                    ))

    def is_blank(self) -> bool:
        return self.uri_attr_id is None

    def iter_records(self):
        """
        Iterate through all records of this class
        """
        if isinstance(self.pk_attr, ArrayAttr):
            if self.pk_attr.nodata is not None:
                for index in np.ndindex(*self.pk_attr.values_shp):
                    if self.uri_attr.get_sval(index) is None:
                        continue
                    yield ArrayRecord(index, self)
            else:
                for index in np.ndindex(*self.pk_attr.values_shp):
                    yield ArrayRecord(index, self)
        else:
            yield ArrayRecord((), self)

    def get_record_by_id(self, rid: RecordID):
        """
        Get a record of the class by id
        """
        return ArrayRecord(rid.index, self)

    def p(self, predicate_uri: str) -> Optional[BaseOutputPredicate]:
        return self.predicates.get(predicate_uri, None)

    def filter(self, conditions: List[FCondition]) -> 'ArrayClass':
        if type(conditions) is list:
            if len(conditions) > 1:
                raise NotImplementedError()
            condition = conditions[0]
        else:
            condition = conditions

        if condition.op == FConditionOp.eq:
            matched_attrs = [self.attrs[a] for a in self.pred2attrs[condition.predicate_uri]]
            if sum((a.attr.size for a in matched_attrs)) == 1:
                val = [a.get_first_value() for a in matched_attrs][0]
                if val == condition.val:
                    return self
                return SubsetArrayClass(self, [])

        raise NotImplementedError()

    def group_by(self, predicate: Union[str, ArrayPredicate]) -> Iterable[Tuple[Any, 'ArrayClass']]:
        """Group by predicate (can be string or predicate id)"""
        if isinstance(predicate, str):
            # perform group by the predicate uri
            if len(self.pred2attrs[predicate]) > 1:
                raise Exception("Key of group_by operator cannot be a list")
            # p = self.p(predicate)[0]
            o = self.attrs[self.pred2attrs[predicate][0]]
        else:
            o = predicate.attr

        if o.size == 1:
            yield o.get_first_value(), self
        else:
            groups = defaultdict(set)
            for id in np.ndindex(self.pk_attr.shape):
                for val in o.get_mval(id):
                    groups[val].add(id)
            for val, ids in groups.items():
                yield val, SubsetArrayClass(self, ids)

    def _get_imfunc(self, alignments: List[Alignment]) -> Union[X2OFunc, X2MFunc]:
        # constraint of the array-backend class, we have it until we figure out how to
        # handle chained join or value join efficiency. This constraint is supposed to
        # be always satisfied since it has been checked before writing data to array-backend
        # format
        assert len(alignments) == 1 and isinstance(alignments[0], RangeAlignment)
        # have to remove fixed dimension (dimension of just one element)
        # e.g.: $.[:].lat.data[0][:][:] => 3 dims: (0, 4, 5) => remove to be (0, 1, 2)
        # the shift will  be: (0, 3, 3)
        # instead we can rely on the step2dim
        # TODO: fix me! handle the attribute id is not ideal.
        if alignments[0].source not in self.backend.attrs:
            attr = self.backend.attrs[f'dnode:{alignments[0].source}']
        else:
            attr = self.backend.attrs[alignments[0].source]

        if isinstance(attr, ScalarAttr):
            assert len(alignments[0].aligned_steps) == 0
            step2dim = []
        else:
            step2dim = attr.step2dim
        target2source = [step2dim[s.source_idx] for s in sorted(alignments[0].aligned_steps, key=lambda s: s.target_idx)]
        return O2ORange0Func(target2source)


@dataclass
class PolymorphismAttribute:
    """
    Represent attribute of one of possible three types: data property, blank object property, and uri object property
    """
    attr: Attribute
    im_func: Union[X2OFunc, X2MFunc]
    is_x2o_func: bool
    is_blank: bool = False
    class_id: str = None

    @property
    def size(self):
        return self.attr.size

    def get_first_value(self):
        zero_idx = next(np.ndindex(*self.attr.shape))
        if self.class_id is not None:
            if self.is_blank:
                return BlankRecordID(zero_idx, self.class_id)
            else:
                return RecordID(self.attr.get_value(zero_idx), zero_idx, self.class_id)
        return self.attr.get_value(zero_idx)

    def get_sval(self, index: Tuple[int, ...]):
        if self.class_id is not None:
            if self.is_blank:
                return BlankRecordID(
                    self.im_func(index) if self.is_x2o_func else next(self.im_func(index)),
                    self.class_id)
            else:
                if self.is_x2o_func:
                    idx = self.im_func(index)
                    val = self.attr.get_value(idx)
                    if self.attr.nodata is not None and self.attr.nodata.value == val:
                        return None
                else:
                    if self.attr.nodata is not None:
                        for idx in next(self.im_func(index)):
                            val = self.attr.get_value(idx)
                            if val != self.attr.nodata.value:
                                break
                        else:
                            return None
                    else:
                        idx = next(self.im_func(index))
                        val = self.attr.get_value(idx)

                return URIRecordID(val, idx, self.class_id)
        else:
            if self.is_x2o_func:
                val = self.attr.get_value(self.im_func(index))
                if self.attr.nodata is not None and val == self.attr.nodata.value:
                    return None
                return val

            if self.attr.nodata is not None:
                for val in self.im_func(index):
                    if val != self.attr.nodata.value:
                        return val
            else:
                return self.attr.get_value(next(self.im_func(index)))

    def get_mval(self, index: Tuple[int, ...]):
        if self.class_id is not None:
            if self.is_blank:
                if self.is_x2o_func:
                    return [BlankRecordID(self.im_func(index), self.class_id)]
                else:
                    return [BlankRecordID(x, self.class_id) for x in self.im_func(index)]
            if self.is_x2o_func:
                idx = self.im_func(index)
                val = self.attr.get_value(idx)
                if self.attr.nodata is not None and val == self.attr.nodata.value:
                    return []
                return [URIRecordID(val, idx, self.class_id)]

            if self.attr.nodata is not None:
                lst = []
                for x in self.im_func(index):
                    val = self.attr.get_value(x)
                    if val == self.attr.nodata.value:
                        continue
                    lst.append(URIRecordID(val, x, self.class_id))
                return lst
            else:
                return [URIRecordID(self.attr.get_value(x), x, self.class_id) for x in self.im_func(index)]
        else:
            if self.is_x2o_func:
                val = self.attr.get_value(self.im_func(index))
                if self.attr.nodata is not None and val == self.attr.nodata.value:
                    return []
                return [val]
            return [val for val in (self.attr.get_value(x) for x in self.im_func(index)) if val is not None]

    def set_value(self, index: Tuple[int, ...], val: Any):
        if self.is_blank:
            raise Exception("Cannot assign value to blank instances")
        if self.is_x2o_func:
            self.attr.set_value(self.im_func(index), val)
        else:
            # expect value to be an iterator
            for i, x in enumerate(self.im_func(index)):
                self.attr.set_value(x, next(val))
