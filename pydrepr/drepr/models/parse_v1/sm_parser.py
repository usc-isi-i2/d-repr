import re

from drepr.utils.validator import Validator, InputError
from ..sm import SemanticModel, ClassNode, DataNode, Edge, LiteralNode, DataType


class SMParser:
    """
    SM has the following schema

    ```
    semantic_model:
      data_nodes:
        <attr_id>: <class_id>--<predicate>[^^<semantic type>]
        # other attributes
      relations:
        - <source_class_id>--<predicate>--<target_class_id>
        # other relations
      literal_nodes:
        - <source_class_id>--<predicate>--<value>
      subjects:
        <class_id>: <attr_id>
      prefixes:
        <prefix>: <uri>
    ```
    """
    SM_KEYS = {"data_nodes", "subjects", "literal_nodes", "prefixes", "relations"}
    DATA_TYPE_VALUES = {x.value for x in DataType}

    REG_SM_CLASS = re.compile(r"^((.+):\d+)$")
    REG_SM_DNODE = re.compile(r"^((?:(?!--).)+:\d+)--((?:(?!\^\^).)+)(?:\^\^(.+))?$")
    REG_SM_LNODE = re.compile(
        r"^((?:(?!--).)+:\d+)--((?:(?!--).)+)--((?:(?!\^\^).)+)(?:\^\^(.+))?$")
    REG_SM_REL = re.compile(r"^((?:(?!--).)+:\d+)--((?:(?!--).)+)--((?:(?!--).)+:\d+)$")

    @classmethod
    def parse(cls, sm: dict) -> SemanticModel:
        Validator.must_be_subset(cls.SM_KEYS, sm.keys(), "properties of semantic model", "Parsing the semantic model")

        Validator.must_have(sm, "data_nodes", "Parsing the semantic model")
        trace0 = "Parsing `data_nodes` of the semantic model"
        Validator.must_be_dict(sm['data_nodes'], trace0)

        nodes = {}
        edges = {}

        for attr_id, stype in sm['data_nodes'].items():
            trace1 = f"{trace0}\nParsing data node `{attr_id}`"
            m = cls.REG_SM_DNODE.match(stype)
            if m is None:
                raise InputError(f"{trace1}\nERROR: the value of data node does not match with the format")

            # do something with the data node
            class_id = m.group(1)
            class_name = cls.REG_SM_CLASS.match(m.group(1)).group(2)
            predicate = m.group(2)
            data_type = m.group(3)

            if data_type is not None:
                Validator.must_in(data_type, cls.DATA_TYPE_VALUES, f"{trace1}\nParsing data type")
                data_type = DataType(data_type)
            else:
                data_type = None

            if class_id not in nodes:
                nodes[class_id] = ClassNode(node_id=class_id, label=class_name)

            data_node = DataNode(node_id=f"dnode:{attr_id}", attr_id=attr_id, data_type=data_type)
            nodes[data_node.node_id] = data_node
            edges[len(edges)] = Edge(len(edges), class_id, data_node.node_id, predicate)

        if 'relations' in sm:
            trace0 = f"Parsing `relations` of the semantic model"
            Validator.must_be_list(sm['relations'], trace0)
            for i, node in enumerate(sm['relations']):
                trace1 = f"{trace0}\nParsing relation at position {i}: {node}"
                Validator.must_be_str(node, trace1)
                m = cls.REG_SM_REL.match(node)
                if m is None:
                    raise InputError(f"{trace1}\nERROR: value of the relation does not match with the format")

                edges[len(edges)] = Edge(len(edges), source_id=m.group(1), target_id=m.group(3), label=m.group(2))

        if 'literal_nodes' in sm:
            trace0 = f"Parsing `literal_nodes` of the semantic model"
            Validator.must_be_list(sm['literal_nodes'], trace0)
            for i, node in enumerate(sm['literal_nodes']):
                trace1 = f"{trace0}\nParsing literal node at position {i}: {node}"
                Validator.must_be_str(node, trace1)
                m = cls.REG_SM_LNODE.match(node)
                if m is None:
                    raise InputError(f"{trace1}\nERROR: value of the literal node does not match with the format")

                class_id = m.group(1)
                class_name = cls.REG_SM_CLASS.match(m.group(1)).group(2)
                predicate = m.group(2)

                data_type = m.group(4)
                if data_type is not None:
                    Validator.must_in(data_type, cls.DATA_TYPE_VALUES, f"{trace1}\nParsing data type")
                data_type = DataType(data_type)

                if class_id not in nodes:
                    nodes[class_id] = ClassNode(node_id=class_id, label=class_name)

                literal_node = LiteralNode(
                    node_id=f"lnode:{i}",
                    value=m.group(3),
                    data_type=data_type)
                nodes[literal_node.node_id] = literal_node
                edges[len(edges)] = Edge(len(edges), source_id=class_id, target_id=literal_node.node_id, label=predicate)

        if 'prefixes' in sm:
            trace0 = f"Parsing `prefixes` of the semantic model"
            Validator.must_be_dict(sm['prefixes'], trace0)
            for prefix, uri in sm['prefixes'].items():
                Validator.must_be_str(uri, f"{trace0}\nParse prefix {prefix}")

            prefixes = dict(sm['prefixes'])
        else:
            prefixes = {}

        if 'subjects' in sm:
            trace0 = f"Parsing `subjects` of the semantic model"
            Validator.must_be_dict(sm['subjects'], trace0)
            for class_id, attr_id in sm['subjects'].items():
                Validator.must_be_str(attr_id, f"{trace0}\nParsing subject of class {class_id}")
                target_id = f"dnode:{attr_id}"
                for edge in edges.values():
                    if edge.source_id == class_id and edge.target_id == target_id:
                        edge.is_subject = True
                        break

        return SemanticModel(nodes, edges, prefixes)
