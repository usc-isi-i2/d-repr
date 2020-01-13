import numpy as np
from dataclasses import dataclass
from typing import List, Dict, Iterable, Any, Tuple, TYPE_CHECKING, Union

from drepr.models import Alignment, defaultdict, RangeAlignment
from drepr.outputs.array_based.array_attr import Attribute
from drepr.outputs.array_based.base_array_class import BaseArrayClass
from drepr.outputs.array_based.filtered_array_class import FilteredArrayClass
from drepr.outputs.array_based.record_id import RecordID, BlankRecordID
from drepr.outputs.prop_data_ndarray import PropDataNDArray, IndexPropRange

if TYPE_CHECKING:
    from drepr.outputs.array_based.array_backend import ArrayBackend
from drepr.outputs.array_based.array_record import ArrayRecord
from drepr.outputs.array_based.index_map_func import O2ORange0Func, X2OFunc, IdentityFunc, X2MFunc
from drepr.outputs.array_based.indexed_sm import SMClass, DataProp, ObjectProp


class ArrayClass(BaseArrayClass):
    def __init__(self, backend: 'ArrayBackend', cls: SMClass):
        self.cls = cls
        self.backend = backend
        self.id = cls.node_id
        self.label = cls.label

        self.pk_attr = backend.attrs[cls.pk_attr]

        if cls.is_blank():
            self.uri_attr = PolymorphismAttribute(self.pk_attr, IdentityFunc(), True, True, self.id)
        else:
            self.uri_attr = PolymorphismAttribute(backend.attrs[cls.uri_attr],
                                                  self._get_imfunc(backend.alignments[self.pk_attr.id, cls.uri_attr]),
                                                  True, False, self.id)

        # contains both values of data property and object property.
        self.attrs = []
        self.pred2attrs: Dict[str, List[int]] = defaultdict(lambda: [])
        for lst in cls.predicates.values():
            for p in lst:
                self.pred2attrs[p.label].append(len(self.attrs))
                if isinstance(p, DataProp):
                    if self.pk_attr.id == p.data_id:
                        imfunc = IdentityFunc()
                    else:
                        imfunc = self._get_imfunc(backend.alignments[self.pk_attr.id, p.data_id])

                    self.attrs.append(PolymorphismAttribute(
                        backend.attrs[p.data_id],
                        imfunc, True
                    ))
                else:
                    p: ObjectProp
                    if p.object.is_blank():
                        o_attr = p.object.pk_attr
                    else:
                        o_attr = p.object.uri_attr

                    imfunc = self._get_imfunc(backend.alignments[self.pk_attr.id, o_attr])
                    self.attrs.append(PolymorphismAttribute(
                        backend.attrs[o_attr], imfunc,
                        imfunc.is_x2o, p.object.is_blank(), p.object.node_id,
                    ))

    def iter_records(self):
        """
        Iterate through all records of this class
        """
        for index in np.ndindex(*self.pk_attr.shape):
            yield ArrayRecord(index, self)

    def get_record_by_id(self, rid: RecordID):
        """
        Get a record of the class by id
        """
        return ArrayRecord(rid.index, self)

    def get_data_prop_as_ndarray(self, edge_id: int, index_edge_ids: List[int]) -> 'PropDataNDArray':
        """
        Get predicate data (identified by `pred_id` as a high-dimensional array). The original data may already be in high-dimension
        array or may be not, but the returned value must be a high-dimensional array.

        The supplied `index_edges` are list of edges that will occupied first dimensions. If an edge in index_edges are
        high-dimensional array as well, then its value will be flatten.

        There must be an alignment between the edge_id and other index edges (the alignment represent the join). The alignment
        must be dimension alignment for now (then we don't need to do a join but only swapping and arranging dimension). In case the
        alignment are chained, then we have to join, and create new table?
        """
        edge = self.cls.sm.edges[edge_id]
        col_id = edge.target_id
        attr = self.backend.attrs[edge.target_id]

        # these index columns may also be in high-dimensional array
        index_attr_ids = [self.cls.sm.edges[eid].target_id for eid in index_edge_ids]
        index_attrs = [self.backend.attrs[cid] for cid in index_attr_ids]

        """
        1. The algorithm works by first retrieve original nd-array of the column called C.
        2. We get alignments between the column and other index columns; and make sure that the alignments are all 
        dimension alignments. If there are some alignments that are not dimension alignments, we fall back to doing join
        between multiple tables, this would be slow!.
        3. For each index column: ICi:
            1. We retrieve the aligned dimensions between the column C and the index column ICi, now if there is any 
            dimension in ICi that is not aligned (*-to-many), we have to replicate the value C to match that. 
            2. Then, we mark which dimension of C is linked to dimension of ICi.
        4. After the above step, we retrieve a nested list, each dimension in C is annotated with a bunch of aligned
           dimension of ICi. We want to swap these dimensions so that the dimensions are in order of the indexed ICi.
           However, we may run into the case where there may be overlapping between dimensions of ICi. For example:
            1 1 1 1
            |-|-|-|-|
                2 2 2 
            Then, the result dimension would be:
            1 1 1
            |-|-|-|
              2 2 2

            If we cannot make dimension of index columns continuous (e.g., mixed, or one dim have 3 index_cols), then
            we have to throw error because there is no way to handle that. It cannot be continuous when at one dimension,
            the list of indexed edges are not continuous e.g, at dim 2, it is indexed by both col 1 and 3 (missing col 2)

            In that case, the users should use other functions to handle the situation yourself.
        """
        # 1st retrieve the data
        data = attr.get_data()
        data_dims = [[] for _ in range(len(data.shape))]

        alignments: List[List[RangeAlignment]] = [self.backend.alignments[col_id, icid] for icid in index_attr_ids]
        for aligns, index_col, index_col_idx in zip(alignments, index_attrs, range(len(index_attrs))):
            # source is always col_id
            assert len(aligns) == 1
            align = aligns[0]
            col_aligned_steps = {step.source_idx for step in align.aligned_steps}
            index_col_aligned_steps = {step.target_idx for step in align.aligned_steps}

            # 3.1 find any dimension in ICi that is not aligned, but also must select more than one element (*-to-many)
            index_col_unbounded_dims = [
                dim
                for step_idx, dim in enumerate(index_col.step2dim)
                if step_idx not in index_col_aligned_steps and dim is not None
            ]
            if len(index_col_unbounded_dims) > 0:
                # calculate the number of elements k we need to repeated
                k = np.prod([index_col.shape[i] for i in index_col_unbounded_dims])
                data = data.repeat(k, axis=-1)
                # perform step 3.2 here
                data_dims.append([index_col_idx])

            # 3.2 mark dimension of data, saying which dimension is linked to which dimension of ICi
            for col_step_idx in col_aligned_steps:
                col_dim = attr.step2dim[col_step_idx]
                data_dims[col_dim].append(index_col_idx)

        # now is the swapping dimensions part, although numpy may just return different view,
        # we don't want to swap a lot because if we can swap freely, why numpy transpose cannot guarantee for returning
        # a view?
        # detect when we cannot swap
        for icis in data_dims:
            if len(icis) > 0 and len(icis) != (icis[-1] - icis[0] + 1):
                raise Exception("Cannot find a way to satisfied the query condition")

        # find the new order of dimensions by sorting
        new_data_dims = []
        new_axies = []
        axis = list(range(len(data_dims)))
        for i in range(len(data_dims)):
            min_j = i
            for j in range(i, len(data_dims)):
                # min(data_dims[*]) = data_dims[*][0]
                if len(data_dims[j]) > 0 and data_dims[j][0] < data_dims[i][0]:
                    min_j = j
            if min_j != i:
                # we have to swap
                data_dims[min_j], data_dims[i] = data_dims[i], data_dims[min_j]
                axis[min_j], axis[i] = axis[i], axis[min_j]
            new_data_dims.append(data_dims[i])
            new_axies.append(axis[i])

        # finally, return the range of each index columns from the new_data_dims to return
        index_attr_positions = [IndexPropRange(len(index_attrs) * 2, 0) for _ in range(len(index_attrs))]
        for i, icis in enumerate(new_data_dims):
            for j in icis:
                index_attr_positions[j].start = min(i, index_attr_positions[j].start)
                index_attr_positions[j].end = max(i + 1, index_attr_positions[j].end)
        data = np.transpose(data, new_axies)
        return PropDataNDArray(data, attr.nodata, index_attr_positions, [a.get_data() for a in index_attrs])

    def filter(self, conditions) -> 'ArrayClass':
        """
        Filter based on
        """
        for predicate, op, value in conditions:
            # depends on some operators that we don't need to filter
            if op == '=':
                matched_attrs = [self.attrs[a] for a in self.pred2attrs[predicate]]
                # the value cannot be equal.
                for id in np.ndindex(self.pk_attr.shape):
                    pass

    def group_by(self, predicate: str) -> Iterable[Tuple[Any, 'ArrayClass']]:
        """Group by predicate (can be string or predicate id)"""
        # perform group by the predicate id
        if len(self.pred2attrs[predicate]) > 1:
            raise Exception("Key of group_by operator cannot be a list")

        p = self.cls.p(predicate)[0]
        o = self.attrs[self.pred2attrs[predicate][0]]
        if o.size == 1:
            yield o.get_first_value(), self
        else:
            groups = defaultdict(set)
            for id in np.ndindex(self.pk_attr.shape):
                for val in o.get_mval(id):
                    groups[val].add(id)
            for val, ids in groups.items():
                yield val, FilteredArrayClass(self, ids)

    def _get_imfunc(self, alignments: List[Alignment]) -> Union[X2OFunc, X2MFunc]:
        # constraint of the array-backend class, we have it until we figure out how to
        # handle chained join or value join efficiency. This constraint is supposed to
        # be always satisfied since it has been checked before writing data to array-backend
        # format
        assert len(alignments) == 1 and isinstance(alignments[0], RangeAlignment)
        # TODO: fix me! bug if there is missing values
        target2source = [s.source_idx - 1 for s in sorted(alignments[0].aligned_steps, key=lambda s: s.target_idx)]
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
                if self.is_x2o_func:
                    return self.im_func(index)
                else:
                    return next(self.im_func(index))
            else:
                if self.is_x2o_func:
                    return BlankRecordID(self.attr.get_value(self.im_func(index)), self.class_id)
                return BlankRecordID(self.attr.get_value(next(self.im_func(index))), self.class_id)
        else:
            if self.is_x2o_func:
                return self.attr.get_value(self.im_func(index))
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
                return [RecordID(self.attr.get_value(idx), idx, self.class_id)]
            return [RecordID(self.attr.get_value(x), x, self.class_id) for x in self.im_func(index)]
        else:
            if self.is_x2o_func:
                return [self.attr.get_value(self.im_func(index))]
            return [self.attr.get_value(x) for x in self.im_func(index)]

    def set_value(self, index: Tuple[int, ...], val: Any):
        if self.is_blank:
            raise Exception("Cannot assign value to blank instances")
        if self.is_x2o_func:
            self.attr.set_value(self.im_func(index), val)
        else:
            # expect value to be an iterator
            for i, x in enumerate(self.im_func(index)):
                self.attr.set_value(x, next(val))
