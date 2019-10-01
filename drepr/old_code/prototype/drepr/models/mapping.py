import abc
from typing import List, Optional, TYPE_CHECKING

from drepr.misc.class_helper import Equal
from drepr.misc.dict2instance import get_str_deser, get_list_int_deser, Dict2InstanceDeSer
from drepr.exceptions import InvalidReprException

if TYPE_CHECKING:
    from drepr.models.representation import Representation


class Mapping(metaclass=abc.ABCMeta):
    @abc.abstractmethod
    def get_source_variable_id(self) -> str:
        raise NotImplementedError()

    @abc.abstractmethod
    def get_target_variable_id(self) -> str:
        raise NotImplementedError()

    @abc.abstractmethod
    def is_single_value_func(self, repr: 'Representation') -> bool:
        raise NotImplementedError()

    @abc.abstractmethod
    def is_surjective(self) -> bool:
        raise NotImplementedError()

    def is_set_value_func(self, repr: 'Representation') -> bool:
        return not self.is_single_value_func(repr)

    def swap(self) -> Optional['Mapping']:
        """Return a invert mapping function is exist"""
        return None


class IdenticalMapping(Mapping, Equal):

    def __init__(self, var: str):
        self.var = var

    def get_source_variable_id(self) -> str:
        return self.var

    def get_target_variable_id(self) -> str:
        return self.var

    def is_single_value_func(self, repr: 'Representation') -> bool:
        return True

    def is_surjective(self) -> bool:
        return True


class DimensionMapping(Mapping, Dict2InstanceDeSer, Equal):
    DeserializeErrorClass = InvalidReprException
    class_properties = {
        "source_var": get_str_deser("source_var", InvalidReprException),
        "target_var": get_str_deser("target_var", InvalidReprException),
        "source_dims": get_list_int_deser("source_dims", InvalidReprException),
        "target_dims": get_list_int_deser("target_dims", InvalidReprException)
    }

    def __init__(self, source_var: str, source_dims: List[int], target_var: str, target_dims: List[int]):
        self.source_var = source_var
        self.source_dims = source_dims
        self.target_var = target_var
        self.target_dims = target_dims

    def get_source_variable_id(self) -> str:
        return self.source_var

    def get_target_variable_id(self) -> str:
        return self.target_var

    def is_single_value_func(self, repr: 'Representation') -> bool:
        y_loc = repr.get_variable(self.target_var).location
        if any(slice.is_range() and dim not in self.target_dims for dim, slice in enumerate(y_loc.slices)):
            return False
        return True

    def is_surjective(self) -> bool:
        # TODO: implement it properly
        return True

    def swap(self) -> Optional["DimensionMapping"]:
        return DimensionMapping(self.target_var, self.target_dims, self.source_var, self.source_dims)


# class ValueMapping(Mapping, EnumDeSer, Equal):
#     pass