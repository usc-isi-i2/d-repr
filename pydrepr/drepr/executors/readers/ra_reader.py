from abc import ABC, abstractmethod
from typing import Union, List

from drepr.models import IndexExpr, RangeExpr

Index = Union[str, int]


class NDArrayReader(ABC):

    @abstractmethod
    def get_value(self, index: List[Index]):
        pass

    @abstractmethod
    def set_value(self, index: List[Index], value):
        pass

    @abstractmethod
    def select(self, steps: List[Union[IndexExpr, RangeExpr]]):
        pass

    @abstractmethod
    def len(self) -> int:
        pass
