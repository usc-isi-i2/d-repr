from abc import ABC, abstractmethod
from dataclasses import dataclass
from enum import Enum
from typing import Union, Optional


class ResourceType(Enum):
    CSV = "csv"
    JSON = "json"
    XML = "xml"
    Spreadsheet = "spreadsheet"
    NetCDF4 = "netcdf4"
    NetCDF3 = "netcdf3"
    GeoTIFF = "geotiff"
    NPDict = "np-dict"
    Shapefile = "shapefile"
    Container = "container"


@dataclass
class CSVProp:
    delimiter: str = ","


@dataclass
class Resource:
    id: str
    type: ResourceType
    prop: Optional[CSVProp] = None

    @staticmethod
    def deserialize(raw: dict):
        if raw['type'] == ResourceType.CSV.value and raw['prop'] is not None:
            prop = CSVProp(raw['prop']['delimiter'])
        else:
            prop = None
        return Resource(raw['id'], ResourceType(raw['type']), prop)


class ResourceData(ABC):
    
    @abstractmethod
    def to_dict(self):
        pass
    

@dataclass
class ResourceDataFile(ResourceData):
    file: str

    def to_dict(self):
        return {"file": self.file}

@dataclass
class ResourceDataString(ResourceData):
    value: Union[str, bytes]

    def to_dict(self):
        return {"string":self.value.decode() if isinstance(self.value, bytes) else self.value}