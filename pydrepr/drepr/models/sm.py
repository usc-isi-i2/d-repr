from dataclasses import dataclass
from enum import Enum
from typing import List, Dict, Optional, Union

from .attr import Attr


class DataType(Enum):
    xsd_decimal = "xsd:decimal"
    xsd_anyURI = "xsd:anyURI"
    xsd_gYear = "xsd:gYear"
    xsd_dateTime = "xsd:dateTime"
    xsd_int = "xsd:int"
    xsd_string = "xsd:string"


@dataclass
class ClassNode:
    node_id: str
    label: str


@dataclass
class DataNode:
    node_id: str
    attr_id: str
    data_type: Optional[DataType] = None


@dataclass
class LiteralNode:
    node_id: str
    value: str
    data_type: Optional[DataType] = None


@dataclass
class Edge:
    source_id: str
    target_id: str
    label: str
    is_subject: bool = False
    is_required: bool = False


Node = Union[LiteralNode, DataNode, ClassNode]


@dataclass
class SemanticModel:
    nodes: Dict[str, Node]
    edges: List[Edge]
    prefixes: Dict[str, str]

    @staticmethod
    def get_default(attrs: List[Attr]) -> 'SemanticModel':
        """
        Automatically generate a semantic model from a list of attributes.

        WARNING: the engine may not able to map data to this semantic model if the final output should be
        comprised of multiple tables.
        """
        prefixes = {"eg": "https://example.org/"}
        aids = {attr.id for attr in attrs}
        cid = None
        for i in range(len(attrs)):
            cid = f"c{i}"
            if cid not in aids:
                break
        nodes = {cid: ClassNode(cid, "eg:Record")}
        edges = []
        for attr in attrs:
            nodes[attr.id] = DataNode(attr.id, attr.id, None)
            edges.append(Edge(cid, attr.id, f"eg:{attr.id}"))

        return SemanticModel(nodes, edges, prefixes)

    @staticmethod
    def get_default_prefixes() -> Dict[str, str]:
        return {
            'drepr': "https://purl.org/drepr/1.0/",
            'rdf': "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
            'rdfs': "http://www.w3.org/2000/01/rdf-schema#",
            'owl': "http://www.w3.org/2002/07/owl#"
        }

    @staticmethod
    def deserialize(raw: dict) -> "SemanticModel":
        nodes = {}
        for nid, n in raw['nodes'].items():
            if n['type'] == 'class_node':
                nodes[nid] = ClassNode(n['node_id'], n['label'])
            elif n['type'] == 'data_node':
                nodes[nid] = DataNode(n['node_id'], n['attr_id'], DataType(n['data_type']) if n['data_type'] is not None else None)
            elif n['type'] == 'literal_node':
                nodes[nid] = LiteralNode(n['node_id'], n['value'], DataType(n['data_type']) if n['data_type'] is not None else None)
            else:
                raise NotImplementedError()
        edges = [Edge(**e) for e in raw['edges']]
        return SemanticModel(nodes, edges, raw['prefixes'])

    def remove_node(self, node_id: str):
        self.nodes.pop(node_id)
        for i in range(len(self.edges) - 1, -1, -1):
            if self.edges[i].source_id == node_id or self.edges[i].target_id == node_id:
                self.edges.pop(i)

    def iter_outgoing_edges(self, node_id: str):
        for e in self.edges:
            if e.source_id == node_id:
                yield e

    def iter_incoming_edges(self, node_id: str):
        for e in self.edges:
            if e.target_id == node_id:
                yield e

    def iter_child_nodes(self, node_id: str):
        for e in self.edges:
            if e.source_id == node_id:
                yield self.nodes[e.source_id]

    def iter_parent_nodes(self, node_id: str):
        for e in self.edges:
            if e.target_id == node_id:
                yield self.nodes[e.target_id]

    def get_rel_iri(self, abs_iri: str) -> str:
        for prefix, uri in self.prefixes.items():
            if abs_iri.startswith(uri):
                return f"{prefix}:{abs_iri.replace(uri, '')}"
        raise ValueError("Create create relative IRI because there is no suitable prefix")

    def get_abs_iri(self, rel_iri: str) -> str:
        prefix, val = rel_iri.split(":", 1)
        if prefix not in self.prefixes:
            raise ValueError(f"Cannot create absolute IRI because the prefix {prefix} does not exist")
        return f"{self.prefixes[prefix]}{val}"

    def is_rel_iri(self, iri: str) -> bool:
        return iri.find(":") != -1
