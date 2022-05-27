from dataclasses import dataclass
from typing import List, Optional

import numpy as np

from drepr.outputs.array_backend.array_attr import NoData


@dataclass
class IndexPropRange:
    # start dimension
    start: int
    # end dimension
    end: int


@dataclass
class PropDataNDArray:
    data: np.ndarray
    nodata: Optional[NoData]
    index_props_range: List[IndexPropRange]
    index_props: List[np.ndarray]
