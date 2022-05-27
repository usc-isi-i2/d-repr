import pickle
from pathlib import Path
from typing import List, Dict, Tuple, Callable, Any, Optional, Union, Generator


class ClassNode:

    def __init__(self, value: str, type: str, outgoing_links: List[int], incoming_links: List[int]):
        self.value = value
        self.type = type
        self.outgoing_links = outgoing_links
        self.incoming_links = incoming_links


class DataNode:

    def __init__(self, value: Any, incoming_links: List[int]):
        self.value = value
        self.incoming_links = incoming_links


class Edge:

    def __init__(self, subject_id: int, predicate: str, object_id: int):
        self.subject_id = subject_id
        self.predicate = predicate
        self.object_id = object_id


class TripleGraph:

    def __init__(self):
        self.nodes: List[Union[ClassNode, DataNode]] = []
        self.edges = []
        self.uri2id = {}

    def add_uri(self, uri: str, type: str):
        if uri not in self.uri2id:
            self.uri2id[uri] = len(self.nodes)
            self.nodes.append(ClassNode(uri, type, [], []))

    def add_object_triple(self, source_uri: str, predicate: str, object_uri: str):
        source_id = self.uri2id[source_uri]
        target_id = self.uri2id[object_uri]

        self.nodes[source_id].outgoing_links.append(len(self.edges))
        self.nodes[target_id].incoming_links.append(len(self.edges))
        self.edges.append(Edge(source_id, predicate, target_id))

    def add_data_triple(self, source_uri: str, predicate: str, object_val: Any):
        source_id = self.uri2id[source_uri]
        target_id = len(self.nodes)

        self.nodes[source_id].outgoing_links.append(len(self.edges))
        self.nodes.append(DataNode(object_val, [len(self.edges)]))
        self.edges.append(Edge(source_id, predicate, target_id))

    def iter_class_nodes(self) -> Generator[ClassNode, None, None]:
        for node_id in self.uri2id.values():
            yield self.nodes[node_id]

    def iter_data_nodes(self) -> Generator[DataNode, None, None]:
        for node in self.nodes:
            if isinstance(node, DataNode):
                yield node

    def iter_outgoing_links(self, node_id) -> Generator[Edge, None, None]:
        for e in self.nodes[node_id].outgoing_links:
            yield self.edges[e]

    def serialize(self, foutput: Union[str, Path]):
        with open(foutput, "wb") as f:
            pickle.dump(self, f)

    @staticmethod
    def deserialize(foutput: Union[str, Path]):
        with open(foutput, "rb") as f:
            return pickle.load(f)
