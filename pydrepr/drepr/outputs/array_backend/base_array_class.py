from abc import ABC, abstractmethod
from typing import List, Dict, Tuple, Callable, Any, Optional, Iterable

from drepr.outputs.array_based.record_id import RecordID


class BaseArrayClass(ABC):
    @abstractmethod
    def iter_records(self) -> Iterable['ArrayRecord']:
        raise

    @abstractmethod
    def get_record_by_id(self, rid: RecordID):
        pass

    @abstractmethod
    def get_data_prop_as_ndarray(self, edge_id: int, index_edge_ids: List[int]) -> 'PropDataNDArray':
        pass

    @abstractmethod
    def filter(self, conditions) -> 'BaseArrayClass':
        pass

    @abstractmethod
    def group_by(self, predicate: str) -> Iterable[Tuple[Any, 'ArrayClass']]:
        pass