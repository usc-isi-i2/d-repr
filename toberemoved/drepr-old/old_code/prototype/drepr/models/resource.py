from enum import Enum

from drepr.misc.class_helper import Equal
from drepr.misc.dict2instance import Dict2InstanceDeSer, get_str_deser, get_object_enum_deser
from drepr.exceptions import InvalidReprException


class ResourceType(Enum):
    csv = "csv"
    json = "json"
    netcdf4 = "netcdf4"


class Resource:
    DeserializeErrorClass = InvalidReprException
    class_properties = {"id": get_str_deser("id", InvalidReprException)}

    def __init__(self, id: str):
        self.id = id


class CSVResource(Resource, Dict2InstanceDeSer, Equal):
    DeserializeErrorClass = InvalidReprException
    class_properties = {
        "id": get_str_deser("id", InvalidReprException),
        "delimiter": get_str_deser("delimiter", InvalidReprException)
    }
    class_property_default_values = {"delimiter": ","}

    def __init__(self, id: str, delimiter: str):
        super().__init__(id)
        self.delimiter = delimiter


class JSONResource(Resource, Dict2InstanceDeSer, Equal):
    pass


class NetCDF4Resource(Resource, Dict2InstanceDeSer, Equal):
    pass


ResourceDeSer = get_object_enum_deser({
    "csv": CSVResource,
    "json": JSONResource,
    "netcdf4": NetCDF4Resource
})
