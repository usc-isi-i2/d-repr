from typing import List, Dict, Tuple, Callable, Any, Optional, Union

from drepr.outputs.array_based.types import RecordID


class PK2AttrFunc:
    """
    A function that map index of the primary attribute's item to target attribute's item.
    The mapping function is always *-to-1 mapping.
    """
    def __init__(self, target2source: List[int]):
        self.target2source = target2source

    def __call__(self, source_idx: RecordID) -> RecordID:
        return tuple(source_idx[i] for i in self.target2source)


class IdentityFunc:
    def __call__(self, source_idx: RecordID) -> RecordID:
        return source_idx


IndexMapFunc = Union[IdentityFunc, PK2AttrFunc]