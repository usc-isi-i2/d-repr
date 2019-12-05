from typing import Dict, Union, Iterable, Any, List, Tuple, Optional
import numpy as np
from drepr.models import DRepr, SemanticModel
from drepr.graph import Node, Edge
from drepr.models.align import RangeAlignment
from drepr.models.preprocessing import PreprocessingType, PMap
from drepr.models.resource import ResourceType
import drepr.executors.cf_convention_map.cf_convention_map as cf_convention_map


class ColArray:
    def __init__(self, values, index_by, shape):
        self.values = values
        self.index_by = index_by
        self.shape = shape

    def get_original_size(self) -> int:
        return np.prod(self.shape)


class ColSingle:
    def __init__(self, value):
        self.value = value

    def get_original_size(self) -> int:
        return 1


NDArrayColumn = Union[ColArray, ColSingle]


class NDArrayGraph:
    def __init__(self, sm: SemanticModel, tables: Dict[str, Dict[str, NDArrayColumn]], table_shps: Dict[str, List[int]]):
        self.sm = sm
        self.tables = tables
        self.table_shps = table_shps

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

    def edge_data_as_ndarray(self, edge_id: int, index_edges: List[int]) -> Tuple[np.ndarray, Optional[List[str]]]:
        """
        Get column data as multi-dimensional array based on `index_columns`. Each dimension `i` of the returned array
        is correponsdence to `index_columns[i]`. However, if there are multiple columns in the table that have same
        `column_id`, then the return array has another extra dimension at the end, which size is equal to the number
        of duplicated columns of `column_id`.

        What if the index predicates of a column is actually from other class that they are linked?

        We need to handle the case when index edges are outside of the table (we have to join between two tables)
        """
        edge = self.sm.edges[edge_id]
        table = self.tables[edge.source_id]
        n_records = np.prod(self.table_shps[edge.source_id])

        column = table[edge.target_id]
        # TODO: we need to handle if we have to do a join between two tables when index_edges outside of the table
        # these index columns may also be in high-dimensional array
        index_cols = [self.sm.edges[eid].target_id for eid in index_edges]
        index_col_shp = [table[cid].get_original_size() for cid in index_cols]

        if isinstance(column, ColArray):
            # the index column may be contain columns of high-dimensional array, so we have to unroll it to get column
            # of one dimension
            col_size = column.get_original_size()
            if all(idx in index_cols for idx in column.index_by[:len(index_cols)]) and len(
                    column.index_by) >= len(index_cols):
                # yes, they occupy first dimensions of the column, great, we don't do anything but replicate the last dimension
                # if necessary
                if n_records == col_size and len(index_cols) == len(column.shape):
                    # great, they slide through all values of the column, and the size of the column match with the size of table
                    # we do nothing
                    return column.values, column.index_by

                if n_records > col_size:
                    # replicate last dimension
                    n_last_dim = n_records / col_size

                new_shp = list(column.shape)
                new_shp[len(index_cols):] = [np.prod(new_shp[len(index_cols):])]

                if n_records == col_size:
                    # we don't need to replicate to make size of the column match with size of the table
                    return column.values.reshape(new_shp), column.index_by
                else:
                    # we need replicate
                    return column.values.reshape(new_shp).repeat(n_records / col_size / new_shp[-1],
                                                                 axis=-1), column.index_by
            else:
                # oh no, some of the columns may be indexed in the column, but some are not
                # in order to make these index column be the start, we need to swap axes
                # first, group consecutive index column together, then we group non-index column together
                # then, swap index with non-index one by one
                raise NotImplementedError()
        else:
            # isinstance(column, ColSingle) = True, great we don't need to do much, as we just need to replicate values
            # first broadcast to the shape of the table
            new_array = np.tile(column.value, self.table_shps[edge.source_id])
            # now reshape to the shape of index columns
            target_shp = [table[cid].get_original_size() for cid in index_cols]
            # last dimension depends on the numner of un-index values
            target_shp.append(n_records / np.prod(target_shp))

            return new_array.reshape(target_shp), None

    # def edge_data_as_array(self, class_id: str, predicate: str):
    #     """
    #     Get column data as array, if there is multiple columns of same id, the returned array
    #     is a 2D array
    #     :return:
    #     """
    #     col = self.tables[class_id][predicate]
    #     if isinstance(col, ColArray):
    #         return col.values.reshape(-1).repeat(np.prod(self.table_shps[class_id]) / col.get_original_size(), axis=-1)
    #
    #     return np.asarray([col.value]).repeat(np.prod(self.table_shps[class_id]))

    # def get_property_unique_values(self, class_id: str, predicate: str):
    #     """
    #     TODO: change me, the function name is very misleading
    #     :return:
    #     """
    #     return self.tables[class_id][predicate]

