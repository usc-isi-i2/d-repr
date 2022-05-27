#!/usr/bin/python
# -*- coding: utf-8 -*-
from typing import Dict, List

from api.misc.dict2instance import Dict2InstanceDeSer, DeSer, get_str2str_deser, get_str_deser, get_object_map_deser, \
    get_object_list_deser


class DataNode(Dict2InstanceDeSer):
    class_properties = {
        "node_id": get_str_deser("node_id"),
        "class_uri": get_str_deser("class_uri"),
        "predicate": get_str_deser("predicate"),
        "data_type": get_str_deser("data_type")
    }

    def __init__(self, node_id: str, class_uri: str, predicate: str, data_type: str):
        self.node_id = node_id
        self.class_uri = class_uri
        self.predicate = predicate
        self.data_type = data_type


class LiteralNode(Dict2InstanceDeSer):
    class_properties = {
        "node_id": get_str_deser("node_id"),
        "class_uri": get_str_deser("class_uri"),
        "predicate": get_str_deser("predicate"),
        "data": DeSer,
        "data_type": get_str_deser("data_type")
    }

    def __init__(self, node_id: str, class_uri: str, predicate: str, data, data_type: str):
        self.node_id = node_id
        self.class_uri = class_uri
        self.predicate = predicate
        self.data = data
        self.data_type = data_type


class Relation(Dict2InstanceDeSer):
    class_properties = {
        "source_id": get_str_deser("source_id"),
        "target_id": get_str_deser("target_id"),
        "predicate": get_str_deser("predicate")
    }

    def __init__(self, source_id: str, predicate: str, target_id: str):
        self.source_id = source_id
        self.predicate = predicate
        self.target_id = target_id


class SemanticModel(Dict2InstanceDeSer):
    class_properties = {
        "data_nodes": get_object_map_deser("data_nodes", DataNode),
        "literal_nodes": get_object_list_deser("literal_nodes", LiteralNode),
        "relations": get_object_list_deser("relations", Relation),
        "prefixes": get_str2str_deser("prefixes")
    }

    def __init__(self, data_nodes: Dict[str, DataNode], literal_nodes: List[LiteralNode], relations: List[Relation],
                 prefixes: Dict[str, str]):
        self.data_nodes = data_nodes
        self.literal_nodes = literal_nodes
        self.relations = relations
        self.prefixes = prefixes

    @staticmethod
    def default():
        return SemanticModel({}, [], [], {})

    def remove_data_node(self, dnode_id: str):
        dnode = self.data_nodes.pop(dnode_id)
        has_more_node = False
        for n in self.data_nodes.values():
            if n.class_uri == dnode.class_uri:
                has_more_node = True
                break

        if not has_more_node:
            for n in self.literal_nodes:
                if n.class_uri == dnode.class_uri:
                    has_more_node = True

        if not has_more_node:
            for i in range(len(self.relations) - 1, -1, -1):
                r = self.relations[i]
                if r.source_id == dnode.class_uri or r.target_id == dnode.class_uri:
                    self.relations.pop(i)

