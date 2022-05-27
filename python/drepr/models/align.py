from dataclasses import dataclass
from enum import Enum
from typing import List, Union


@dataclass
class AlignedStep:
    source_idx: int
    target_idx: int


@dataclass
class RangeAlignment:
    source: str
    target: str
    aligned_steps: List[AlignedStep]


@dataclass
class ValueAlignment:
    source: str
    target: str


Alignment = Union[RangeAlignment, ValueAlignment]


class AlignmentType(Enum):
    range = "range"
    value = "value"
