from dataclasses import dataclass
from typing import List, Optional

import numpy as np

from drepr.outputs.array_based.array_attr import NoData


@dataclass
class IndexPropRange:
    start: int
    end: int


@dataclass
class PropDataNDArray:
    data: np.ndarray
    nodata: Optional[NoData]
    index_props_range: List[IndexPropRange]
    index_props: List[np.ndarray]
