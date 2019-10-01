import gzip
import ujson
from itertools import islice
from pathlib import Path
from typing import List, Any, Union

from drepr.models import Variable, Location
from drepr.services.ra_iterator import RAIterator
from drepr.services.ra_reader.ra_reader import RAReader


class TreeRAReader(RAReader):

    def __init__(self, data: Union[dict, list]):
        super().__init__()
        self.data = data

    def get_value(self, index: List[Union[str, int]], start_idx: int = 0) -> Any:
        ptr = self.data
        for i in islice(index, start_idx, len(index) - 1):
            ptr = ptr[i]

        return ptr[index[-1]]

    def replace_value(self, index: List[Union[str, int]], value: Any, start_idx: int = 0):
        ptr = self.data
        for i in islice(index, start_idx, len(index) - 1):
            ptr = ptr[i]
        ptr[index[-1]] = value

    def insert_value(self, index: List[Union[str, int]], value: Any, start_idx: int = 0):
        ptr = self.data
        for i in islice(index, start_idx, len(index) - 1):
            ptr = ptr[i]

        # expect to call this function in array, if not they should use replace_value
        ptr.insert(index[-1], value)

    def remove_value(self, index: List[Union[str, int]], start_idx: int = 0):
        ptr = self.data
        for i in islice(index, start_idx, len(index) - 1):
            ptr = ptr[i]
        ptr.pop(index[-1])

    def dump2json(self) -> Union[dict, list]:
        return self.data

    def ground_location_mut(self, loc: Location, start_idx: int = 0) -> None:
        ptr = self.data
        for i, slice in enumerate(islice(loc.slices, start_idx, None)):
            if slice.is_range():
                if slice.end is None and ptr is not None:
                    slice.end = len(ptr)

                if ptr is not None and slice.end - slice.start == 1:
                    ptr = ptr[slice.start]
                else:
                    # we don't know how to navigate to nested dimensions
                    ptr = None
            else:
                if ptr is not None:
                    ptr = ptr[slice.idx]


class JSONRAReader(TreeRAReader):

    def __init__(self, fpath: Path):
        open_file = gzip.open if fpath.name.endswith(".gz") else open
        with open_file(str(fpath), encoding="utf-8") as f:
            data = ujson.load(f)

        super().__init__(data)
