from dataclasses import dataclass

import numpy as np
from typing import Union, List, Tuple, Optional

from drepr.models import Path


@dataclass
class NoData:
    value: any


class ColArray:
    def __init__(self, id: str, values: np.ndarray, path: Path, step2dim: List[Optional[int]], nodata: Optional[NoData]):
        self.id = id
        self.values = values
        self.steps = path.steps
        self.step2dim = step2dim
        self.nodata = nodata

    @property
    def shape(self) -> Tuple[int, ...]:
        return self.values.shape

    def get_data(self):
        if not isinstance(self.values, np.ndarray):
            self.values = np.asarray(self.values)
        return self.values


class ColSingle:
    def __init__(self, value):
        self.value = value
        self.values = None

    @property
    def shape(self) -> Tuple[int, ...]:
        return ()

    def get_data(self):
        if self.values is None:
            self.values = np.asarray([self.value])
        return self.values


NDArrayColumn = Union[ColArray, ColSingle]