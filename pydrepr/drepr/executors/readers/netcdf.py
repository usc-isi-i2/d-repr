from pathlib import Path
from typing import List, Union

import numpy as np
from netCDF4 import Dataset

from drepr.executors.readers.ra_reader import NDArrayReader, Index
from drepr.models import IndexExpr, RangeExpr


class NetCDF4Reader(NDArrayReader):
    """
        Reader for NetCDF4Resource
    """

    def __init__(self, filename: str, dataset: Dataset):
        self.dataset = dataset
        # read metadata
        self.metadata = {}
        for attr in dataset.ncattrs():
            self.metadata[attr] = getattr(self.dataset, attr)
        self.metadata["filename"] = filename
        self.variables = {
            k: {
                "data": dataset.variables[k],
                "@": {
                    attr: getattr(dataset.variables[k], attr)
                    for attr in dataset.variables[k].ncattrs()
                }
            }
            for k in dataset.variables.keys()
        }

    @classmethod
    def from_file(cls, infile: str):
        ds = Dataset(infile)
        return cls(Path(infile).name, ds)

    def len(self) -> int:
        # count the global metadata at '@'
        return len(self.dataset.variables.keys()) + 1

    def len_range(self) -> int:
        raise Exception("Error in your drepr model")

    def get_value(self, index: List[Index]):
        if index[0] == '@':
            # read metadata
            ptr = self.metadata
            for i in range(1, len(index)):
                ptr = ptr[index[i]]
            return ptr

        # read variables, must be [<variable_id>, data|@, ...]
        ptr = self.variables
        for idx in index:
            ptr = ptr[idx]
        return ptr

    def set_value(self, index: List[Index], value):
        # steps.0 must be index because there is no structure change
        if index[0] == '@':
            # update metadata
            ptr = self.metadata
            for i in range(1, len(index) - 1):
                ptr = ptr[index[i]]
            ptr[index[-1]] = value
            return

        # update variables, but can only update the metadata
        assert index[1] == '@'
        ptr = self.variables[index[0]]['@']
        for i in range(2, len(index) - 2):
            ptr = ptr[index[i]]
        ptr[index[-1]] = value

    def select(self, steps: List[Union[IndexExpr, RangeExpr]]):
        # steps.0 must be index because there is no structure change
        assert isinstance(steps[0], IndexExpr)
        if steps[0].val == "@":
            # select metadata
            value = self.metadata
            for i in range(1, len(steps)):
                assert isinstance(steps[i], IndexExpr)
                value = value[steps[i].val]
            return value

        # select variable
        variable = self.variables[steps[0].val]
        assert isinstance(steps[1], IndexExpr)
        if steps[1].val == '@':
            # select metadata of variable
            value = variable['@']
            for i in range(2, len(steps)):
                assert isinstance(steps[i], IndexExpr)
                value = value[steps[i].val]
            return value

        # select data of variables
        assert steps[1].val == 'data'
        if not isinstance(variable['data'], np.ndarray):
            # lazy load numpy array otherwise, it will be netcdf dataset (data is living in disk)
            variable['data'] = np.asarray(variable['data'])

        # slicing through numpy array is a piece of cake
        value = variable['data']
        for i in range(2, len(steps)):
            if isinstance(steps[i], IndexExpr):
                value = value[steps[i].val]
            else:
                value = value[steps[i].start:steps[i].end:steps[i].step]
        return value