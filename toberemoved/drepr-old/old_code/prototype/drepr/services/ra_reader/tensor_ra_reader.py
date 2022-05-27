import csv
import gzip
from itertools import islice
from pathlib import Path
from typing import List, Tuple, Any, Union, Optional

from drepr.models import Location
from drepr.services.ra_reader.ra_reader import RAReader


class TensorRAReader(RAReader):
    # TODO: we can improve this by detecting if the shape is changed when executing preprocessing function

    def __init__(self, data: list, shape: Optional[List[int]]):
        super().__init__()
        self.data = data
        self.shape = shape

    def get_value(self, index: List[Union[str, int]], start_idx: int = 0) -> Any:
        ptr = self.data
        for i in islice(index, start_idx, len(index) - 1):
            ptr = ptr[i]

        return ptr[index[-1]]

    def replace_value(self, index: List[Union[str, int]], value: Any, start_idx: int = 0):
        ptr = self.data
        for i in islice(index, start_idx, len(index) - 1):
            ptr = ptr[i]

        if not (isinstance(value, (str, float, int)) and type(ptr[index[-1]]) is type(value)):
            # the dimension was changed, it is not tensor ra reader anymore
            self.shape = None
        ptr[index[-1]] = value

    def insert_value(self, index: List[Union[str, int]], value: Any, start_idx: int = 0):
        ptr = self.data
        for i in islice(index, start_idx, len(index) - 1):
            ptr = ptr[i]

        if isinstance(value, (str, float, int)) and self.shape is not None:
            self.shape[-1] += 1
        else:
            self.shape = None

        ptr.insert(index[-1], value)

    def remove_value(self, index: List[Union[str, int]], start_idx: int = 0):
        ptr = self.data
        for i in islice(index, start_idx, len(index) - 1):
            ptr = ptr[i]

        if self.shape is not None:
            self.shape[-1] -= 1
        ptr.pop(index[-1])

    def dump2json(self) -> Union[dict, list]:
        return self.data

    def ground_location_mut(self, loc: Location, start_idx: int = 0) -> None:
        if self.shape is not None:
            for i, slice in enumerate(islice(loc.slices, start_idx, None)):
                if slice.is_range() and slice.end is None:
                    slice.end = self.shape[i]
        else:
            # can only ground the first dimension based on what we've known
            if loc.slices[start_idx].is_range() and loc.slices[start_idx].end is None:
                loc.slices[start_idx].end = len(self.data)


class CSVRAReader(TensorRAReader):

    def __init__(self, fpath: Path, quotechar='"', delimiter=","):
        open_file = gzip.open if fpath.name.endswith(".gz") else open
        with open_file(str(fpath), mode="rt", encoding="utf-8-sig") as f:
            reader = csv.reader(f, quotechar=quotechar, delimiter=delimiter)
            data = [row for row in reader]
            shape = [len(data), len(data[0])]

        super().__init__(data, shape)


