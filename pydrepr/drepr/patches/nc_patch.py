import copy
from typing import Dict

import ujson
from netCDF4 import Dataset

from drepr.models import ResourceType, DRepr
from drepr.patches import ResourceData, ResourceDataFile, ResourceDataString


def patch(repr: DRepr, resources: Dict[str, ResourceData]) -> DRepr:
    """
    This patch will turn any netcdf resources to JSON
    """
    need_patch = False
    netcdf_types = {ResourceType.NetCDF4, ResourceType.NetCDF3}
    for resource in repr.resources:
        if resource.type in netcdf_types:
            need_patch = True

    if need_patch:
        repr = copy.deepcopy(repr)
        for resource in repr.resources:
            if resource.type in netcdf_types:
                resource.type = ResourceType.JSON

                assert isinstance(
                    resources[resource.id],
                    ResourceDataFile), "Doesn't support loading netcdf from raw bytes yet"
                nc_data = Dataset(resources[resource.id].file_path,
                                  "r+",
                                  format=ResourceType.NetCDF4.value.upper())

                doc = {}
                for vname in nc_data.variables.keys():
                    doc[vname] = nc_data.variables[vname][:].tolist()

                resources[resource.id] = ResourceDataString(ujson.dumps(doc))
                # resources[resource.id] = ResourceDataFile(f"/tmp/{str(uuid.uuid4())}.json")
                # resources[resource.id] = ResourceDataFile("/tmp/bbfad741-f13f-4227-9a3c-9ab5fbe54bc4.json")
                # with open(resources[resource.id].file_path, "w") as f:
                #     ujson.dump(doc, f)

    return repr
