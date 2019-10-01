from collections import defaultdict
from dataclasses import dataclass
from itertools import islice
from typing import Union, Dict, List, Iterable, Optional

import ujson

from drepr.models import DRepr
from drepr.engine import execute, StringOutput, OutputFormat
from drepr.models.sm import ClassNode


@dataclass
class Edge:
    id: int
    source: int
    target: int
    label: str


@dataclass
class Node:
    id: int
    data: dict
    edges_out: List[int]
    edges_in: List[int]


class Graph:
    def __init__(self, nodes: List[Node], edges: List[Edge], ds_model: Optional[DRepr] = None):
        self.nodes = nodes
        self.edges = edges

        if ds_model is not None:
            self.prefixes: Dict[str, str] = ds_model.sm.prefixes
            self.class2nodes: Dict[str, List[int]] = {
                n.label: []
                for n in ds_model.sm.nodes.values() if isinstance(n, ClassNode)
            }
        else:
            self.prefixes: Dict[str, str] = {}
            self.class2nodes: Dict[str, List[int]] = defaultdict(lambda: [])

        self.class2nodes[None] = []
        for n in nodes:
            self.class2nodes[n.data.get('@type', None)].append(n.id)

    def iter_nodes(self) -> Iterable[Node]:
        return self.nodes

    def iter_nodes_by_class(self, cls: str) -> Iterable[Node]:
        for nid in self.class2nodes[cls]:
            yield self.nodes[nid]

    def iter_edges(self) -> Iterable[Edge]:
        return self.edges

    @staticmethod
    def from_drepr(ds_model: DRepr, resources: Union[str, Dict[str, str]]) -> "Graph":
        result = execute(ds_model, resources, StringOutput(OutputFormat.GraphJSON))
        ser_nodes = result["nodes"].split("\n")
        ser_edges = result["edges"].split("\n")
        assert ser_nodes[-1] == "" and ser_edges[-1] == ""

        nodes = []
        edges = []
        for ser_node in islice(ser_nodes, 0, len(ser_nodes) - 1):
            u = ujson.loads(ser_node)
            nodes.append(Node(u['id'], u['data'], [], []))

        for ser_edge in islice(ser_edges, 0, len(ser_edges) - 1):
            sid, tid, lbl = ser_edge.split("\t")
            sid = int(sid)
            tid = int(tid)

            eid = len(edges)
            edges.append(Edge(eid, sid, tid, lbl))
            nodes[tid].edges_in.append(eid)
            nodes[sid].edges_out.append(eid)

        return Graph(nodes, edges, ds_model)

    def serialize(self, fpath: str):
        with open(fpath, 'w') as f:
            ujson.dump(f, {"prefixes": self.prefixes, "nodes": self.nodes, "edges": self.edges})
