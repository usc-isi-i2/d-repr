from abc import ABC, abstractmethod
from typing import Union, List

from drepr.models import IndexExpr, RangeExpr

Index = Union[str, int]


class NDArrayReader(ABC):

    @abstractmethod
    def get_value(self, index: List[Index]):
        """
        Get value at the given index
        """
        pass

    @abstractmethod
    def set_value(self, index: List[Index], value):
        """
        Assigning value to the given index
        """
        pass

    @abstractmethod
    def select(self, steps: List[Union[IndexExpr, RangeExpr]]):
        """
        Select values that match the path.
        """
        pass

    @abstractmethod
    def len(self) -> int:
        """
        Get length of the current node.
        """
        pass

    @abstractmethod
    def len_range(self) -> int:
        """
        Get length of the current node (range only).
        """
        pass
