from collections import defaultdict
from typing import Union, Dict, List, Iterable

from drepr.engine import execute, MemoryOutput, OutputFormat
from drepr.models import DRepr, SemanticModel
from drepr.outputs.base_lst_output_class import BaseLstOutputClass
from drepr.outputs.base_output_class import BaseOutputClass
from drepr.outputs.base_output_sm import BaseOutputSM
from drepr.outputs.base_record import BaseRecord
from drepr.outputs.graph_backend.graph_class import GraphClass
from drepr.outputs.graph_backend.lst_graph_class import LstGraphClass
from drepr.outputs.namespace import Namespace
from drepr.outputs.record_id import GraphRecordID


class GraphBackend(BaseOutputSM):

    def __init__(self, class2nodes: Dict[str, List[dict]], drepr: DRepr):
        self.drepr = drepr
        self.sm = drepr.sm
        self.classes: Dict[str, GraphClass] = {}
        self.uri2classes: Dict[str, List[GraphClass]] = defaultdict(list)

        for class_id, nodes in class2nodes.items():
            for u in nodes:
                u['@id'] = GraphRecordID(u['@id'], class_id)

        for c in self.sm.iter_class_nodes():
            self.classes[c.node_id] = GraphClass(self, c, class2nodes[c.node_id])
            self.uri2classes[c.label].append(self.classes[c.node_id])

    @classmethod
    def from_drepr(cls, ds_model: Union[DRepr, str], resources: Union[str, Dict[str, str]]) -> "GraphBackend":
        if type(ds_model) is str:
            ds_model = DRepr.parse_from_file(ds_model)

        class2nodes = execute(ds_model, resources, MemoryOutput(OutputFormat.GraphPy))
        return cls(class2nodes, ds_model)

    def iter_classes(self) -> Iterable[BaseOutputClass]:
        return iter(self.classes.values())

    def get_record_by_id(self, rid: GraphRecordID) -> BaseRecord:
        return self.classes[rid.class_id].get_record_by_id(rid)

    def c(self, class_uri: str) -> BaseLstOutputClass:
        return LstGraphClass(self.uri2classes[class_uri])

    def cid(self, class_id: str) -> GraphClass:
        return self.classes[class_id]

    def get_sm(self) -> SemanticModel:
        return self.sm

