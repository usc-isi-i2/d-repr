from pathlib import Path
from typing import List, Any, Union

import netCDF4
import numpy

from drepr.models import Variable, Location
from drepr.services.ra_reader.ra_reader import RAReader
from drepr.services.ra_reader.tensor_ra_reader import TensorRAReader


class NetCDFRAReader(RAReader):

    def __init__(self, fpath: Path):
        super().__init__()
        net_cdf_file = netCDF4.Dataset(str(fpath))
        self.tensors = {}

        for var in net_cdf_file.variables.values():
            data = numpy.asarray(var).tolist()
            shape = var.shape
            self.tensors[var.name] = TensorRAReader(data, shape)

    def get_value(self, index: List[Union[str, int]], start_idx: int = 0) -> Any:
        return self.tensors[index[0]].get_value(index, start_idx + 1)

    def replace_value(self, index: List[Union[str, int]], value: Any, start_idx: int = 0):
        self.tensors[index[0]].replace_value(index, value, start_idx + 1)

    def insert_value(self, index: List[Union[str, int]], value: Any, start_idx: int = 0):
        self.tensors[index[0]].insert_value(index, value, start_idx + 1)

    def remove_value(self, index: List[Union[str, int]], start_idx: int = 0):
        self.tensors[index[0]].remove_value(index, start_idx + 1)

    def dump2json(self) -> Union[dict, list]:
        return {k: v.dump2json() for k, v in self.tensors.items()}

    def ground_location_mut(self, loc: Location, start_idx: int = 0) -> None:
        self.tensors[loc.slices[start_idx].idx].ground_location_mut(loc, start_idx + 1)
