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


class SMClass:
    def __init__(self, sm: SemanticModel, node_id: str):
        self.sm = sm
        self.node_id = node_id
        self.pk_attr = None
        self.predicates = defaultdict(lambda: [])
        self.index_po = defaultdict(lambda: [])

        for e in sm.iter_outgoing_edges(node_id):
            if isinstance(sm.nodes[e.target_id], ClassNode):
                self.index_po[(e.label, sm.nodes[e.target_id].label)].append(SMClass(sm, e.target_id))
            else:
                self.predicates[e.label].append(DataProp(sm, e))

            if e.is_subject:
                # this requires us to analyze the d-repr output first.
                self.pk_attr = e.target_id

    @property
    def label(self):
        return self.sm.nodes[self.node_id].label

    def p(self, predicate_uri: str) -> List[DataProp]:
        """Get list of predicates by their uri"""
        return self.predicates[predicate_uri]

    def o(self, predicate_uri: str, target_uri: str) -> List['SMClass']:
        """Get list of target classes"""
        return self.index_po[(predicate_uri, target_uri)]


class IndexedSM:
    def __init__(self, sm: SemanticModel):
        self.sm = sm
        self.sm_classes = defaultdict(lambda: [])
        for n in sm.iter_class_nodes():
            self.sm_classes[n.label].append(SMClass(sm, n.node_id))

    def c(self, class_uri: str) -> List[SMClass]:
        """
        Get a list of classes defined in the semantic model by the URI.
        """
        return self.sm_classes[class_uri]

    # noinspection PyMethodMayBeStatic
    def ns(self, uri: str) -> Namespace:
        return Namespace(uri)


# 51 tan thanh
# 40/3 duong bau cat 1