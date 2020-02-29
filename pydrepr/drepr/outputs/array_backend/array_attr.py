from dataclasses import dataclass

import numpy as np
from typing import Union, List, Tuple, Optional

from drepr.models import Path


@dataclass
class NoData:
    value: any


class ArrayAttr:
    def __init__(self, id: str, values: np.ndarray, path: Path, step2dim: List[Optional[int]], nodata: Optional[NoData]):
        self.id = id
        self.values = values
        self.values_shp = values.shape[:sum(int(d is not None) for d in step2dim)]
        self.steps = path.steps
        self.step2dim = step2dim
        self.nodata = nodata

    @property
    def shape(self) -> Tuple[int, ...]:
        return self.values.shape

    @property
    def size(self) -> int:
        return self.values.size

    def get_data(self):
        if not isinstance(self.values, np.ndarray):
            self.values = np.asarray(self.values)
        return self.values

    def get_value(self, index: Tuple[int, ...]):
        return self.values[index]

    def set_value(self, index: Tuple[int, ...], val):
        self.values[index] = val


class ScalarAttr:
    def __init__(self, id: str, value):
        self.id = id
        self.value = value
        self.values = None
        self.nodata = None

    @property
    def shape(self) -> Tuple[int, ...]:
        return ()

    @property
    def size(self):
        return 1

    def get_data(self):
        if self.values is None:
            self.values = np.asarray([self.value])
        return self.values

    def get_value(self, _index: Tuple[int, ...]):
        return self.value

    def set_value(self, _index: Tuple[int, ...], val):
        self.value = val


Attribute = Union[ArrayAttr, ScalarAttr]
