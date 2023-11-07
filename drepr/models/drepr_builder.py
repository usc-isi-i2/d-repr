from collections import defaultdict
from typing import List, Optional, Union

from .align import Alignment
from .attr import Attr
from .drepr import DRepr
from .preprocessing import Preprocessing
from .resource import Resource
from .sm import SemanticModel, DataType, DataNode, Edge, LiteralNode, ClassNode


class DReprBuilder:
    def __init__(self):
        self.resources: List[Resource] = []
        self.preprocessing: List[Preprocessing] = []
        self.attrs: List[Attr] = []
        self.aligns: List[Alignment] = []
        self.sm: SemanticModel = SemanticModel({}, [], {})

    def add_resource(self, resource: Resource) -> 'DReprBuilder':
        self.resources.append(resource)
        return self

    def add_preprocessing(self, preprocessing: Preprocessing) -> 'DReprBuilder':
        self.preprocessing.append(preprocessing)
        return self

    def add_attribute(self, attr: Attr) -> 'DReprBuilder':
        self.attrs.append(attr)
        return self

    def add_alignment(self, align: Alignment) -> 'DReprBuilder':
        self.aligns.append(align)
        return self

    def add_sm(self) -> 'SMBuilder':
        return SemanticModelBuilder(self)

    def build(self) -> DRepr:
        DRepr(self.resources, self.preprocessing, self.attrs, self.aligns, self.sm)


class SemanticModelBuilder:
    def __init__(self, repr_builder: DReprBuilder):
        self.repr_builder = repr_builder
        self.class_nodes = defaultdict(lambda: 0)
        self.sm: SemanticModel = self.repr_builder.sm

    def add_prefix(self, prefix: str, uri: str):
        self.sm.prefixes[prefix] = uri
        return self

    def add_class(self, class_name: str) -> 'ClassBuilder':
        self.class_nodes[class_name] += 1
        class_id = f"{class_name}:{self.class_nodes[class_name]}"
        self.sm.nodes[class_id] = ClassNode(class_id, class_name)
        return ClassBuilder(self, class_id)

    def add_relation(self, source_class_id: str, target_class_id: str, predicate: str):
        self.sm.edges.append(Edge(source_class_id, target_class_id, predicate))
        return self

    def finish(self) -> DReprBuilder:
        return self.repr_builder


class ClassBuilder:
    def __init__(self, sm_builder: SemanticModelBuilder, class_id):
        self.sm_builder = sm_builder
        self.class_id = class_id

    def add_data_node(self, predicate: str, attr_id: str, dtype: Optional[DataType] = None, is_subject: bool = False):
        node_id = f"dnode:{attr_id}"
        assert node_id not in self.sm_builder.sm.nodes
        self.sm_builder.sm.nodes[node_id] = DataNode(node_id, attr_id, dtype)
        self.sm_builder.sm.edges.append(Edge(self.class_id, node_id, predicate, is_subject))
        return self

    def add_literal_node(self, predicate: str, val: Union[str, int, float], dtype: Optional[DataType] = None):
        node_id = f"lnode:{len(self.sm_builder.sm.nodes)}"
        self.sm_builder.sm.nodes[node_id] = LiteralNode(node_id, val, dtype)
        self.sm_builder.sm.edges.append(Edge(self.class_id, node_id, predicate))
        return self

    def add_class(self, predicate: str, class_name: str):
        child_cls_builder = self.sm_builder.add_class(class_name)
        self.sm_builder.sm.edges.append(Edge(self.class_id, child_cls_builder.class_id, predicate))
        return child_cls_builder

    def finish(self) -> SemanticModelBuilder:
        return self.sm_builder
