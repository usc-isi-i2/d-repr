from typing import List, Dict, Tuple, Callable, Any, Optional, Union, Iterable

from drepr.outputs.array_based.types import RecordID


class O2ORange0Func:
    """
    A function that map index of the primary attribute's item to target attribute's item.
    The mapping function is always *-to-1 mapping.
    """
    def __init__(self, target2source: List[int]):
        self.target2source = target2source
        self.is_x2o = True

    def __call__(self, source_idx: RecordID) -> RecordID:
        return tuple(source_idx[i] for i in self.target2source)


class O2MRange0Func:
    """
    """
    def __init__(self):
        self.is_x2o = False

    def __call__(self, source_idx: RecordID) -> Iterable[RecordID]:
        pass


class IdentityFunc:

    def __init__(self):
        self.is_x2o = True

    def __call__(self, source_idx: RecordID) -> RecordID:
        return source_idx


X2OFunc = Union[IdentityFunc, O2ORange0Func]
X2MFunc = Union[O2MRange0Func]
