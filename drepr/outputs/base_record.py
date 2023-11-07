from abc import ABC, abstractmethod
from typing import List, Dict, Tuple, Callable, Any, Optional, Union


class BaseRecord(ABC):
    id: Union[str, tuple]

    @abstractmethod
    def s(self, predicate_uri: str) -> Any:
        pass

    @abstractmethod
    def m(self, predicate_uri: str) -> Any:
        pass

    @abstractmethod
    def us(self, predicate_uri: str, val: Any):
        pass

    @abstractmethod
    def um(self, predicate_uri: str, val: Any):
        pass

    @abstractmethod
    def to_dict(self) -> dict:
        """Return a copied data of this record as a dictionary"""
        pass