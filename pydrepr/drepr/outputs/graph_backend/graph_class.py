from collections import defaultdict
from typing import List, Tuple, Any, Union, Iterable, TYPE_CHECKING, Dict

from drepr.models import ClassNode, SemanticModel, DRepr
from drepr.outputs.record_id import RecordID, GraphRecordID
from drepr.outputs.base_output_class import BaseOutputClass
from drepr.outputs.base_record import BaseRecord

if TYPE_CHECKING:
    from drepr.outputs.graph_backend.graph_backend import GraphBackend
from drepr.outputs.graph_backend.graph_predicate import GraphPredicate
from drepr.outputs.graph_backend.graph_record import GraphRecord
from drepr.outputs.graph_backend.subset_graph_class import SubsetGraphClass


class GraphClass(BaseOutputClass):

    def __init__(self, backend: 'GraphBackend', u: ClassNode, nodes: List[dict]) -> None:
        super().__init__()
        self.backend = backend
        self.cls = u
        self.nodes = {v['@id']: GraphRecord(v) for v in nodes}
        self.predicates: Dict[str, GraphPredicate] = defaultdict(list)
        self._is_blank = True
        uri2edges = defaultdict(list)
        for e in backend.drepr.sm.iter_outgoing_edges(u.node_id):
            if e.label == 'drepr:uri':
                self._is_blank = False
            uri2edges[e.label].append(e)

        for uri, edges in uri2edges.items():
            self.predicates[uri] = GraphPredicate(backend, self, edges)

    def _init(self, backend: 'GraphBackend'):
        for pred in self.predicates.values():
            pred._init(backend)

    @property
    def id(self):
        return self.cls.node_id

    @property
    def uri(self):
        return self.cls.label

    def is_blank(self) -> bool:
        return self._is_blank

    def iter_records(self) -> Iterable[GraphRecord]:
        return iter(self.nodes.values())

    def get_record_by_id(self, rid: RecordID) -> BaseRecord:
        return self.nodes[rid]

    def p(self, predicate_uri: str) -> List[GraphPredicate]:
        return self.predicates[predicate_uri]

    def o(self, predicate_uri: str, target_uri: str) -> List['GraphClass']:
        return self.index_po[predicate_uri, target_uri]

    def filter(self, conditions: Union['FCondition', List['FCondition']]) -> 'SubsetGraphClass':
        pass

    def group_by(self, predicate: Union[str, GraphPredicate]) -> Iterable[Tuple[Any, 'SubsetGraphClass']]:
        if isinstance(predicate, GraphPredicate):
            predicate = predicate.uri

        groups = {}
        for r in self.nodes.values():
            if r.s(predicate) not in groups:
                groups[r.s(predicate)] = SubsetGraphClass(self, [r])
            else:
                groups[r.s(predicate)].nodes.append(r)

        return iter(groups.items())

    def __len__(self):
        return len(self.nodes)