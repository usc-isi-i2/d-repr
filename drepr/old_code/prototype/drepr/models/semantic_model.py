#!/usr/bin/python
# -*- coding: utf-8 -*-
import re
from itertools import chain
from typing import *

import networkx as nx

from drepr.misc.class_helper import Equal
from drepr.misc.dict2instance import Dict2InstanceDeSer, get_object_map_deser, \
    get_object_list_deser, get_str2str_deser, DeSer, get_str_deser
from drepr.misc.ont_ns import KarmaOnt, OntNS
from drepr.exceptions import InvalidReprException


class ClassID(DeSer, Equal):
    """Represent id of a class node in the semantic model, which is a combine of URI and number"""

    def __init__(self, uri: str, number: int):
        self.uri = uri
        self.number = number
        self.id = f"{self.uri}:{self.number}"

    @staticmethod
    def create(class_id: str) -> 'ClassID':
        class_uri, number = class_id.rsplit(":", 1)
        return ClassID(class_uri, int(number))

    @staticmethod
    def serialize_func(self: "ClassID"):
        return self.id

    @classmethod
    def deserialize(cls, class_id: str) -> 'ClassID':
        return cls.create(class_id)

    @classmethod
    def unsafe_deserialize(cls, class_id: str) -> 'ClassID':
        return cls.create(class_id)

    def set_uri(self, new_uri: str):
        self.uri = new_uri
        self.id = f"{self.uri}:{self.number}"

    def __str__(self):
        return self.id


class SemanticType(Dict2InstanceDeSer, Equal):
    xsd_int = "xsd:int"
    xsd_decimal = "xsd:decimal"
    xsd_string = "xsd:string"
    xsd_dateTime = "xsd:dateTime"
    xsd_boolean = "xsd:boolean"
    xsd_any_uri = "xsd:anyURI"
    xsd_types = {xsd_int, xsd_string, xsd_dateTime, xsd_boolean, xsd_decimal, xsd_any_uri}

    class_properties = {
        "node_id": ClassID,
        "class_uri": get_str_deser("class_uri", InvalidReprException),
        "predicate_uri": get_str_deser("predicate_uri", InvalidReprException),
        "value_type": get_str_deser("value_type", InvalidReprException),
    }
    class_property_possible_values = {"value_type": xsd_types}

    def __init__(self, node_id: ClassID, class_uri: str, predicate_uri: str,
                 value_type: Optional[str]):
        self.node_id = node_id
        self.class_uri = class_uri
        self.predicate_uri = predicate_uri
        self.value_type = value_type  # value such as xsd:int, xsd:dateTime

    def set_class_uri(self, new_uri: str):
        self.class_uri = new_uri
        self.node_id.set_uri(new_uri)

    def set_predicate_uri(self, new_uri: str):
        self.predicate_uri = new_uri

    def is_uri(self) -> bool:
        """Test if this is a special URI property"""
        return self.predicate_uri == KarmaOnt.uri

    def __str__(self):
        return f"{self.node_id}--{self.predicate_uri}"


class SemanticRelation(Dict2InstanceDeSer, Equal):
    class_properties = {
        "source": ClassID,
        "predicate": get_str_deser("predicate", InvalidReprException),
        "target": ClassID
    }

    def __init__(self, source: ClassID, predicate: str, target: ClassID):
        self.source = source
        self.predicate = predicate
        self.target = target


class SemanticModel(Dict2InstanceDeSer, Equal):
    DeserializeErrorClass = InvalidReprException
    class_properties = {
        "semantic_types": get_object_map_deser("semantic_types", SemanticType),
        "semantic_relations": get_object_list_deser("semantic_relations", SemanticRelation),
        "ontology_prefixes": get_str2str_deser("ontology_prefixes", InvalidReprException)
    }
    class_rename_props = {"ontology_prefixes": "ont_prefixes"}
    eq_ignored_props = {"ont", "graph"}

    def __init__(self, semantic_types: Dict[str, SemanticType],
                 semantic_relations: List[SemanticRelation], ont_prefixes: Dict[str, str]):
        self.semantic_types = semantic_types
        self.semantic_relations = semantic_relations
        self.ont_prefixes = ont_prefixes
        self.ont = OntNS(ont_prefixes)
        self._init()

    def serialize(self):
        return self.serialize_func(self)

    def _init(self):
        self.graph = nx.DiGraph()
        for var_id, stype in self.semantic_types.items():
            self.graph.add_node(var_id, label=var_id, is_class_node=False)
            self.graph.add_node(str(stype.node_id), label=stype.class_uri, is_class_node=True)
            self.graph.add_edge(str(stype.node_id), var_id, label=stype.predicate_uri)

        for rel in self.semantic_relations:
            self.graph.add_node(str(rel.source), label=rel.source.uri, is_class_node=True)
            self.graph.add_node(str(rel.target), label=rel.target.uri, is_class_node=True)
            self.graph.add_edge(str(rel.source), str(rel.target), label=rel.predicate)

    def iter_class_nodes(self) -> Generator[str, None, None]:
        for nid, ndata in self.graph.nodes(data=True):
            if ndata['is_class_node']:
                yield nid

    def iter_data_nodes(self) -> Generator[str, None, None]:
        for nid, ndata in self.graph.nodes(data=True):
            if not ndata['is_class_node']:
                yield nid

    def get_predicate(self, source_id: str, target_id: str) -> str:
        return self.graph.edges[source_id, target_id]['label']

    def delete_data_node(self, dnode_id: str):
        delete_nodes = set()
        if dnode_id not in self.semantic_types:
            return

        self._upward_cascade_collect_removing_nodes(dnode_id, delete_nodes)
        for u in delete_nodes:
            for e in chain(self.graph.in_edges(u), self.graph.out_edges(u)):
                self.graph.remove_edge(*e)
            self.graph.remove_node(u)

        self.semantic_types.pop(dnode_id)
        self.semantic_relations = [
            srel for srel in self.semantic_relations
            if str(srel.source) not in delete_nodes and str(srel.target) not in delete_nodes
        ]

    def _upward_cascade_collect_removing_nodes(self, node_id: str, delete_nodes: Set[str]):
        delete_nodes.add(node_id)
        for u in self.graph.in_edges(node_id):
            if len(self.graph.out_edges(u)) == 1:
                self._upward_cascade_collect_removing_nodes(u, delete_nodes)
