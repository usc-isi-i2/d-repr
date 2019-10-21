from collections import Counter
from pathlib import Path

import numpy as np
from PIL import Image
from PIL.TiffTags import TAGS

from drepr.models import DRepr, ResourceType, yaml, ClassNode, DataNode, RangeExpr, LiteralNode
from drepr.ndarray import NDArray


def cf_convention_ndarray_map(ds_model: DRepr, resource_file: str):
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








if __name__ == '__main__':
    resource_file = "/Users/rook/workspace/MINT/MINT-Transformation/examples/scotts_transformations/soil/CLYPPT_M_sl1_1km.tiff"
    ds_model = """
version: '1'
resources: geotiff
attributes: {}
alignments: {}
    """
    map_ndarray_cf_convention(DRepr.parse(yaml.load(ds_model)), resource_file)
