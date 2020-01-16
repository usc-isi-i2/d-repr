from abc import ABC, abstractmethod
from typing import List, Tuple, Any, Iterable, Union

from drepr.outputs.base_output_predicate import BaseOutputPredicate
from drepr.outputs.base_record import BaseRecord
from drepr.outputs.base_subset_output_class import BaseSubsetOutputClass
from drepr.outputs.record_id import RecordID


class BaseOutputClass(ABC):
    # id of the class in the semantic model
    id: str
    uri: str

    @abstractmethod
    def is_blank(self) -> bool:
        pass

    @abstractmethod
    def iter_records(self) -> Iterable[BaseRecord]:
        pass

    @abstractmethod
    def get_record_by_id(self, rid: RecordID) -> BaseRecord:
        pass

    @abstractmethod
    def p(self, predicate_uri: str) -> List[BaseOutputPredicate]:
        """Get list of predicates associated with this class based on their URIs"""
        pass

    @abstractmethod
    def o(self, predicate_uri: str, target_uri: str) -> List['BaseOutputClass']:
        pass

    @abstractmethod
    def filter(self, conditions: Union['FCondition', List['FCondition']]) -> BaseSubsetOutputClass:
        pass

    @abstractmethod
    def group_by(self, predicate: Union[str, BaseOutputPredicate]) -> Iterable[Tuple[Any, BaseSubsetOutputClass]]:
        """Group records of this classes by values of a predicate"""
        pass