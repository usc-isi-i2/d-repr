from typing import List, Dict, Tuple, Callable, Any, Optional, Union

from drepr.models import Variable, Location
from drepr.services.ra_iterator import RAIterator
from drepr.services.ra_reader.ra_reader import RAReader


class MultiRAReader(RAReader):
    def __init__(self, ra_readers: Dict[str, RAReader]):
        self.ra_readers = ra_readers

    def get_value(self, index: List[Union[str, int]], start_idx: int = 0) -> Any:
        return self.ra_readers[index[start_idx]].get_value(index, start_idx + 1)

    def replace_value(self, index: List[Union[str, int]], value: Any, start_idx: int = 0):
        self.ra_readers[index[start_idx]].replace_value(index, value, start_idx + 1)

    def insert_value(self, index: List[Union[str, int]], value: Any, start_idx: int = 0):
        self.ra_readers[index[start_idx]].insert_value(index, value, start_idx + 1)

    def remove_value(self, index: List[Union[str, int]], start_idx: int = 0):
        self.ra_readers[index[start_idx]].remove_value(index, start_idx + 1)

    def ground_location_mut(self, loc: Location, start_idx: int = 0) -> None:
        return self.ra_readers[loc.resource_id].ground_location_mut(loc, start_idx)

    def dump2json(self) -> Union[dict, list]:
        return {k: v.dump2json() for k, v in self.ra_readers.items()}

    def iter_data(self, grounded_loc: Location, reverse: bool = False) -> RAIterator:
        upperbound = [grounded_loc.resource_id]
        lowerbound = [grounded_loc.resource_id]
        step_sizes = [0]
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

