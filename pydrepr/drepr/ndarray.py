from typing import Dict, Union, Iterable, Any

from drepr import DRepr
from drepr.graph import Node, Edge
from drepr.models.align import RangeAlignment
from drepr.models.preprocessing import PreprocessingType, PMap
from drepr.models.resource import ResourceType
from drepr.executors.cf_convention_map.cf_convention_map import CFConventionNDArrayMap


class NDArray:

    def __init__(self, tables: Dict[str, Dict[str, Any]], table_relations: Dict[str, Dict[str, Any]]):
        self.tables = tables
        self.table_relations = table_relations

    @staticmethod
    def from_drepr(ds_model: DRepr, resources: Union[str, Dict[str, str]]):
        # check if we can create a NDArray representation of the dataset
        # the convention that we support is CF convention
        if not CFConventionNDArrayMap.analyze(ds_model):
            raise NotImplementedError()

        if isinstance(resources, dict):
            resource = next(iter(resources.values()))
        else:
            resource = resources

        return CFConventionNDArrayMap.execute(ds_model, resource)

    def get_column_as_array(self, table_id: str, column_id: str):
        """
        Get column data as array, if there is multiple columns of same id, the returned array
        is a 2D array
        :return:
        """
        return self.tables[table_id][column_id]

    def get_column_unique_values(self, table_id: str, column_id: str):
        """
        :return:
        """
        return self.tables[table_id][column_id]

