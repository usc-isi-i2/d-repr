from typing import List, Dict, Tuple, Callable, Any, Optional, Iterable, TYPE_CHECKING

from drepr.outputs.array_backend.array_record import ArrayRecord
from drepr.outputs.base_output_predicate import BaseOutputPredicate
from drepr.outputs.base_subset_output_class import BaseSubsetOutputClass

if TYPE_CHECKING:
    from drepr.outputs.array_backend.array_class import ArrayClass
from drepr.outputs.record_id import RecordID


class SubsetArrayClass(BaseSubsetOutputClass):
    def is_blank(self) -> bool:
        pass

    def p(self, predicate_uri: str) -> List[BaseOutputPredicate]:
        pass

    def o(self, predicate_uri: str, target_uri: str) -> List['BaseSubsetOutputClass']:
        pass

    def __init__(self, cls: 'ArrayClass', ids: Iterable[RecordID]):
        self.cls = cls
        self.ids = ids

    def iter_records(self):
        for id in self.ids:
            yield ArrayRecord(id.index, self.cls)

    def get_record_by_id(self, rid: RecordID):
        return self.cls.get_record_by_id(rid)

    def get_data_prop_as_ndarray(self, edge_id: int, index_edge_ids: List[int]):
        pass

    def filter(self, conditions):
        pass

    def group_by(self, pred_uri: str):
        pass
