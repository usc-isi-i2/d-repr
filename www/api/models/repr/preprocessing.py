from typing import List, Dict, Tuple, Callable, Any, Optional, Union

from api.misc.dict2instance import Dict2InstanceDeSer, get_str_deser, get_object_enum_deser
from api.models.repr.location import Location


class PMap(Dict2InstanceDeSer):
    class_properties = {
        "input": Location,
        "output": get_str_deser("output", nullable=True),
        "code": get_str_deser("code")
    }

    def __init__(self, input: Location, output: Optional[str], code: str):
        self.input = input
        self.output = output
        self.code = code


class PFilter(Dict2InstanceDeSer):
    class_properties = {
        "input": Location,
        "output": get_str_deser("output", nullable=True),
        "code": get_str_deser("code")
    }

    def __init__(self, input: Location, output: Optional[str], code: str):
        self.input = input
        self.output = output
        self.code = code


PreprocessingDeSer = get_object_enum_deser({"pmap": PMap, "pfilter": PFilter})
PreprocessingFunc = Union[PMap, PFilter]
