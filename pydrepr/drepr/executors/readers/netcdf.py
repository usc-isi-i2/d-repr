from pathlib import Path
from typing import List

from netCDF4 import Dataset

from drepr.executors.readers.ra_reader import NDArrayReader, Index


class NetCDF4Reader(NDArrayReader):
    """
        Reader for NetCDF4Resource
    """

    def __init__(self, filename: str, dataset: Dataset):
        self.dataset = dataset
        # read metadata
        self.metadata = {
            "filename": filename
        }
        for attr in dataset.ncattrs():
            self.metadata[attr] = getattr(self.dataset, attr)
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
        pass