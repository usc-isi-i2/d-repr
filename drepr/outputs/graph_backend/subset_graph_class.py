from typing import List, Tuple, Any, Union, Iterable, TYPE_CHECKING

from drepr.outputs.base_output_predicate import BaseOutputPredicate
from drepr.outputs.base_record import BaseRecord
from drepr.outputs.base_subset_output_class import BaseSubsetOutputClass
from drepr.outputs.graph_backend.graph_record import GraphRecord
from drepr.outputs.record_id import RecordID

if TYPE_CHECKING:
    from drepr.outputs.graph_backend.graph_class import GraphClass


class SubsetGraphClass(BaseSubsetOutputClass):

    def __init__(self, cls: 'GraphClass', nodes: List[GraphRecord]) -> None:
        super().__init__()
        self.cls = cls
        self.nodes = nodes

    def is_blank(self) -> bool:
        return self.cls.is_blank()

    def iter_records(self) -> Iterable[GraphRecord]:
        return self.nodes

    def get_record_by_id(self, rid: RecordID) -> BaseRecord:
        pass

    def p(self, predicate_uri: str) -> List[BaseOutputPredicate]:
        return self.cls.p(predicate_uri)

    def o(self, predicate_uri: str, target_uri: str) -> List['BaseOutputClass']:
        # return self.cls.p()
        pass

    def filter(self, conditions: Union['FCondition', List['FCondition']]) -> 'BaseSubsetOutputClass':
        pass

    def group_by(self, predicate: Union[str, BaseOutputPredicate]) -> Iterable[Tuple[Any, 'BaseSubsetOutputClass']]:
        pass