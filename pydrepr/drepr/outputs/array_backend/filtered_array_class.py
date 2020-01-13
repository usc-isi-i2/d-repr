from typing import List, Dict, Tuple, Callable, Any, Optional, Iterable, TYPE_CHECKING

from drepr.outputs.array_based.array_record import ArrayRecord

if TYPE_CHECKING:
    from drepr.outputs.array_based.array_class import ArrayClass
from drepr.outputs.array_based.base_array_class import BaseArrayClass
from drepr.outputs.array_based.record_id import RecordID


class FilteredArrayClass(BaseArrayClass):
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
