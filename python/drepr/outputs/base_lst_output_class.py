from abc import ABC, abstractmethod
from typing import List, Dict, Tuple, Callable, Any, Optional, Iterable, Union

from drepr.outputs.record_id import RecordID
from drepr.outputs.base_output_class import BaseOutputClass
from drepr.outputs.base_output_predicate import BaseOutputPredicate
from drepr.outputs.base_record import BaseRecord


class BaseLstOutputClass(ABC):

    @abstractmethod
    def iter_records(self) -> Iterable[BaseRecord]:
        """
        """
        pass

    @abstractmethod
    def filter(self, conditions: Union['FCondition', List['FCondition']]) -> 'BaseLstOutputClass':
        pass

    @abstractmethod
    def group_by(self, predicate: Union[str, BaseOutputPredicate]) -> Iterable[Tuple[Any, 'BaseLstOutputClass']]:
        """Group records of this classes by values of a predicate"""
        pass

    @abstractmethod
    def __iter__(self) -> Iterable[BaseOutputClass]:
        pass

    @abstractmethod
    def __len__(self) -> int:
        pass

    @abstractmethod
    def __getitem__(self, item: int) -> BaseOutputClass:
        pass