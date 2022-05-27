import abc
import copy
from typing import List, Any, Generator

from drepr.models.variable import Location
from drepr.models.mapping import Mapping, DimensionMapping
from drepr.services.index_iterator import IndexIterator, DynamicIndexIterator
from drepr.services.ra_reader.ra_reader import RAReader


class MappingFunc(metaclass=abc.ABCMeta):

    @abc.abstractmethod
    def is_single_value_func(self) -> bool:
        """telling if this function is map a single data item to a single data items"""
        raise NotImplementedError()

    def is_set_value_func(self) -> bool:
        """telling if this function is map a single data item to a set of data items"""
        return not self.is_single_value_func()

    @abc.abstractmethod
    def is_surjective(self) -> bool:
        raise NotImplementedError()

    @abc.abstractmethod
    def single_val_exec(self, val: Any, index: List[int]) -> List[int]:
        raise NotImplementedError()

    @abc.abstractmethod
    def multiple_val_exec(self, val: Any, index: List[int]) -> Generator[List[int], None, None]:
        raise NotImplementedError()


class IdenticalMappingFunc(MappingFunc):

    def is_single_value_func(self) -> bool:
        return True

    def is_surjective(self) -> bool:
        return True

    def single_val_exec(self, val: Any, index: List[int]) -> List[int]:
        return index

    def multiple_val_exec(self, val: Any, index: List[int]) -> Generator[List[int], None, None]:
        pass


class IndexMappingFunc(MappingFunc):

    def __init__(self, ra_reader: RAReader, mapping: Mapping, x_loc: Location, y_loc: Location):
        self.ra_reader = ra_reader
        self.x_loc = x_loc
        self.y_loc = y_loc

        assert isinstance(mapping, DimensionMapping)
        # step 0: init new index with null list
        y0_idx: List[int] = [None] * len(y_loc.slices)
        # step 1: update fixed dimensions
        for i, slice in enumerate(y_loc.slices):
            if not slice.is_range():
                y0_idx[i] = slice.idx

        self.unmapped_dims = [i for i in range(len(y0_idx)) if y0_idx[i] is None and i not in mapping.target_dims]
        self.lower_bound_index = [y_loc.slices[i].start for i in self.unmapped_dims]
        self.upper_bound_index = [y_loc.slices[i].end for i in self.unmapped_dims]

        self.y0_idx = y0_idx
        self.is_single_val_func = len(self.unmapped_dims) == 0
        self.mapped_dims = [(xdim, ydim) for xdim, ydim in zip(mapping.source_dims, mapping.target_dims)]

    def is_single_value_func(self) -> bool:
        return self.is_single_val_func

    def is_surjective(self) -> bool:
        return True

    def single_val_exec(self, val: Any, index: List[int]) -> List[int]:
        y_idx = copy.copy(self.y0_idx)
        for xdim, ydim in self.mapped_dims:
            y_idx[ydim] = int((index[xdim] - self.x_loc.slices[xdim].start) / self.x_loc.slices[xdim].step * \
                           self.y_loc.slices[ydim].step) + self.y_loc.slices[ydim].start

        return y_idx

    def multiple_val_exec(self, val: Any, index: List[int]) -> Generator[List[int], None, None]:
        if all(x is not None for x in self.upper_bound_index):
            y_idx = copy.copy(self.y0_idx)
            for xdim, ydim in self.mapped_dims:
                y_idx[ydim] = int((index[xdim] - self.x_loc.slices[xdim].start) / self.x_loc.slices[xdim].step * \
                                  self.y_loc.slices[ydim].step) + self.y_loc.slices[ydim].start

            idx_iterator = IndexIterator(self.upper_bound_index, self.lower_bound_index)

            for idx in idx_iterator:
                for i, v in zip(self.unmapped_dims, idx):
                    y_idx[i] = v
                yield y_idx
        else:
            lowerbound = copy.copy(self.y0_idx)
            upperbound = copy.copy(self.y0_idx)
            steps = [1] * len(self.y0_idx)

            for xdim, ydim in self.mapped_dims:
                lowerbound[ydim] = int((index[xdim] - self.x_loc.slices[xdim].start) / self.x_loc.slices[xdim].step * \
                                  self.y_loc.slices[ydim].step) + self.y_loc.slices[ydim].start
                upperbound[ydim] = lowerbound[ydim]
                steps[ydim] = 0

            for ydim in self.unmapped_dims:
                lowerbound[ydim] = self.y_loc.slices[ydim].start

            idx_iterator = DynamicIndexIterator(self.ra_reader, upperbound, lowerbound, steps)
            for idx in idx_iterator:
                yield idx

