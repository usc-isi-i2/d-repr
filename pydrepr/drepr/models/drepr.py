from collections import OrderedDict, defaultdict
from dataclasses import dataclass, asdict
from enum import Enum
from io import StringIO
from typing import List, Dict, Any, NamedTuple, Optional

import ujson
from ruamel.yaml import YAML

from drepr.models.parse_v2 import ReprV2Parser
from drepr.utils.validator import Validator, InputError
from .align import Alignment, RangeAlignment, AlignmentType, ValueAlignment, AlignedStep
from .attr import Attr
from .parse_v1 import ReprV1Parser
from .preprocessing import Preprocessing, PMap, PFilter, RMap, PSplit
from .resource import Resource, CSVProp
from .sm import SemanticModel, DataNode, ClassNode, LiteralNode

yaml = YAML()
yaml.Representer.add_representer(OrderedDict, yaml.Representer.represent_dict)


@dataclass
class EngineFormat:
    model: Dict[str, Any]
    edges_optional: List[bool]
    resource_idmap: Dict[str, int]
    attribute_idmap: Dict[str, int]
    sm_node_idmap: Dict[str, int]


@dataclass
class DRepr:
    resources: List[Resource]
    preprocessing: List[Preprocessing]
    attrs: List[Attr]
    aligns: List[Alignment]
    sm: SemanticModel

    @staticmethod
    def parse(raw: dict) -> "DRepr":
        Validator.must_have(raw, 'version', "Parsing D-REPR configuration")
        if raw['version'] == '1':
            model = ReprV1Parser.parse(raw)
            model.is_valid()
            return model
        elif raw['version'] == '2':
            model = ReprV2Parser.parse(raw)
            model.is_valid()
            return model
        raise InputError(f"Parsing error, get unknown version: {raw['version']}")

    @staticmethod
    def parse_from_file(fpath: str) -> "DRepr":
        if fpath.endswith(".json"):
            with open(fpath, "r") as f:
                return DRepr.parse(ujson.load(f))

        if fpath.endswith(".yml") or fpath.endswith(".yaml"):
            with open(fpath, 'r') as f:
                return DRepr.parse(yaml.load(f))

        raise Exception(f"Does not supported this file: {fpath}. Only support json or yaml file")

    @staticmethod
    def empty() -> "DRepr":
        return DRepr([], [], [], [], SemanticModel({}, [], {}))

    @staticmethod
    def deserialize(raw: dict) -> "DRepr":
        resources = [Resource.deserialize(o) for o in raw['resources']]
        preprocessing = [Preprocessing.deserialize(o) for o in raw['preprocessing']]
        attrs = [Attr.deserialize(o) for o in raw['attrs']]
        aligns = []

        for align in raw['aligns']:
            if align['type'] == AlignmentType.range.value:
                aligns.append(
                    RangeAlignment(align['source'], align['target'], [
                        AlignedStep(step['source_idx'], step['target_idx'])
                        for step in align['aligned_steps']
                    ]))
            elif align['type'] == AlignmentType.value.value:
                aligns.append(ValueAlignment(align['source'], align['target']))
            else:
                raise NotImplementedError()
        sm = SemanticModel.deserialize(raw['sm'])

        return DRepr(resources, preprocessing, attrs, aligns, sm)

    def serialize(self) -> dict:
        obj = asdict(self)
        # post-process the enum
        for res in obj['resources']:
            res['type'] = res['type'].value
        for prepro in obj['preprocessing']:
            prepro['type'] = prepro['type'].value
            for i, step in enumerate(prepro['value']['path']['steps']):
                if isinstance(step, Enum):
                    prepro['value']['path']['steps'][i] = step.value

        for attr in obj['attrs']:
            attr['sorted'] = attr['sorted'].value
            for i, step in enumerate(attr['path']['steps']):
                if isinstance(step, Enum):
                    attr['steps'][i] = step.value

            attr['value_type'] = attr['value_type'].value
        for node in obj['sm']['nodes'].values():
            if node.get('data_type', None) is not None:
                node['data_type'] = node['data_type'].value

        # adding a bit of meta-data about the alignment
        for align, raw_align in zip(self.aligns, obj['aligns']):
            if isinstance(align, RangeAlignment):
                raw_align['type'] = AlignmentType.range.value
            elif isinstance(align, ValueAlignment):
                raw_align['type'] = AlignmentType.value.value
            else:
                raise NotImplementedError()

        # similarly, add meta-data about the nodes
        for node in obj['sm']['nodes'].values():
            if isinstance(self.sm.nodes[node['node_id']], ClassNode):
                node['type'] = 'class_node'
            elif isinstance(self.sm.nodes[node['node_id']], DataNode):
                node['type'] = 'data_node'
            elif isinstance(self.sm.nodes[node['node_id']], LiteralNode):
                node['type'] = 'literal_node'
            else:
                raise NotImplementedError()

        return obj

    def is_valid(self):
        """
        Perform a check to see if this D-REPR is valid. Raise AssertionError if this is not valid
        """
        # CHECK 1: all references (resource id, attribute ids) are valid
        resource_ids = {r.id for r in self.resources}
        for pref in self.preprocessing:
            if pref.value.output is not None:
                # preprocessing create new resource
                assert pref.value.output not in resource_ids, "Cannot overwrite existing resources"
                resource_ids.add(pref.value.output)
        attr_ids = {attr.id for attr in self.attrs}

        for attr in self.attrs:
            assert attr.resource_id in resource_ids, f"Attribute {attr.resource_id} does not belong to any resources"
        for align in self.aligns:
            assert align.source in attr_ids and align.target in attr_ids, \
                f"The alignment {align} links to non-existence attributes"

        for node in self.sm.nodes.values():
            if isinstance(node, DataNode):
                assert node.attr_id in attr_ids, f"The semantic model has a link to " \
                                                 f"a non-existence attribute: {node.attr_id}"

        # CHECK 2: check class and predicates are valid
        for node in self.sm.nodes.values():
            if isinstance(node, ClassNode):
                if self.sm.is_rel_iri(node.label):
                    prefix = node.label.split(":", 1)[0]
                    assert prefix in self.sm.prefixes, f"Unknown prefix `{prefix}` of the " \
                                                                            f"ontology class {node.label}"
        for edge in self.sm.edges.values():
            if self.sm.is_rel_iri(edge.label):
                prefix = edge.label.split(":", 1)[0]
                assert prefix in self.sm.prefixes, f"Unknown prefix `{prefix}` of the " \
                                                                        f"ontology predicate {edge.label}"

    def to_lang_format(self, simplify: bool = True, use_json_path: bool = False) -> dict:
        return ReprV1Parser.dump(self, simplify, use_json_path)

    def to_lang_yml(self, simplify: bool = True, use_json_path: bool = False) -> str:
        model = self.to_lang_format(simplify, use_json_path)
        out = StringIO()
        yaml.dump(model, out)
        return out.getvalue()

    def to_engine_format(self) -> EngineFormat:
        """
        Turn this D-REPR configuration into the format that the engine can read
        :return:
        """
        # map string id to incremental numbers (for resource id and attribute id)
        ridmap = OrderedDict()
        for resource in self.resources:
            ridmap[resource.id] = len(ridmap)
        for pref in self.preprocessing:
            if pref.value.output is not None:
                ridmap[pref.value.output] = len(ridmap)
        aidmap = OrderedDict()
        for attr in self.attrs:
            aidmap[attr.id] = len(aidmap)

        resources = []
        for res in self.resources:
            resources.append({"type": res.type.value})
            if isinstance(res.prop, CSVProp):
                resources[-1]['value'] = {
                    "resource_id": ridmap[res.id],
                    "delimiter": res.prop.delimiter
                }
            else:
                resources[-1]['value'] = ridmap[res.id]

        preprocessing = []
        for pref in self.preprocessing:
            prepro = {
                "type": pref.type.value,
                "resource_id": ridmap[pref.value.resource_id],
                "path": pref.value.path.to_engine_format(),
                "output": ridmap[pref.value.output] if pref.value.output is not None else None,
            }
            if isinstance(pref.value, PMap):
                prepro['code'] = pref.value.code
                prepro['change_structure'] = pref.value.change_structure
            elif isinstance(pref.value, (PFilter, PSplit)):
                prepro['code'] = pref.value.code
            elif isinstance(pref.value, RMap):
                prepro['func_id'] = {"t": pref.value.func_id.value}
            else:
                raise NotImplementedError()
            preprocessing.append(prepro)

        attributes = [{
            "id": aidmap[a.id],
            "resource_id": ridmap[a.resource_id],
            "path": a.path.to_engine_format(),
            "unique": a.unique,
            "sorted": a.sorted.value,
            "vtype": a.value_type.value,
            "missing_values": [self._serde_engine_value(v) for v in a.missing_values]
        } for a in self.attrs]

        alignments = []
        for align in self.aligns:
            if isinstance(align, RangeAlignment):
                alignments.append({
                    "type": AlignmentType.range.value,
                    "source": aidmap[align.source],
                    "target": aidmap[align.target],
                    "aligned_dims": [{
                        "source": ad.source_idx,
                        "target": ad.target_idx
                    } for ad in align.aligned_steps]
                })
            elif isinstance(align, ValueAlignment):
                alignments.append({
                    "type": AlignmentType.value.value,
                    "source": aidmap[align.source],
                    "target": aidmap[align.target]
                })
            else:
                raise NotImplementedError()

        engine_sm: Dict[str, Any] = {
            "nodes": [],
            "edges": [],
            "prefixes": [(prefix, uri) for prefix, uri in self.sm.prefixes.items()]
        }
        nodes = {"cnodes": [], "dnodes": [], "lnodes": []}
        for node_id, node in self.sm.nodes.items():
            if isinstance(node, ClassNode):
                nodes['cnodes'].append({
                    "type": "class_node",
                    "node_id": node_id,
                    "rel_label": node.label,
                    "abs_label": self.sm.get_abs_iri(node.label)
                })
            elif isinstance(node, DataNode):
                nodes['dnodes'].append({
                    "type": "data_node",
                    "node_id": node_id,
                    "attr_id": aidmap[node.attr_id],
                    "data_type": node.data_type.value if node.data_type is not None else None,
                })
            elif isinstance(node, LiteralNode):
                nodes["lnodes"].append({
                    "type": "literal_node",
                    "node_id": node_id,
                    "val": self._serde_engine_value(node.value),
                    "data_type": node.data_type.value if node.data_type is not None else None,
                })
            else:
                raise NotImplementedError()

        for k in ["cnodes", "dnodes", "lnodes"]:
            for n in nodes[k]:
                engine_sm['nodes'].append(n)
        nidmap = {}
        for i, n in enumerate(engine_sm["nodes"]):
            nidmap[n['node_id']] = i
            n['node_id'] = i

        for eid, edge in self.sm.edges.items():
            engine_sm['edges'].append({
                "edge_id": eid,
                "source": nidmap[edge.source_id],
                "target": nidmap[edge.target_id],
                "rel_label": edge.label,
                "abs_label": self.sm.get_abs_iri(edge.label),
                "is_subject": edge.is_subject
            })

        edges_optional = [
            not edge.is_required
            for edge in self.sm.edges.values()
        ]

        return EngineFormat({
            "resources": resources,
            "preprocessing": preprocessing,
            "attributes": attributes,
            "alignments": alignments,
            "semantic_model": engine_sm
        }, edges_optional, ridmap, aidmap, nidmap)

    def _serde_engine_value(self, value: Any):
        """Serialize a python value to a json representation of the Value struct in the Rust engine"""
        if value is None:
            return {"t": "Null"}
        elif isinstance(value, bool):
            return {"t": "Bool", "c": value}
        elif isinstance(value, int):
            return {"t": "I64", "c": value}
        elif isinstance(value, float):
            return {"t": "F64", "c": value}
        elif isinstance(value, str):
            return {"t": "Str", "c": value}
        elif isinstance(value, list):
            return {"t": "Array", "c": [self._serde_engine_value(v) for v in value]}
        elif isinstance(value, (dict, OrderedDict)):
            return {"t": "Object", "c": {
                k: self._serde_engine_value(v)
                for k, v in value.items()
            }}
        else:
            raise InputError(f"Cannot serialize the value of type: {type(value)} to JSON")

    def remove_resource(self, resource_id: str):
        self.resources = [r for r in self.resources if r.id != resource_id]
        for i in range(len(self.preprocessing) - 1, -1, -1):
            if self.preprocessing[i].value.resource_id == resource_id:
                self.preprocessing.pop(i)

        for i in range(len(self.attrs) - 1, -1, -1):
            if self.attrs[i].resource_id == resource_id:
                self.remove_attribute(self.attrs[i].id, idx=i)

    def get_resource_by_id(self, resource_id: str) -> Optional[Resource]:
        for r in self.resources:
            if r.id == resource_id:
                return r
        return None

    def get_attr_by_id(self, attr_id: str) -> Optional[Attr]:
        for a in self.attrs:
            if a.id == attr_id:
                return a
        return None

    def remove_attribute(self, attr_id: str, idx: Optional[int] = None):
        if idx is None:
            idx = next(i for i in range(len(self.attrs), -1, -1) if self.attrs[i].id == attr_id)

        self.attrs.pop(idx)
        for i in range(len(self.aligns) - 1, -1, -1):
            if self.aligns[i].source == attr_id or self.aligns[i].target == attr_id:
                self.aligns.pop(i)

        for node in self.sm.nodes:
            if isinstance(node, DataNode) and node.attr_id == attr_id:
                self.sm.remove_node(node.node_id)

    def update_attribute(self, attr_id: str, new_attr: Attr):
        for i, attr in enumerate(self.attrs):
            if attr.id == attr_id:
                self.attrs[i] = new_attr

        for align in self.aligns:
            if align.source == attr_id:
                align.source = new_attr.id
            elif align.target == attr_id:
                align.target = new_attr.id

        for node in self.sm.nodes:
            if isinstance(node, DataNode) and node.attr_id == attr_id:
                node.attr_id = new_attr.id
