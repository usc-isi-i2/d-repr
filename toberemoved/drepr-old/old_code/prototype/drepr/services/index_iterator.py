import copy
from itertools import islice
from typing import List, Optional, TYPE_CHECKING

if TYPE_CHECKING:
    from drepr.services.ra_reader.ra_reader import RAReader


class IndexIterator:
    def __init__(self,
                 upper_bound_index: List[int],
                 lower_bound_index: List[int],
                 step_sizes: List[int] = None):
        n_dims = len(upper_bound_index)
        self.step_sizes = [1] * n_dims if step_sizes is None else step_sizes
        self.lower_bound_index = lower_bound_index
        self.upper_bound_index = upper_bound_index

        # store the unfrozen dimensions in reversed order
        self.no_more_val = False
        self.unfrozen_dimensions = []
        for i, l, u, s in zip(range(n_dims), lower_bound_index, upper_bound_index, self.step_sizes):
            if (s == 0 and u != l) or (s > 0 and u <= l):
                self.no_more_val = True

            if s > 0:
                # this dimension is not frozen
                self.unfrozen_dimensions.append(i)

        self.unfrozen_dimensions = list(reversed(self.unfrozen_dimensions))
        self.curr_index: List[int] = copy.copy(lower_bound_index)

        if len(self.unfrozen_dimensions) > 0:
            # a trick to generate correct next value at the beginning
            self.curr_index[self.unfrozen_dimensions[0]] -= self.step_sizes[self.
                                                                            unfrozen_dimensions[0]]

    def __iter__(self):
        return self

    def __next__(self):
        res = self.next()
        if res is None:
            raise StopIteration()
        return res

    def next(self) -> Optional[List[int]]:
        """Get the next index by move current index one step and return it"""
        if self.no_more_val:
            return None

        for dim_pivot in self.unfrozen_dimensions:
            self.curr_index[dim_pivot] += self.step_sizes[dim_pivot]
            if self.curr_index[dim_pivot] >= self.upper_bound_index[dim_pivot]:
                self.curr_index[dim_pivot] = self.lower_bound_index[dim_pivot]
            else:
                return self.curr_index
        else:
            self.no_more_val = True
            if len(self.unfrozen_dimensions) == 0:
                return self.curr_index
            else:
                return None


class ReversedIndexIterator:
    def __init__(self,
                 upper_bound_index: List[int],
                 lower_bound_index: List[int],
                 step_sizes: List[int] = None):
        n_dims = len(upper_bound_index)
        self.step_sizes = [1] * n_dims if step_sizes is None else step_sizes
        self.lower_bound_index = lower_bound_index
        self.upper_bound_index = upper_bound_index

        # store the unfrozen dimensions in reversed order
        self.no_more_val = False
        self.unfrozen_dimensions = []
        self.curr_index: List[int] = []
        for i, l, u, s in zip(range(n_dims), lower_bound_index, upper_bound_index, self.step_sizes):
            if (s == 0 and u != l) or (s > 0 and u <= l):
                self.no_more_val = True

            if s > 0:
                # this dimension is not frozen
                self.unfrozen_dimensions.append(i)
                self.curr_index.append(u - s)
            else:
                self.curr_index.append(u)

        self.unfrozen_dimensions = list(reversed(self.unfrozen_dimensions))
        if len(self.unfrozen_dimensions) > 0:
            # a trick to generate correct next value at the beginning
            self.curr_index[self.unfrozen_dimensions[0]] += self.step_sizes[self.
                                                                            unfrozen_dimensions[0]]

    def __iter__(self):
        return self

    def __next__(self):
        res = self.next()
        if res is None:
            raise StopIteration()
        return res

    def next(self) -> Optional[List[int]]:
        """Get the next index by move current index back one step and return it"""
        if self.no_more_val:
            return None

        for dim_pivot in self.unfrozen_dimensions:
            self.curr_index[dim_pivot] -= self.step_sizes[dim_pivot]
            if self.curr_index[dim_pivot] < self.lower_bound_index[dim_pivot]:
                self.curr_index[
                    dim_pivot] = self.upper_bound_index[dim_pivot] - self.step_sizes[dim_pivot]
            else:
                return self.curr_index
        else:
            self.no_more_val = True
            if len(self.unfrozen_dimensions) == 0:
                return self.curr_index
            else:
                return None


