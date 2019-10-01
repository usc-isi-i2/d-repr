from enum import Enum
from typing import Optional, TYPE_CHECKING

from drepr.misc.dict2instance import Dict2InstanceDeSer, get_str_deser, get_enum_deser
from drepr.models import InvalidReprException
from drepr.models.variable import Location

if TYPE_CHECKING:
    from drepr.models.representation import Representation


class FuncType(Enum):

    Map = "map"
    Filter = "filter"
    Split = "split"
    Flatten = "flatten"


class TransformFunc(Dict2InstanceDeSer):
    DeserializeErrorClass = InvalidReprException
    class_properties = {
        "type": get_enum_deser("type", FuncType, InvalidReprException),
        "input_var": get_str_deser("input_var"),
        "location": Location,
        "function": get_str_deser("function"),
        "output_resource": get_str_deser("output_resource")
    }
    class_property_default_values = {'input_var': None, 'location': None, 'output_resource': None, 'function': ''}

    def __init__(self, type: str, input_var: Optional[str], location: Optional[Location],
                 function: str, output_resource: Optional[str]):
        self.type = type
        self.input_var = input_var
        self.location = location
        self.function = function
        self.output_resource = output_resource

    def get_location(self, repr: 'Representation'):
        if self.location is not None:
            return self.location

        return repr.get_variable(self.input_var).location
