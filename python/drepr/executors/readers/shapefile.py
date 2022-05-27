import numpy as np, fiona
from typing import List, Dict, Tuple, Callable, Any, Optional, Union

from drepr.executors.readers.ra_reader import NDArrayReader, Index
from drepr.models import IndexExpr, RangeExpr


class ShapefileReader(NDArrayReader):
    """
    Reader for shapefile resources
    """
    def __init__(self, filename: str, features: List[dict]):
        self.metadata = {"filename": filename}
        self.data = features

    @staticmethod
    def from_file(resource_file):
        with fiona.open(resource_file, 'r') as f:
            features = list(f)
        return ShapefileReader(resource_file, features)

    def len(self) -> int:
        # count the global metadata at '@'
        return len(self.data) + 1

    def len_range(self) -> int:
        return len(self.data)

    def get_value(self, index: List[Index]):
        if index[0] == '@':
            # read metadata
            ptr = self.metadata
            for i in range(1, len(index)):
                ptr = ptr[index[i]]
            return ptr

        ptr = self.data
        for idx in index:
            ptr = ptr[idx]
        return ptr

    def set_value(self, index: List[Index], value):
        if index[0] == '@':
            # update metadata
            ptr = self.metadata
            for i in range(1, len(index) - 1):
                ptr = ptr[index[i]]
            ptr[index[-1]] = value
            return

        ptr = self.data[index[0]]
        for i in range(1, len(index) - 1):
            ptr = ptr[index[i]]
        ptr[index[-1]] = value

    def select(self, steps: List[Union[IndexExpr, RangeExpr]]):
        # steps.0 must be index because there is no structure change
        if isinstance(steps[0], IndexExpr) and steps[0].val == '@':
            # select metadata
            value = self.metadata
            for i in range(1, len(steps)):
                assert isinstance(steps[i], IndexExpr)
                value = value[steps[i].val]
            return value

        value = self._recursive_select(self.data, steps, 0)
        if isinstance(value, list):
            return np.asarray(value)
        return value

    def _recursive_select(self, value, steps: List[Union[IndexExpr, RangeExpr]], i):
        if i >= len(steps):
            return value

        if isinstance(steps[i], IndexExpr):
            return self._recursive_select(value[steps[i].val], steps, i+1)

        if isinstance(steps[i], RangeExpr):
            # special case for 2D geometry
            if all(isinstance(steps[j], RangeExpr) and steps[j].is_select_all() for j in range(i, len(steps))):
                return value

            if steps[i].end is None:
                end = len(value)
            else:
                end = steps[i].end
            return [self._recursive_select(value[j], steps, i+1)
                    for j in range(steps[i].start, end, steps[i].step)]

        raise ValueError("Invalid step: %s" % steps[i])
