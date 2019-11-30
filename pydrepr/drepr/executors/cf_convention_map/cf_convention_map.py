from drepr.executors.cf_convention_map.geotiff_map import map_geotiff
from drepr.executors.cf_convention_map.netcdf_map import map_netcdf
from drepr.models import DRepr, ResourceType, RangeAlignment, PMap


class CFConventionNDArrayMap:

    @classmethod
    def analyze(cls, ds_model: DRepr) -> bool:
        # only one resource
        if len(ds_model.resources) > 1:
            return False

        # ensure it is geotiff, netcdf4
        if ds_model.resources[0].type not in {ResourceType.GeoTIFF, ResourceType.NetCDF3, ResourceType.NetCDF4}:
            return False

        # only have dimension alignment
        if not all(isinstance(align, RangeAlignment) for align in ds_model.aligns):
            return False

        # only have map preprocessing, which mutate the current data
        for prepro in ds_model.preprocessing:
            if not isinstance(prepro.value, PMap) \
                    or prepro.value.output is not None \
                    or prepro.value.change_structure:
                return False

        return True

    @classmethod
    def execute(cls, ds_model: DRepr, resource_file: str):
        """
        Map high-dimensional single resource datasets to NDArray format
        """
        if ds_model.resources[0].type == ResourceType.GeoTIFF:
            return map_geotiff(ds_model, resource_file)
        elif ds_model.resources[0].type == ResourceType.NetCDF4:
            return map_netcdf(ds_model, resource_file, "4")
        elif ds_model.resources[0].type == ResourceType.NetCDF3:
            return map_netcdf(ds_model, resource_file, "3")
        else:
            raise ValueError("Invalid resource type %s" % ds_model.resources[0].type)
