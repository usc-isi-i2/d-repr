from typing import Dict, Union, Iterable, Any, List, Tuple, Optional
import numpy as np
from drepr import DRepr
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


class NDArrayTables:
    def __init__(self, tables: Dict[str, Dict[str, NDArrayColumn]], table_shps: Dict[str, List[int]], table_relations: Dict[str, Dict[str, Any]]):
        self.tables = tables
        self.table_shps = table_shps
        self.table_relations = table_relations

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

    def get_column_as_ndarray(self, table_id: str, column_id: str, index_columns: List[str]) -> Tuple[np.ndarray, Optional[List[str]]]:
        """
        Get column data as multi-dimensional array based on `index_columns`. Each dimension `i` of the returned array
        is correponsdence to `index_columns[i]`. However, if there are multiple columns in the table that have same
        `column_id`, then the return array has another extra dimension at the end, which size is equal to the number
        of duplicated columns of `column_id`.

        :param table_id:
        :param column_id:
        :param index_columns:
        :return:
        """
        table = self.tables[table_id]
        n_records = np.prod(self.table_shps[table_id])

        column = table[column_id]
        if isinstance(column, ColArray):
            # the index column may be contain columns of high-dimensional array, so we have to unroll it to get column of
            # one dimension

            # check if the target indexed columns occupy these first dimensions of the column
            index_col_ids = set(index_columns)
            col_size = column.get_original_size()
            index_col_shp = [table[cid].get_original_size() for cid in index_columns]
            if all(idx in index_col_ids for idx in column.index_by[:len(index_col_ids)]) and len(column.index_by) >= len(index_col_ids):
                # yes, they occupy first dimensions of the column, great, we don't do anything but replicate the last dimension
                # if necessary
                if n_records == col_size and len(index_col_ids) == len(column.shape):
                    # great, they slide through all values of the column, and the size of the column match with the size of table
                    # we do nothing
                    return column.values, column.index_by

                if n_records > col_size:
                    # replicate last dimension
                    n_last_dim = n_records / col_size

                new_shp = column.shape
                new_shp[len(index_col_ids):] = [np.prod(new_shp[len(index_col_ids):])]

                if n_records == col_size:
                    # we don't need to replicate to make size of the column match with size of the table
                    return column.values.reshape(new_shp), column.index_by
                else:
                    # we need replicate
                    return column.values.reshape(new_shp).repeat(n_records / col_size / new_shp[-1], axis=-1), column.index_by
            else:
                # oh no, some of the columns may be indexed in the column, but some are not
                # in order to make these index column be the start, we need to swap axes
                # first, group consecutive index column together, then we group non-index column together
                # then, swap index with non-index one by one
                raise NotImplementedError()
        else:
            # isinstance(column, ColSingle) = True, great we don't need to do much, as we just need to replicate values
            # first broadcast to the shape of the table
            new_array = np.tile(column.value, self.table_shps[table_id])
            # now reshape to the shape of index columns
            target_shp = [table[cid].get_original_size() for cid in index_columns]
            # last dimension depends on the numner of un-index values
            target_shp.append(n_records / np.prod(target_shp))

            return new_array.reshape(target_shp), None

    def get_column_as_array(self, table_id: str, column_id: str):
        """
        Get column data as array, if there is multiple columns of same id, the returned array
        is a 2D array
        :return:
        """
        col = self.tables[table_id][column_id]
        if isinstance(col, ColArray):
            return col.values.reshape(-1).repeat(np.prod(self.table_shps[table_id]) / col.get_original_size(), axis=-1)

        return np.asarray([col.value]).repeat(np.prod(self.table_shps[table_id]))

    def get_column_unique_values(self, table_id: str, column_id: str):
        """
        :return:
        """
        return self.tables[table_id][column_id]

