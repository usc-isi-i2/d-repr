from collections import defaultdict
from dataclasses import dataclass
from typing import List, Dict, Tuple, Callable, Any, Optional, Union

from drepr.models import SemanticModel, Edge, ClassNode, DataNode


class Namespace:

    def __init__(self, base: str, seen_uris: Optional[Dict[str, str]] = None):
        self._base = base
        self.seen_uris = seen_uris or {}

    def __getattr__(self, item):
        if item not in self.seen_uris:
            uri = self._base + item
            self.seen_uris[item] = uri
        return self.seen_uris[item]

    def __getitem__(self, item):
        if item not in self.seen_uris:
            uri = self._base + item
            self.seen_uris[item] = uri
        return self.seen_uris[item]


class DataProp:
    def __init__(self, sm: SemanticModel, edge: Edge):
        self.sm = sm
        self.edge = edge
        self.data_id = edge.target_id

    @property
    def label(self):
        return self.edge.label

    @property
    def edge_id(self):
        return self.edge.edge_id


class ObjectProp:
    def __init__(self, indexed_sm: 'IndexedSM', edge: Edge, object: 'SMClass'):
        self.indexed_sm = indexed_sm
        self.edge = edge
        self.object: SMClass = object

    @property
    def label(self):
        return self.edge.label

    @property
    def edge_id(self):
        return self.edge.edge_id


class SMClass:
    def __init__(self, indexed_sm: 'IndexedSM', node_id: str):
        self.indexed_sm = indexed_sm
        self.sm = indexed_sm.sm
        self.node_id = node_id
        self.pk_attr: str = ""
        self.uri_attr: Optional[str] = None
        self.predicates: Dict[str, List[Union[DataProp, ObjectProp]]] = defaultdict(lambda: [])
        self.index_po: Dict[Tuple[str, str], List[SMClass]] = defaultdict(lambda: [])

    def _init(self):
        for e in self.sm.iter_outgoing_edges(self.node_id):
            if isinstance(self.sm.nodes[e.target_id], ClassNode):
                o_uri = self.sm.nodes[e.target_id].label
                o = self.indexed_sm.get_class_by_id(e.target_id)
                self.index_po[(e.label, o_uri)].append(o)
                self.predicates[e.label].append(ObjectProp(self.indexed_sm, e, o))
            else:
                if e.label == 'drepr:uri':
                    self.uri_attr = e.target_id
                else:
                    self.predicates[e.label].append(DataProp(self.sm, e))

            if e.is_subject:
                # this requires us to analyze the d-repr output first.
                self.pk_attr = e.target_id

    def is_blank(self) -> bool:
        return self.uri_attr is None

    @property
    def label(self):
        return self.sm.nodes[self.node_id].label

    def p(self, predicate_uri: str) -> List[Union[DataProp, ObjectProp]]:
        """Get list of predicates by their uri"""
        return self.predicates[predicate_uri]

    def o(self, predicate_uri: str, target_uri: str) -> List['SMClass']:
        """Get list of target classes"""
        return self.index_po[(predicate_uri, target_uri)]


class IndexedSM:
    def __init__(self, sm: SemanticModel):
        self.sm = sm
        self.sm_classes: Dict[str, List[SMClass]] = defaultdict(lambda: [])
        self.id2cls: Dict[str, SMClass] = {}

        for n in sm.iter_class_nodes():
            cls = SMClass(self, n.node_id)
            self.sm_classes[n.label].append(cls)
            self.id2cls[n.node_id] = cls

        for c in self.id2cls.values():
            c._init()

    def c(self, class_uri: str) -> List[SMClass]:
        """
        Get a list of classes defined in the semantic model by the URI.
        """
        return self.sm_classes[class_uri]

    def get_class_by_id(self, node_id: str) -> SMClass:
        return self.id2cls[node_id]

    # noinspection PyMethodMayBeStatic
    def ns(self, uri: str) -> Namespace:
        return Namespace(uri)
