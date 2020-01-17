from typing import List, Optional, TYPE_CHECKING, Union

import numpy as np

from drepr.models import RangeAlignment, Edge
from drepr.outputs.array_backend.array_attr import Attribute
from drepr.outputs.array_backend.lst_array_class import LstArrayClass
from drepr.outputs.prop_data_ndarray import PropDataNDArray, IndexPropRange

if TYPE_CHECKING:
    from drepr.outputs.array_backend.array_backend import ArrayBackend
from drepr.outputs.base_output_predicate import BaseOutputPredicate


class ArrayDataPredicate(BaseOutputPredicate):
    def __init__(self, backend: 'ArrayBackend', edges: List[Edge]):
        self.backend = backend
        self.edges = edges
        self.uri = edges[0].label

    def as_ndarray(self, index_predicates: List[Union['ArrayDataPredicate', 'ArrayObjectPredicate']]) -> PropDataNDArray:
        """
        Get predicate data (identified by `pred_id` as a high-dimensional array). The original data may already be in high-dimension
        array or may be not, but the returned value must be a high-dimensional array.

        The supplied `index_edges` are list of edges that will occupied first dimensions. If an edge in index_edges are
        high-dimensional array as well, then its value will be flatten.

        There must be an alignment between the edge_id and other index edges (the alignment represent the join). The alignment
        must be dimension alignment for now (then we don't need to do a join but only swapping and arranging dimension). In case the
        alignment are chained, then we have to join, and create new table?
        """
        if len(self.edges) > 1 or any(len(p.edges) > 1 for p in index_predicates):
            raise Exception("Cannot convert values of this predicate to an ndarray indexed by other predicates "
                            "because values for one entry may be greater than one")

        attr = self.attr(0)
        # these index columns may also be in high-dimensional array
        index_attrs: List[Attribute] = [p.attr(0) for p in index_predicates]

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

        alignments: List[List[RangeAlignment]] = [self.backend.alignments[attr.id, iattr.id] for iattr in
                                                  index_attrs]
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

    def o(self) -> Optional['LstArrayClass']:
        return None

    def attr(self, idx: int):
        return self.backend.attrs[self.edges[idx].target_id]


class ArrayObjectPredicate(BaseOutputPredicate):
    def __init__(self, backend: 'ArrayBackend', edges: List[Edge]):
        self.uri = edges[0].label
        self.edges = edges
        self.targets = LstArrayClass([backend.cid(e.target_id) for e in edges])

    def _init(self, backend):
        self.attrs = [
            backend.attrs[target.pk_attr_id] if target.is_blank()
            else backend.attrs[target.uri_attr_id]
            for target in self.targets
        ]

    def as_ndarray(self, indexed_predicates: List['ArrayPredicate']) -> PropDataNDArray:
        raise NotImplementedError()

    def o(self) -> Optional['LstArrayClass']:
        return self.targets

    def attr(self, idx: int):
        return self.attrs[idx]


ArrayPredicate = Union[ArrayDataPredicate, ArrayObjectPredicate]