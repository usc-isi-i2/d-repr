import abc
from typing import *

from drepr.models import Variable, Location
from drepr.services.ra_iterator import RAIterator


class RAReader(metaclass=abc.ABCMeta):

    def __init__(self):
        self.grounded_locations: Dict[Location, Location] = {}

    @abc.abstractmethod
    def get_value(self, index: List[Union[str, int]], start_idx: int = 0) -> Any:
        raise NotImplementedError()

    @abc.abstractmethod
    def replace_value(self, index: List[Union[str, int]], value: Any, start_idx: int = 0):
        """Replace value of an index"""
        raise NotImplementedError()

    @abc.abstractmethod
    def insert_value(self, index: List[Union[str, int]], value: Any, start_idx: int = 0):
        raise NotImplementedError()

    @abc.abstractmethod
    def remove_value(self, index: List[Union[str, int]], start_idx: int = 0):
        raise NotImplementedError()

    @abc.abstractmethod
    def dump2json(self) -> Union[dict, list]:
        raise NotImplementedError()

    @abc.abstractmethod
    def ground_location_mut(self, loc: Location, start_idx: int = 0) -> None:
        """Ground a location"""
        raise NotImplementedError()

    def has_grounded_location(self, loc: Location) -> bool:
        return loc in self.grounded_locations

    def set_grounded_location(self, origin_loc: Location, grounded_loc: Location):
        self.grounded_locations[origin_loc] = grounded_loc

    def get_grounded_location(self, loc: Location) -> Location:
        return self.grounded_locations[loc]

    def get_variable_size(self, var: Variable) -> int:
        n_elems = 1
        loc = self.grounded_locations[var.location]
        for i, s in enumerate(loc.slices):
            if s.is_range():
                n_elems *= (s.end - s.start) / s.step
        return n_elems

    def iter_data(self, grounded_loc: Location, reverse: bool = False) -> RAIterator:
        upperbound = []
        lowerbound = []
        step_sizes = []
        is_dynamic = False

        for i, s in enumerate(grounded_loc.slices):
            if s.is_range():
                lowerbound.append(s.start)
                upperbound.append(s.end)
                step_sizes.append(s.step)

                if s.end is None:
                    is_dynamic = True
            else:
                lowerbound.append(s.idx)
                upperbound.append(s.idx)
                step_sizes.append(0)

        return RAIterator(self, upperbound, lowerbound, step_sizes, reverse, is_dynamic)