class DynamicIndexIterator:
    def __init__(self,
                 ra_reader: 'RAReader',
                 upper_bound_index: List[Optional[int]],
                 lower_bound_index: List[int],
                 step_sizes: List[int] = None):
        self.n_dims = len(upper_bound_index)

        self.ra_reader = ra_reader
        self.step_sizes = [1] * self.n_dims if step_sizes is None else step_sizes
        self.lower_bound_index = lower_bound_index
        self.upper_bound_index = upper_bound_index

        # store the unfrozen dimensions in reversed order
        self.no_more_val = False
        self.unfrozen_dimensions = []
        for i, l, u, s in zip(
                range(self.n_dims), lower_bound_index, upper_bound_index, self.step_sizes):
            if (s == 0 and u != l) or (s > 0 and u is not None and u <= l):
                self.no_more_val = True

            if s > 0:
                # this dimension is not frozen
                self.unfrozen_dimensions.append(i)

        self.unfrozen_dimensions = list(reversed(self.unfrozen_dimensions))
        self.curr_index: List[int] = copy.copy(lower_bound_index)

        # init local upper bound index
        self.local_up_idx: List[int] = copy.copy(self.upper_bound_index)
        self.cached_data_ptr = [None] * self.n_dims
        self.update_local_upperbound_index()

        if len(self.unfrozen_dimensions) > 0:
            # a trick to generate correct next value at the beginning
            self.curr_index[self.unfrozen_dimensions[0]] -= self.step_sizes[self.
                                                                            unfrozen_dimensions[0]]

    def update_local_upperbound_index(self, start_idx: int = 0):
        """Recalculate correct upper bound of the current index (in one dimension) always start from second dimension"""
        if start_idx == 0:
            # we always know the upperbound of the first (0) dimension
            self.cached_data_ptr[0] = self.ra_reader.get_value(self.curr_index[:1])
            start_idx += 1

        for i, l, u, s in islice(
                zip(
                    range(self.n_dims), self.lower_bound_index, self.upper_bound_index,
                    self.step_sizes), start_idx, None):
            if u is None:
                # if the upperbound is undefined, we set it
                self.local_up_idx[i] = len(self.cached_data_ptr[i - 1])

            self.cached_data_ptr[i] = self.cached_data_ptr[i - 1][self.curr_index[i]]

    def __iter__(self):
        return self

    def __next__(self):
        res = self.next()
        if res is None:
            raise StopIteration()
        return res

    def next(self) -> Optional[List[int]]:
        """Get the next index by move current index one step and return it"""
        if self.no_more_val:
            return None

        # for the last dimension, we don't need to re-calculate the upperbound everytime
        # we do that
        dim_pivot = self.unfrozen_dimensions[0]
        self.curr_index[dim_pivot] += self.step_sizes[dim_pivot]
        if self.curr_index[dim_pivot] >= self.local_up_idx[dim_pivot]:
            self.curr_index[dim_pivot] = self.lower_bound_index[dim_pivot]
        else:
            return self.curr_index

        for dim_pivot in islice(self.unfrozen_dimensions, 1, None):
            self.curr_index[dim_pivot] += self.step_sizes[dim_pivot]
            if self.curr_index[dim_pivot] >= self.local_up_idx[dim_pivot]:
                self.curr_index[dim_pivot] = self.lower_bound_index[dim_pivot]
            else:
                # TODO: may be we can still improve the performance
                self.update_local_upperbound_index(dim_pivot)
                return self.curr_index
        else:
            self.no_more_val = True
            if len(self.unfrozen_dimensions) == 0:
                return self.curr_index
            else:
                return None


