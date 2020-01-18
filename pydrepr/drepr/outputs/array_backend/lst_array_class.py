from typing import List, Dict, Tuple, Callable, Any, Optional, Iterable, TYPE_CHECKING

if TYPE_CHECKING:
    from drepr.outputs.array_backend.array_class import ArrayClass
from drepr.outputs.array_backend.array_record import ArrayRecord
from drepr.outputs.array_backend.subset_array_class import SubsetArrayClass
from drepr.outputs.record_id import RecordID
from drepr.outputs.base_output_predicate import BaseOutputPredicate
from drepr.outputs.base_lst_output_class import BaseLstOutputClass
from drepr.outputs.base_output_class import BaseOutputClass


class LstArrayClass(BaseLstOutputClass):

    def __iter__(self) -> Iterable['ArrayClass']:
        return iter(self.classes)

    def __len__(self) -> int:
        return len(self.classes)

    def __getitem__(self, item: int) -> BaseOutputClass:
        return self.classes[item]

    def p(self, predicate_uri: str) -> List[BaseOutputPredicate]:
        pass

    def o(self, predicate_uri: str, target_uri: str) -> 'BaseOutputClass':
        pass

    def __init__(self, classes: List['ArrayClass']):
        self.classes = classes

    def iter_records(self) -> Iterable[ArrayRecord]:
        for cls in self.classes:
            for r in cls.iter_records():
                yield r

    def filter(self, conditions) -> 'LstArrayClass':
        new_classes = []
        for cls in self.classes:
            cls = cls.filter(conditions)
            if isinstance(cls, SubsetArrayClass) and len(cls.ids) == 0:
                continue
            new_classes.append(cls)
        return LstArrayClass(new_classes)

    def group_by(self, predicate: str) -> Iterable[Tuple[Any, 'ArrayClass']]:
        pass

    def get_record_by_id(self, rid: RecordID):
        pass


