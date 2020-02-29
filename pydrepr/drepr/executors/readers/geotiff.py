from pathlib import Path
from typing import List, Union

import numpy as np

from PIL import Image
from PIL.TiffTags import TAGS

from drepr.executors.readers.ra_reader import NDArrayReader, Index
from drepr.models import IndexExpr, RangeExpr


class GeoTIFFReader(NDArrayReader):
    """
    Reader for GeoTIFF Resources
    """
    def __init__(self, metadata: dict, data: np.ndarray):
        self.metadata = metadata
        self.data = data

    @classmethod
    def from_file(cls, infile: str):
        with Image.open(infile) as img:
            metadata = {TAGS[key]: img.tag[key] for key in img.tag.keys()}

            # set other metadata
            metadata["filename"] = Path(infile).name
            data = np.asarray(data)
        return GeoTIFFReader(metadata, data)
    
    def len(self) -> int:
        return self.data.shape[0] + 1

    def len_range(self) -> int:
        return self.data.shape[0]

    def select(self, steps: List[Union[IndexExpr, RangeExpr]]):
        if isinstance(steps[0], IndexExpr) and steps[0].val == '@':
            # select metadata
            value = self.metadata
            for i in range(1, len(steps)):
                assert isinstance(steps[i], IndexExpr)
                value = value[steps[i].val]
            return value
        
        # select data from ndarray
        value = self.data
        for step in steps:
            if isinstance(step, IndexExpr):
                value = value[step[i].val]
            elif isinstance(step, RangeExpr):
                value = value[steps[i].start:steps[i].end:steps[i].step]
            else:
                raise NotImplementedError()
        return value
    
    def set_value(self, index: List[Index], value):
        if index[0] == '@':
            ptr = self.metadata
        else:
            ptr = self.data

        for i in range(1, len(index) - 1):
            ptr = ptr[index[i]]
        ptr[index[-1]] = value

    def get_value(self, index: List[Index]):
        if index[0] == '@':
            # read metadata
            ptr = self.metadata
        else:
            ptr = self.data

        for idx in index:
            ptr = ptr[idx]
        return ptr