class ReversedDynamicIndexIterator:
    def __init__(self,
                 ra_reader: 'RAReader',
                 upper_bound_index: List[Optional[int]],
                 lower_bound_index: List[int],
                 step_sizes: List[int] = None):
        self.n_dims = len(upper_bound_index)

        self.ra_reader = ra_reader
        self.step_sizes = [1] * self.n_dims if step_sizes is None else step_sizes
        self.lower_bound_index = lower_bound_index
        self.upper_bound_index = upper_bound_index

        # store the unfrozen dimensions in reversed order
        self.no_more_val = False
        self.unfrozen_dimensions = []

        # init local upper bound index as well
        self.local_up_idx: List[int] = copy.copy(self.upper_bound_index)
        self.cached_data_ptr: list = [None] * self.n_dims
        self.curr_index: List[int] = [None] * self.n_dims
        self.curr_index[0] = self.upper_bound_index[0] if self.step_sizes[0] == 0 else self.upper_bound_index[0] - \
                                                                                       self.step_sizes[0]
        for i, l, u, s in zip(
                range(self.n_dims), lower_bound_index, upper_bound_index, self.step_sizes):
            if (s == 0 and u != l) or (s > 0 and u is not None and u <= l):
                self.no_more_val = True

            if s > 0:
                # this dimension is not frozen
                self.unfrozen_dimensions.append(i)

        self.update_local_upperbound_and_curr_index()
        self.unfrozen_dimensions = list(reversed(self.unfrozen_dimensions))
        if len(self.unfrozen_dimensions) > 0:
            # a trick to generate correct next value at the beginning
            self.curr_index[self.unfrozen_dimensions[0]] += self.step_sizes[self.
                                                                            unfrozen_dimensions[0]]

    def update_local_upperbound_and_curr_index(self, start_idx: int = 0):
        """Recalculate correct upper bound of the current index (in one dimension) always start from second dimension, and then reset the correct index of later dimension"""
        if start_idx == 0:
            # we always know the upperbound of the first (0) dimension
            self.cached_data_ptr[0] = self.ra_reader.get_value(self.curr_index[:1])
            if self.local_up_idx[1] is None:
                self.local_up_idx[1] = len(self.cached_data_ptr[0])
            if self.step_sizes[1] == 0:
                self.curr_index[1] = self.local_up_idx[1]
            else:
                self.curr_index[1] = self.local_up_idx[1] - self.step_sizes[1]
            start_idx += 1

        if self.upper_bound_index[start_idx] is None:
            self.local_up_idx[start_idx] = len(self.cached_data_ptr[start_idx - 1])
        self.cached_data_ptr[start_idx] = self.cached_data_ptr[start_idx - 1][self.curr_index[start_idx]]

        for i, l, u, s in islice(
                zip(
                    range(self.n_dims), self.lower_bound_index, self.upper_bound_index,
                    self.step_sizes), start_idx + 1, None):
            if u is None:
                # if the upperbound is undefined, we set it
                self.local_up_idx[i] = len(self.cached_data_ptr[i - 1])

            if s == 0:
                self.curr_index[i] = self.local_up_idx[i]
            else:
                self.curr_index[i] = self.local_up_idx[i] - s
            self.cached_data_ptr[i] = self.cached_data_ptr[i - 1][self.curr_index[i]]

    def __iter__(self):
        return self

    def __next__(self):
        res = self.next()
        if res is None:
            raise StopIteration()
        return res

    def next(self) -> Optional[List[int]]:
        """Get the next index by move current index one step and return it"""
        if self.no_more_val:
            return None

        # for the last dimension, we don't need to re-calculate the upperbound everytime
        # we do that
        dim_pivot = self.unfrozen_dimensions[0]
        self.curr_index[dim_pivot] -= self.step_sizes[dim_pivot]
        if self.curr_index[dim_pivot] >= self.lower_bound_index[dim_pivot]:
            return self.curr_index

        for dim_pivot in islice(self.unfrozen_dimensions, 1, None):
            self.curr_index[dim_pivot] -= self.step_sizes[dim_pivot]
            if self.curr_index[dim_pivot] >= self.lower_bound_index[dim_pivot]:
                # TODO: may be we can still improve the performance
                self.update_local_upperbound_and_curr_index(dim_pivot)
                return self.curr_index
        else:
            self.no_more_val = True
            if len(self.unfrozen_dimensions) == 0:
                return self.curr_index
            else:
                return None
