import re

from drepr.utils.validator import Validator, InputError
from ..sm import SemanticModel, ClassNode, DataNode, Edge, LiteralNode, DataType


class SMParser:
    """
    SM has the following schema

    ```
    semantic_model:
      <class_id>:
        properties:
            - [<predicate>, <attr_id>, <sem_type>, <is_required=false>]
        links:
            - [<predicate>, <value>, <sem_type>]
        static_properties:
            -
      prefixes:
        <prefix>: <uri>
    ```
    """
    CLS_KEYS = {"properties", "subject", "links", "static_properties"}
    DATA_TYPE_VALUES = {x.value for x in DataType}

    REG_SM_CLASS = re.compile(r"^((.+):\d+)$")
    REG_SM_DNODE = re.compile(r"^((?:(?!--).)+:\d+)--((?:(?!\^\^).)+)(?:\^\^(.+))?$")
    REG_SM_LNODE = re.compile(
        r"^((?:(?!--).)+:\d+)--((?:(?!--).)+)--((?:(?!\^\^).)+)(?:\^\^(.+))?$")
    REG_SM_REL = re.compile(r"^((?:(?!--).)+:\d+)--((?:(?!--).)+)--((?:(?!--).)+:\d+)$")

    @classmethod
    def parse(cls, sm: dict) -> SemanticModel:
        nodes = {}
        edges = {}

        prefixes = sm.pop('prefixes', {})
        trace0 = f"Parsing `prefixes` of the semantic model"
        Validator.must_be_dict(prefixes, trace0)
        for prefix, uri in prefixes.items():
            Validator.must_be_str(uri, f"{trace0}\nParse prefix {prefix}")

        for class_id, class_conf in sm.items():
            trace0 = f"Parsing class `{class_id}` of the semantic model"
            Validator.must_be_dict(class_conf, trace0)
            Validator.must_be_subset(cls.CLS_KEYS, class_conf.keys(), "keys of ontology class",
                                     trace0)

            try:
                class_name = cls.REG_SM_CLASS.match(class_id).group(2)
            except Exception as e:
                raise InputError(
                    f"{trace0}\nERROR: invalid class_id `{class_id}`. Expect to be <string>:<number>"
                )

            nodes[class_id] = ClassNode(class_id, class_name)

        for class_id, class_conf in sm.items():
            trace0 = f"Parsing class `{class_id}` of the semantic model"

            for i, prop in enumerate(class_conf.get('properties', [])):
                trace1 = f"{trace0}\nParsing property {i}: {prop}"
                if len(prop) == 2:
                    predicate, attr_id = prop
                    data_type = None
                    is_required = False
                elif len(prop) == 3:
                    predicate, attr_id, data_type = prop
                    if isinstance(data_type, bool) or data_type.lower() in {"true", "false"}:
                        is_required = data_type if isinstance(data_type,
                                                              bool) else data_type == "true"
                        data_type = None
                    else:
                        is_required = False
                elif len(prop) == 4:
                    predicate, attr_id, data_type, is_required = prop
                else:
                    raise InputError(
                        f"{trace1}\nERROR: Expect value of the property to be an array of two "
                        f"three or four items (<predicate>, <attribute_id>[, semantic_type='auto'][, is_required=false])"
                    )

                if data_type is not None:
                    Validator.must_in(data_type, cls.DATA_TYPE_VALUES,
                                      f"{trace1}\nParsing data type")
                    data_type = DataType(data_type)

                node = DataNode(node_id=f"dnode:{attr_id}", attr_id=attr_id, data_type=data_type)
                nodes[node.node_id] = node
                edges[len(edges)] = Edge(len(edges), class_id, node.node_id, predicate, is_required=is_required)

            for i, link_conf in enumerate(class_conf.get('links', [])):
                trace1 = f"{trace0}\nParsing link {i}: {link_conf}"
                if len(link_conf) != 2:
                    raise InputError(
                        f"{trace1}\nERROR: Expect value of the link to be an array of two "
                        f"items (<predicate>, <class_id>)")
                predicate, object_class_id = link_conf
                edges[len(edges)] = Edge(len(edges), class_id, object_class_id, predicate)

            for i, prop in enumerate(class_conf.get('static_properties', [])):
                trace1 = f"{trace0}\nParsing static properties {i}: {prop}"
                if len(prop) == 2:
                    predicate, value = prop
                    data_type = None
                elif len(prop) == 3:
                    predicate, value, data_type = prop
                else:
                    raise InputError(
                        f"{trace1}\nERROR: Expect value of the property to be an array of two "
                        f"or three items (<predicate>, <attribute_id>, [semantic_type])")

                if data_type is not None:
                    Validator.must_in(data_type, cls.DATA_TYPE_VALUES,
                                      f"{trace1}\nParsing data type")
                    data_type = DataType(data_type)

                node = LiteralNode(node_id=f"lnode:{len(nodes)}", value=value, data_type=data_type)
                nodes[node.node_id] = node
                edges[len(edges)] = Edge(len(edges), class_id, node.node_id, predicate)

        for class_id, class_conf in sm.items():
            trace0 = f"Parsing class `{class_id}` of the semantic model"
            if 'subject' in class_conf:
                trace1 = f"{trace0}\nParsing subject"
                attr_id = class_conf['subject']
                target_id = f"dnode:{attr_id}"
                for edge in edges.values():
                    if edge.source_id == class_id and edge.target_id == target_id:
                        edge.is_subject = True
                        break
                else:
                    raise InputError(f"{trace1}\nERROR: Subject of the class node must be one "
                                     f"of the attributes used in the semantic model")

        return SemanticModel(nodes, edges, prefixes)
