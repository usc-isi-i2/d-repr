from typing import List, Dict, Tuple, Callable, Any, Optional, Iterable, Union, TYPE_CHECKING

from drepr.outputs.base_output_predicate import BaseOutputPredicate
from drepr.outputs.base_record import BaseRecord
from drepr.outputs.base_lst_output_class import BaseLstOutputClass
from drepr.outputs.base_output_class import BaseOutputClass

if TYPE_CHECKING:
    from drepr.outputs.graph_backend.graph_class import GraphClass
from drepr.outputs.record_id import GraphRecordID


class LstGraphClass(BaseLstOutputClass):

    def __init__(self, classes: List['GraphClass']) -> None:
        super().__init__()
        self.classes = classes

    def iter_records(self) -> Iterable[BaseRecord]:
        pass

    def get_record_by_id(self, rid: GraphRecordID) -> BaseRecord:
        pass

    def p(self, predicate_uri: str) -> List[BaseOutputPredicate]:
        pass

    def o(self, predicate_uri: str, target_uri: str) -> List['BaseOutputClass']:
        pass

    def filter(self, conditions: Union['FCondition', List['FCondition']]) -> 'BaseSubsetOutputClass':
        pass

    def group_by(self, predicate: Union[str, BaseOutputPredicate]) -> Iterable[Tuple[Any, 'BaseSubsetOutputClass']]:
        pass

    def __iter__(self) -> Iterable[BaseOutputClass]:
        return iter(self.classes)

    def __len__(self) -> int:
        return len(self.classes)

    def __getitem__(self, item: int) -> BaseOutputClass:
        return self.classes[item]