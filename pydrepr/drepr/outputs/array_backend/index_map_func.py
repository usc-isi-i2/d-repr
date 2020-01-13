from typing import List, Dict, Tuple, Callable, Any, Optional, Union, Iterable

class O2ORange0Func:
    """
    A function that map index of the primary attribute's item to target attribute's item.
    The mapping function is always *-to-1 mapping.
    """
    def __init__(self, target2source: List[int]):
        self.target2source = target2source
        self.is_x2o = True

    def __call__(self, source_idx: Tuple[int, ...]) -> Tuple[int, ...]:
        return tuple(source_idx[i] for i in self.target2source)


class O2MRange0Func:
    """
    """
    def __init__(self):
        self.is_x2o = False

    def __call__(self, source_idx: Tuple[int, ...]) -> Iterable[Tuple[int, ...]]:
        pass


class IdentityFunc:

    def __init__(self):
        self.is_x2o = True

    def __call__(self, source_idx: Tuple[int, ...]) -> Tuple[int, ...]:
        return source_idx


X2OFunc = Union[IdentityFunc, O2ORange0Func]
X2MFunc = Union[O2MRange0Func]
