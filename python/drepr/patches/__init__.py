from typing import NamedTuple, Union

ResourceDataFile = NamedTuple("ResourceDataType", [("file_path", str)])
ResourceDataString = NamedTuple("ResourceDataString", [("value", bytes)])
ResourceData = Union[ResourceDataString, ResourceDataFile]

from .nc_patch import *
from .jp_propname_patch import *
from .xml_patch import *
