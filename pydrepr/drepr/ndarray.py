from typing import Dict, Union, Iterable

from drepr import DRepr
from drepr.graph import Node, Edge
from drepr.models.align import RangeAlignment
from drepr.models.preprocessing import PreprocessingType, PMap
from drepr.models.resource import ResourceType


class NDArray:

    def __init__(self, ds_model: DRepr, resource: str):
        self.ds_model = ds_model
        self.resource = resource

    def iter_nodes(self) -> Iterable[Node]:
        pass

    def iter_nodes_by_class(self, cls: str) -> Iterable[Node]:
        pass

    def iter_edges(self) -> Iterable[Edge]:
        pass

    @staticmethod
    def from_drepr(ds_model: DRepr, resources: Union[str, Dict[str, str]]):
        # check if we can create a NDArray representation of the dataset
        # the convention that we support is CF convention
        if not NDArray.is_cf_convention(ds_model):
            return False

        if isinstance(resources, dict):
            resource = next(iter(resources.values()))
        else:
            resource = resources

        return NDArray(ds_model, resource)

    @staticmethod
    def is_cf_convention(ds_model: DRepr) -> bool:
        # have only one resource, which is netcdf
        if len(ds_model.resources) > 1 and ds_model.resources[0].type != ResourceType.NetCDF:
            return False

        # only have map preprocessing, which mutate the current data
        for prepro in ds_model.preprocessing:
            if not isinstance(prepro.value, PMap) \
                    or prepro.value.output is not None \
                    or prepro.value.change_structure:
                return False

        # all alignments are dimension alignments
        for align in ds_model.aligns:
            if not isinstance(align, RangeAlignment):
                return False
