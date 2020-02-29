from typing import List, Dict, Tuple, Callable, Any, Optional, Union

import ujson
import numpy as np
from drepr.executors.readers.ra_reader import NDArrayReader, Index
from drepr.models import IndexExpr, RangeExpr


class NPDictReader(NDArrayReader):
    """
    A key-value resource for the purpose of testing. Any array in this resource will be read as numpy array
    """
    def __init__(self, data):
        self.data = data

    @classmethod
    def from_file(cls, infile: str):
        with open(infile, "r") as f:
            data = ujson.load(f)
            for k, v in data.items():
                if isinstance(v, list):
                    data[k] = np.asarray(v)
        return NPDictReader(data)

    def get_value(self, index: List[Index]):
        return self.data[index[0]]

    def set_value(self, index: List[Index], value):
        raise NotImplementedError()

    def select(self, steps: List[Union[IndexExpr, RangeExpr]]):
        assert isinstance(steps[0], IndexExpr)
        return self.data[steps[0].val]

    def len(self) -> int:
        return len(self.data)

    def len_range(self) -> int:
        raise Exception("Error in your drepr model")