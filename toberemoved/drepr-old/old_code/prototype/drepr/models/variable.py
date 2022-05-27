import abc
from enum import Enum
from typing import *

from drepr.misc.class_helper import Equal
from drepr.misc.dict2instance import Dict2InstanceDeSer, get_str_deser, get_bool_deser, get_object_list_deser, \
    get_set_deser, get_int_deser, get_primitive_deser, get_object_enum_deser, get_enum_deser
from drepr.exceptions import InvalidReprException


class Slice:
    @abc.abstractmethod
    def clone(self) -> 'Slice':
        raise NotImplementedError()

    @abc.abstractmethod
    def is_range(self) -> bool:
        raise NotImplementedError()

    def __repr__(self):
        if isinstance(self, DynamicRangeSlice):
            start, end, step = self.dyn_start, self.dyn_end, self.dyn_step
        elif isinstance(self, RangeSlice):
            start, end, step = self.start, self.end, self.step
        elif isinstance(self, DynamicIndexSlice):
            return f"{self.dyn_idx}"
        elif isinstance(self, IndexSlice):
            return f"{self.idx}"
        else:
            raise Exception("Congrat! You found a bug!")

        if step != 1:
            step = f":{step}"
        else:
            step = ""

        if end is None:
            return f"{start}..{step}"
        else:
            return f"{start}..{end}{step}"


class RangeSlice(Slice, Dict2InstanceDeSer, Equal):
    DeserializeErrorClass = InvalidReprException
    class_properties = {
        'start': get_int_deser('start'),
        'end': get_int_deser('end', nullable=True),
        'step': get_int_deser('step')
    }
    class_property_default_values = {'step': 1, 'end': None}

    def __init__(self, start: int, end: Optional[int] = None, step: int = 1):
        self.start = start
        self.end = end
        self.step = step

    def clone(self):
        return RangeSlice(self.start, self.end, self.step)

    def is_range(self) -> bool:
        return True


class IndexSlice(Slice, Dict2InstanceDeSer, Equal):
    DeserializeErrorClass = InvalidReprException
    class_properties = {'idx': get_primitive_deser('idx', (str, int))}

    def __init__(self, idx: Union[str, int]):
        self.idx = idx

    def clone(self):
        return IndexSlice(self.idx)

    def is_range(self) -> bool:
        return False


class DynamicRangeSlice(Slice, Dict2InstanceDeSer, Equal):
    DeserializeErrorClass = InvalidReprException
    class_properties = {
        'dyn_start': get_primitive_deser("dyn_start", (str, int)),
        'dyn_end': get_primitive_deser("dyn_end", (str, int)),
        'dyn_step': get_primitive_deser("dyn_step", (str, int)),
    }

    def __init__(self,
                 dyn_start: Optional[Union[str, int]] = None,
                 dyn_end: Optional[Union[str, int]] = None,
                 dyn_step: Optional[Union[str, int]] = 1):
        self.dyn_start = dyn_start
        self.dyn_end = dyn_end
        self.dyn_step = dyn_step

        self.start = dyn_start if not (type(dyn_start) is str
                                       and dyn_start.startswith('${')) else None
        self.end = dyn_end if not (type(dyn_end) is str and dyn_end.startswith('${')) else None
        self.step = dyn_step if not (type(dyn_step) is str and dyn_step.startswith('${')) else None

    def clone(self):
        instance = DynamicRangeSlice(self.dyn_start, self.dyn_end, self.dyn_step)
        instance.start = self.start
        instance.end = self.end
        instance.step = self.step

        return instance

    def is_range(self) -> bool:
        return True


class DynamicIndexSlice(Slice, Dict2InstanceDeSer, Equal):
    DeserializeErrorClass = InvalidReprException
    class_properties = {'dyn_idx': get_str_deser('dyn_idx')}

    def __init__(self, dyn_idx: str):
        self.dyn_idx = dyn_idx
        self.idx = None

    def clone(self):
        slice = DynamicIndexSlice(self.dyn_idx)
        slice.idx = self.idx
        return slice

    def is_range(self) -> bool:
        return False


class Location(Dict2InstanceDeSer):
    class_properties = {
        "resource_id": get_str_deser("resource_id", InvalidReprException),
        "slices": get_object_list_deser(
            "slices",
            get_object_enum_deser({
                "range_slice": RangeSlice,
                "index_slice": IndexSlice,
                "dyn_range_slice": DynamicRangeSlice,
                "dyn_index_slice": DynamicIndexSlice
            }))
    }

    def __init__(self, resource_id: str, slices: List[Slice]):
        self.resource_id = resource_id
        self.slices = slices

    def get_parent(self):
        """Like a path to directory hierarchy, access to its parent location"""
        return Location(self.resource_id, self.slices[:-1])

    def clone(self):
        return Location(self.resource_id, [s.clone() for s in self.slices])


class VariableType(Enum):

    Unspecified = "unspecified"
    Int = "int"
    Float = "float"
    String = "string"


class VariableSorted(Enum):

    Null = "null"
    Ascending = "ascending"
    Descending = "descending"


class VariableValue(Enum):

    Literal = "literal"
    List = "list"


class Variable(Equal, Dict2InstanceDeSer):
    class_readable_name = "Variable"
    class_properties = {
        "id": get_str_deser("id", InvalidReprException),
        "value": get_enum_deser("value", VariableValue, InvalidReprException),
        "location": Location,
        "sorted": get_enum_deser("sorted", VariableSorted, InvalidReprException),
        "unique": get_bool_deser("unique", InvalidReprException),
        "missing_values": get_set_deser("missing_values", InvalidReprException),
        "type": get_enum_deser("type", VariableType, InvalidReprException)
    }
    class_property_possible_values = {}
    class_property_default_values = {
        "sorted": VariableSorted.Null,
        "value": VariableValue.Literal,
        "unique": False,
        "missing_values": {"", None},
        "type": VariableType.Unspecified
    }
    DeserializeErrorClass = InvalidReprException

    def __init__(self, id: str, value: VariableValue, location: Location, sorted: VariableSorted,
                 unique: bool, missing_values: set, type: VariableType):
        self.id: str = id
        self.value = value
        self.unique = unique
        self.sorted: str = sorted
        self.location = location
        self.missing_values = missing_values
        self.type = type

    def get_n_dims(self) -> int:
        """Get number of dimensions of this variable"""
        return len(self.location.slices)

    def cmp_size(self, another: 'Variable') -> int:
        """Return 0 if two variables have same size, 1 if size of current var is greater than other and 0 otherwise"""
        default_upper_bound = 1000
        x_size = self.get_estimated_size(default_upper_bound)
        y_size = another.get_estimated_size(default_upper_bound)
        if x_size > y_size:
            return 1

        if x_size < y_size:
            return -1

        return 0

    def get_estimated_size(self, upper_bound: int) -> int:
        item = 1
        for slice in self.location.slices:
            if not isinstance(slice, (DynamicRangeSlice, RangeSlice)):
                continue

            item *= ((slice.end or upper_bound) - slice.start) // slice.step
        return item

    def parse_value(self, val: Any):
        if self.type == VariableType.Unspecified:
            return val
        elif self.type == VariableType.Int:
            return int(val)
        elif self.type == VariableType.Float:
            return float(val)
        elif self.type == VariableType.String:
            return str(val)
