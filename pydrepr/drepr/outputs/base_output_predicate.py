from abc import ABC, abstractmethod
from typing import List, Dict, Tuple, Callable, Any, Optional

from drepr.outputs.prop_data_ndarray import PropDataNDArray


class BaseOutputPredicate(ABC):
    # id of the edge in the semantic model
    id: int
    uri: str

    @abstractmethod
    def as_ndarray(self, indexed_predicates: List['BaseOutputPredicate']) -> PropDataNDArray:
        pass

    @abstractmethod
    def o(self) -> Optional['BaseOutputClass']:
        """Get target if this predicate is object property"""
        pass