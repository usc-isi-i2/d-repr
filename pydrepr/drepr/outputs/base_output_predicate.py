from abc import ABC, abstractmethod
from typing import List, TYPE_CHECKING, Optional

from drepr.models import Edge

if TYPE_CHECKING:
    from drepr.outputs.base_lst_output_class import BaseLstOutputClass
from drepr.outputs.prop_data_ndarray import PropDataNDArray


class BaseOutputPredicate(ABC):
    # URI of the predicate
    uri: str
    # list of outgoing edges of the same class that represent the same predicate.
    edges: List[Edge]

    @abstractmethod
    def as_ndarray(self, indexed_predicates: List['BaseOutputPredicate']) -> PropDataNDArray:
        """Get values of this predicate as nd-array, where each item in the array is a value of this predicate
        of one instance.

        This function will throw exceptions if values of the predicate of one instance are more than one.
        """
        pass

    @abstractmethod
    def o(self) -> Optional['BaseLstOutputClass']:
        """
        Get targets if this predicate is object property.
        We have more than one target when the same predicate is linked to classes that have the same URI.
        """
        pass
