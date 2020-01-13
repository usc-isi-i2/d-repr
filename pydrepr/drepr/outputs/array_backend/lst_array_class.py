from typing import List, Dict, Tuple, Callable, Any, Optional, Iterable

from drepr.outputs.array_based.array_class import ArrayClass
from drepr.outputs.array_based.base_array_class import BaseArrayClass
from drepr.outputs.array_based.array_record import ArrayRecord
from drepr.outputs.array_based.record_id import RecordID


class LstArrayClass(BaseArrayClass):
    def __init__(self, classes: List[ArrayClass]):
        self.classes = classes

    def iter_records(self) -> Iterable[ArrayRecord]:
        for cls in self.classes:
            for r in cls.iter_records():
                yield r

    def get_data_prop_as_ndarray(self, edge_id: int, index_edge_ids: List[int]) -> 'PropDataNDArray':
        pass

    def filter(self, conditions) -> 'BaseArrayClass':
        pass

    def group_by(self, predicate: str) -> Iterable[Tuple[Any, 'ArrayClass']]:
        pass

    def get_record_by_id(self, rid: RecordID):
        pass


