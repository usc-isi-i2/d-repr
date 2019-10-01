from typing import List, Tuple, Any, Optional

from drepr.services.index_iterator import ReversedIndexIterator, IndexIterator, ReversedDynamicIndexIterator, \
    DynamicIndexIterator


class RAIterator:
    def __init__(self,
                 ra_reader,
                 upper_bound_index: List[int],
                 lower_bound_index: List[int],
                 step_sizes: List[int] = None,
                 reverse: bool = False,
                 is_dynamic: bool = False):
        self.ra_reader = ra_reader
        if is_dynamic:
            if reverse:
                self.idx_iterator = ReversedDynamicIndexIterator(ra_reader, upper_bound_index,
                                                                 lower_bound_index, step_sizes)
            else:
                self.idx_iterator = DynamicIndexIterator(ra_reader, upper_bound_index,
                                                         lower_bound_index, step_sizes)
        else:
            if reverse:
                self.idx_iterator = ReversedIndexIterator(upper_bound_index, lower_bound_index,
                                                          step_sizes)
            else:
                self.idx_iterator = IndexIterator(upper_bound_index, lower_bound_index, step_sizes)

    def __iter__(self):
        return self

    def __next__(self):
        res = self.next()
        if res is None:
            raise StopIteration()
        return res

    def next(self) -> Optional[Tuple[List[int], Any]]:
        """Loop through the N-dimensional array using some slice"""
        curr_index = self.idx_iterator.next()
        if curr_index is None:
            return None

        return curr_index, self.ra_reader.get_value(curr_index)
