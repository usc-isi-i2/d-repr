from dataclasses import dataclass

import numpy as np
from typing import Dict, Union, Iterable, List, Optional

import drepr.executors.cf_convention_map.cf_convention_map as cf_convention_map
from drepr.models import DRepr, SemanticModel, Alignment
from drepr.models.align import RangeAlignment
from drepr.ndarray.column import NoData, NDArrayColumn, ColSingle


@dataclass
class IndexEdgeRange:
    start: int
    end: int


@dataclass
class NodeProxy:
    pass


@dataclass
class EdgeDataNDArray:
    data: np.ndarray
    nodata: Optional[NoData]
    index_edges: List[IndexEdgeRange]


class NDArrayGraph:
    def __init__(self, sm: SemanticModel, tables: Dict[str, Dict[str, NDArrayColumn]],
                 alignments: Dict[str, Dict[str, Alignment]]):
        self.sm = sm
        self.tables = tables
        self.alignments = alignments

    @staticmethod
    def from_drepr(ds_model: DRepr, resources: Union[str, Dict[str, str]]):
        # check if we can create a NDArray representation of the dataset
        # the convention that we support is CF convention
        if not cf_convention_map.CFConventionNDArrayMap.analyze(ds_model):
            raise NotImplementedError()

        if isinstance(resources, dict):
            resource = next(iter(resources.values()))
        else:
            resource = resources

        return cf_convention_map.CFConventionNDArrayMap.execute(ds_model, resource)

    def iter_class_ids(self, cls: str):
        for node in self.sm.iter_class_nodes():
            if node.label == cls:
                yield node.node_id

    def edge_data_as_ndarray(self, edge_id: int, index_edges: List[int]) -> EdgeDataNDArray:
        """
        Get edge data (identified by `edge_id` as a high-dimensional array). The original data may already be in high-dimension
        array or may be not, but the returned value must be a high-dimensional array.

        The supplied `index_edges` are list of edges that will occupied first dimensions. If an edge in index_edges are
        high-dimensional array as well, then its value will be flatten. Each

        There must be an alignment between the edge_id and other index edges (the alignment represent the join). The alignment
        must be dimension alignment for now (then we don't need to do a join but only swapping and arranging dimension). In case the
        alignment are chained, then we have to join, and create new table?

        """
        edge = self.sm.edges[edge_id]
        table_id = edge.source_id
        col_id = edge.target_id
        table = self.tables[table_id]
        column = table[edge.target_id]

        # these index columns may also be in high-dimensional array
        index_col_ids = [self.sm.edges[eid].target_id for eid in index_edges]
        index_cols = [self.tables[self.sm.edges[eid].source_id][cid] for eid, cid in zip(index_edges, index_col_ids)]

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
        """
        # 1st retrieve the data
        data = column.get_data()
        data_dims = [[] for _ in range(len(data.shape))]

        alignments: List[RangeAlignment] = [self.alignments[col_id][icid] for icid in index_col_ids]
        for align, index_col, index_col_idx in zip(alignments, index_cols, range(len(index_cols))):
            # if align.source == index_col.id:
            #     index_col_aligned_steps = {step.source_idx for step in align.aligned_steps}
            #     col_aligned_steps = {step.target_idx for step in align.aligned_steps}
            # else:

            # source is always col_id
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
                col_dim = column.step2dim[col_step_idx]
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
        index_col_positions = [IndexEdgeRange(len(index_cols) * 2, 0) for _ in range(len(index_cols))]
        for i, icis in enumerate(new_data_dims):
            for j in icis:
                index_col_positions[j].start = min(i, index_col_positions[j].start)
                index_col_positions[j].end = max(i + 1, index_col_positions[j].end)
        data = np.transpose(data, new_axies)
        return EdgeDataNDArray(data, column.nodata, index_col_positions)

    def iter_nodes_by_class(self, cls: str) -> Iterable[NodeProxy]:
        """
        Iterate through each node in the table, and edit the value directly there.
        """
        raise NotImplementedError()

    def _deprecated_get1rowtbl(self, cls: str) -> dict:
        class_id = next(self.iter_class_ids(cls))
        table = self.tables[class_id]
        record = {}
        for k, v in table.items():
            assert isinstance(v, ColSingle), v
            pred = next(self.sm.iter_incoming_edges(k)).label
            if pred not in record:
                record[pred] = v.value
            else:
                if isinstance(record[pred], list):
                    record[pred].append(v.value)
                else:
                    record[pred] = [v.value]
        return record
