from abc import ABC, abstractmethod
from typing import List, Tuple, Any, Iterable, Union, Optional

from drepr.outputs.base_output_predicate import BaseOutputPredicate
from drepr.outputs.base_record import BaseRecord
from drepr.outputs.record_id import RecordID


class BaseOutputClass(ABC):
    # id of the class in the semantic model
    id: str
    uri: str

    @abstractmethod
    def is_blank(self) -> bool:
        """Return true if the class does not has `drepr:uri` predicate"""
        pass

    @abstractmethod
    def iter_records(self) -> Iterable[BaseRecord]:
        """Iterate through each record of this class"""
        pass

    @abstractmethod
    def get_record_by_id(self, rid: RecordID) -> BaseRecord:
        """Get a record by id"""
        pass

    @abstractmethod
    def p(self, predicate_uri: str) -> Optional[BaseOutputPredicate]:
        """Get a outgoing predicate by URI. Return None if the class does not have URI"""
        pass

    @abstractmethod
    def filter(self, conditions: Union['FCondition', List['FCondition']]) -> 'BaseOutputClass':
        """Filter out records that does not satisfied the giving condition"""
        pass

    @abstractmethod
    def group_by(self, predicate: Union[str, BaseOutputPredicate]) -> Iterable[Tuple[Any, 'BaseOutputClass']]:
        """Group records of this classes by values of a predicate"""
        pass
