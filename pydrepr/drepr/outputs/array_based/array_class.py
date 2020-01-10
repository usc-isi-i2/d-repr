import numpy as np
from dataclasses import dataclass
from typing import List, Dict, Iterable, Any, Tuple, TYPE_CHECKING, Union

from drepr.models import Alignment, defaultdict, RangeAlignment
from drepr.outputs.array_based.array_attr import Attribute
if TYPE_CHECKING:
    from drepr.outputs.array_based.array_backend import ArrayBackend
from drepr.outputs.array_based.array_record import ArrayRecord
from drepr.outputs.array_based.index_map_func import O2ORange0Func, X2OFunc, IdentityFunc, X2MFunc
from drepr.outputs.array_based.indexed_sm import IndexedSM, SMClass, DataProp, ObjectProp
from drepr.outputs.array_based.types import AttrID, RecordID


class ArrayClass:
    def __init__(self, backend: 'ArrayBackend', cls: SMClass):
        self.cls = cls
        self.id = cls.node_id
        self.label = cls.label

        self.pk_attr = backend.attrs[cls.pk_attr]

        if cls.is_blank():
            self.uri_attr = PolymorphismAttribute(self.pk_attr, IdentityFunc, True, True)
        else:
            self.uri_attr = PolymorphismAttribute(backend.attrs[cls.uri_attr],
                                                  self._get_imfunc(backend.alignments[self.pk_attr.id, cls.uri_attr]),
                                                  True, False)

        # contains both values of data property and object property.
        self.attrs = []
        self.pred2attrs: Dict[str, List[int]] = defaultdict(lambda: [])
        for lst in cls.predicates.values():
            for p in lst:
                self.pred2attrs[p.label].append(len(self.attrs))
                if isinstance(p, DataProp):
                    self.attrs.append(PolymorphismAttribute(
                        backend.attrs[p.data_id],
                        self._get_imfunc(backend.alignments[self.pk_attr.id, p.data_id]),
                        True, False
                    ))
                else:
                    p: ObjectProp
                    if p.object.is_blank():
                        imfunc = self._get_imfunc(backend.alignments[self.pk_attr.id, p.object.pk_attr])
                    else:
                        imfunc = self._get_imfunc(backend.alignments[self.pk_attr.id, p.object.uri_attr])
                    self.attrs.append(PolymorphismAttribute(
                        backend.attrs[p.object.uri_attr],
                        imfunc,
                        imfunc.is_x2o, p.object.is_blank()
                    ))

    def iter_records(self):
        """
        Iterate through all records of this class
        """
        shape = self.pk_attr.shape
        if len(shape) == 0:
            yield ArrayRecord((0,), self)
        elif len(shape) == 1:
            for i in range(shape[0]):
                yield ArrayRecord((i,), self)
        elif len(shape) == 2:
            for i in range(shape[0]):
                for j in range(shape[1]):
                    yield ArrayRecord((i, j), self)
        elif len(shape) == 3:
            for i in range(shape[0]):
                for j in range(shape[1]):
                    for k in range(shape[2]):
                        yield ArrayRecord((i, j, k), self)
        elif len(shape) == 4:
            for i in range(shape[0]):
                for j in range(shape[1]):
                    for k in range(shape[2]):
                        for p in range(shape[3]):
                            yield ArrayRecord((i, j, k, p), self)
        else:
            rid = [0] * len(shape)
            for r in self._recur_iter_record(shape, 0, rid):
                yield r

    def get_record_by_id(self, rid: RecordID):
        """
        Get a record of the class by id
        """
        return ArrayRecord(rid, self)

    def get_predicates_as_ndarray(self, pred_id: int, index_preds: List[int]) -> np.ndarray:
        """
        Get edge data (identified by `edge_id` as a high-dimensional array). The original data may already be in high-dimension
        array or may be not, but the returned value must be a high-dimensional array.

        The supplied `index_edges` are list of edges that will occupied first dimensions. If an edge in index_edges are
        high-dimensional array as well, then its value will be flatten. Each

        There must be an alignment between the edge_id and other index edges (the alignment represent the join). The alignment
        must be dimension alignment for now (then we don't need to do a join but only swapping and arranging dimension). In case the
        alignment are chained, then we have to join, and create new table?
        """
        pass

    def filter(self, condition) -> 'ArrayClass':
        pass

    def group_by(self, predicate: str) -> Iterable[Tuple[Any, 'ArrayClass']]:
        # perform group by the predicate id
        if len(self.pred2attrs[predicate]) > 1:
            raise Exception("Key of group_by operator cannot be a list")

        if self.cls.is_object_prop(predicate):
            pass
            # guarantee
        # if self.pred2attrs[predicate]
        # self.pred2attrs[predicate]
        pass

    def _get_imfunc(self, alignments: List[Alignment]) -> Union[X2OFunc, X2MFunc]:
        # constraint of the array-backend class, we have it until we figure out how to
        # handle chained join or value join efficiency. This constraint is supposed to
        # be always satisfied since it has been checked before writing data to array-backend
        # format
        assert len(alignments) == 1 and isinstance(alignments[0], RangeAlignment)
        # TODO: fix me! bug if there is missing values
        target2source = [s.source_idx - 1 for s in sorted(alignments[0].aligned_steps, key=lambda s: s.target_idx)]
        return O2ORange0Func(target2source)

    def _recur_iter_record(self, shp: Tuple[int, ...], dim: int, rid: List[int, ...]):
        if dim == len(shp) - 1:
            for i in range(shp[dim]):
                rid[dim] = i
                yield ArrayRecord(rid, self)
        else:
            for i in range(shp[dim]):
                rid[dim] = i
                for r in self._recur_iter_record(shp, dim+1, rid):
                    yield r


@dataclass
class PolymorphismAttribute:
    """
    Represent attribute of one of possible three types: data property, blank object property, and uri object property
    """
    attr: Attribute
    im_func: Union[X2OFunc, X2MFunc]
    is_x2o_func: bool
    is_blank: bool

    def get_sval(self, id: RecordID):
        if self.is_blank:
            if self.is_x2o_func:
                return self.im_func(id)
            else:
                return next(self.im_func(id))
        if self.is_x2o_func:
            return self.attr.get_value(self.im_func(id))
        return self.attr.get_value(next(self.im_func(id)))

    def get_mval(self, id: RecordID):
        if self.is_blank:
            if self.is_x2o_func:
                return [self.im_func(id)]
            else:
                return list(self.im_func(id))
        if self.is_x2o_func:
            return [self.attr.get_value(self.im_func(id))]
        return [self.attr.get_value(x) for x in self.im_func(id)]

    def set_value(self, id: RecordID, val: Any):
        if self.is_blank:
            raise Exception("Cannot assign value to blank instances")
        if self.is_x2o_func:
            self.attr.set_value(self.im_func(id), val)
        else:
            # expect value to be an iterator
            for i, x in enumerate(self.im_func(id)):
                self.attr.set_value(x, next(val))
