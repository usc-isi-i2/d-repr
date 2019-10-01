#!/usr/bin/python
# -*- coding: utf-8 -*-

from typing import *
from collections import OrderedDict, defaultdict
from api.misc.dict2instance import Dict2InstanceDeSer, get_object_map_deser, get_object_list_deser, DeSer
from api.models.repr.location import Location
from api.models.repr.alignments import AlignmentDeSer, AlignmentEnum, ValueAlignment, DimAlignment
from api.models.repr.ext_resource import ExtResource
from api.models.repr.preprocessing import PreprocessingDeSer, PreprocessingFunc, PMap, PFilter
from api.models.repr.resources import ResourceDeSer, ResourceEnum, CSVResource, JSONResource
from api.models.repr.semantic_model import SemanticModel, Relation, DataNode, LiteralNode
from api.models.repr.variables import Variable
from drepr import DRepr, models

if TYPE_CHECKING:
    from api.models.resource import Resource


class Representation(Dict2InstanceDeSer):
    """An extension of the representation in order to keep track of resources"""
    class_properties = {
        "ext_resources": get_object_map_deser("ext_resources", ExtResource),
        "resources": get_object_map_deser("resources", ResourceDeSer),
        "preprocessing": get_object_list_deser("preprocessing", PreprocessingDeSer),
        "variables": get_object_map_deser("variables", Variable),
        "alignments": get_object_list_deser("alignments", AlignmentDeSer),
        "semantic_model": SemanticModel
    }

    def __init__(self, ext_resources: Dict[str, ExtResource], resources: Dict[str, ResourceEnum],
                 preprocessing: List[PreprocessingFunc], variables: Dict[str, Variable],
                 alignments: List[AlignmentEnum], semantic_model: SemanticModel):
        self.resources = resources
        self.preprocessing = preprocessing
        self.variables = variables
        self.alignments = alignments
        self.semantic_model = semantic_model
        self.ext_resources = ext_resources

    @staticmethod
    def default():
        return Representation({}, {}, [], {}, [], SemanticModel.default())

    @staticmethod
    def from_repr(ds_model: DRepr, ext_resources: Dict[str, ExtResource]):
        repr = Representation(ext_resources, {}, [], {}, [], SemanticModel.default())

        for res in ds_model.resources:
            if res.type == models.ResourceType.CSV:
                repr.resources[res.id] = CSVResource(
                    res.id, res.prop.delimiter if res.prop.delimiter is not None else None)
            elif res.type == models.ResourceType.JSON:
                repr.resources[res.id] = JSONResource(res.id)
            else:
                raise NotImplementedError()

        for prepro in ds_model.preprocessing:
            if prepro.type == models.PreprocessingType.pmap:
                repr.preprocessing.append(
                    PMap(
                        Location.from_path(prepro.value.resource_id, prepro.value.path),
                        prepro.value.output, prepro.value.code))
            elif prepro.type == models.PreprocessingType.pfilter:
                repr.preprocessing.append(
                    PFilter(
                        Location.from_path(prepro.value.resource_id, prepro.value.path),
                        prepro.value.output, prepro.value.code))
            else:
                raise NotImplementedError()

        for attr in ds_model.attrs:
            repr.variables[attr.id] = Variable(attr.id,
                                               Location.from_path(attr.resource_id,
                                                                  attr.path), attr.unique,
                                               attr.sorted.value, attr.value_type.value,
                                               set(attr.missing_values))

        for align in ds_model.aligns:
            if isinstance(align, models.RangeAlignment):
                repr.alignments.append(
                    DimAlignment(align.source, align.target, [{
                        "source": ad.source_idx,
                        "target": ad.target_idx
                    } for ad in align.aligned_steps]))
            elif isinstance(align, models.ValueAlignment):
                repr.alignments.append(ValueAlignment(align.source, align.target))
            else:
                raise NotImplementedError()

        repr.semantic_model.prefixes = ds_model.sm.prefixes
        # for generating class id in the semantic model
        class_ids = defaultdict(lambda: {})

        for edge in ds_model.sm.edges:
            source = ds_model.sm.nodes[edge.source_id]
            target = ds_model.sm.nodes[edge.target_id]

            if isinstance(source, models.ClassNode):
                if source.node_id not in class_ids[source.label]:
                    class_ids[source.label][
                        source.node_id] = f"{source.label}:{len(class_ids[source.label])}"

                source_id = class_ids[source.label][source.node_id]
                if isinstance(target, models.ClassNode):
                    if target.node_id not in class_ids[target.label]:
                        class_ids[target.label][
                            target.node_id] = f"{target.label}:{len(class_ids[target.label])}"

                    target_id = class_ids[target.label][target.node_id]
                    repr.semantic_model.relations.append(Relation(source_id, edge.label, target_id))
                elif isinstance(target, models.DataNode):
                    repr.semantic_model.data_nodes[target.attr_id] = DataNode(
                        source_id, source.label, edge.label,
                        target.data_type.value if target.data_type is not None else None)
                elif isinstance(target, models.LiteralNode):
                    repr.semantic_model.literal_nodes.append(
                        LiteralNode(
                            source_id, source.label, edge.label, target.value,
                            target.data_type.value if target.data_type is not None else None))
                else:
                    raise NotImplementedError()
            else:
                assert False
        return repr

    @staticmethod
    def to_repr(wrepr: dict) -> DRepr:
        repr: Representation = Representation.deserialize(wrepr)
        ds_model = DRepr.empty()

        for res in repr.resources.values():
            if isinstance(res, CSVResource):
                ds_model.resources.append(
                    models.Resource(res.id, models.ResourceType.CSV, models.CSVProp(res.delimiter)))
            elif isinstance(res, JSONResource):
                ds_model.resources.append(models.Resource(res.id, models.ResourceType.JSON, None))
            else:
                raise NotImplementedError()

        for prepro in repr.preprocessing:
            if isinstance(prepro, PMap):
                ds_model.preprocessing.append(models.Preprocessing(
                    models.PreprocessingType.pmap,
                    models.PMap(prepro.input.resource_id, prepro.input.get_path(),
                                prepro.code, prepro.output, None)
                ))
            elif isinstance(prepro, PFilter):
                ds_model.preprocessing.append(models.Preprocessing(
                    models.PreprocessingType.pfilter,
                    models.PFilter(
                    prepro.input.resource_id, prepro.input.get_path(),
                    prepro.code, prepro.output
                )))
            else:
                raise NotImplementedError()

        for attr in repr.variables.values():
            ds_model.attrs.append(models.Attr(
                attr.id, attr.location.resource_id,
                attr.location.get_path(), list(attr.missing_values),
                attr.unique, models.Sorted(attr.sorted), models.ValueType(attr.value_type)
            ))

        for align in repr.alignments:
            if isinstance(align, DimAlignment):
                ds_model.aligns.append(models.RangeAlignment(align.source, align.target, [
                    models.AlignedStep(s['source'], s['target'])
                    for s in align.aligned_dims
                ]))
            elif isinstance(align, ValueAlignment):
                ds_model.aligns.append(models.ValueAlignment(align.source, align.target))
            else:
                raise NotImplementedError()

        for attr_id, node in repr.semantic_model.data_nodes.items():
            dnode_id = f"dnode:{attr_id}"
            ds_model.sm.nodes[dnode_id] = models.DataNode(
                dnode_id, attr_id,
                models.DataType(node.data_type) if node.data_type is not None else None)

            if node.node_id not in ds_model.sm.nodes:
                ds_model.sm.nodes[node.node_id] = models.ClassNode(node.node_id, node.class_uri)
            ds_model.sm.edges.append(models.Edge(node.node_id, dnode_id, node.predicate))

        for i, node in enumerate(repr.semantic_model.literal_nodes):
            lnode_id = f"lnode:{i}"
            ds_model.sm.nodes[lnode_id] = models.LiteralNode(
                lnode_id, node.data,
                models.DataType(node.data_type) if node.data_type is not None else None)

            if node.node_id not in ds_model.sm.nodes:
                ds_model.sm.nodes[node.node_id] = models.ClassNode(node.node_id, node.class_uri)
            ds_model.sm.edges.append(models.Edge(node.node_id, lnode_id, node.predicate))

        for rel in repr.semantic_model.relations:
            ds_model.sm.edges.append(models.Edge(rel.source_id, rel.target_id, rel.predicate))

        ds_model.sm.prefixes = repr.semantic_model.prefixes
        return ds_model

    @staticmethod
    def unsafe_get_resource_db_id(raw_wrepr: dict, resource_id: str) -> int:
        return raw_wrepr['ext_resources'][resource_id]['resource_db_id']

    def get_resource_db_id(self, resource_id: str) -> int:
        return self.ext_resources[resource_id].resource_db_id

    def add_resource(self, record: 'Resource'):
        self.ext_resources[record.resource_id] = ExtResource.from_resource(record)
        r = {'type': record.resource['type'], **record.resource}
        self.resources[record.resource_id] = ResourceDeSer.deserialize(r)

    def has_resource(self, resource_id: str):
        return resource_id in self.ext_resources

    def remove_resource(self, resource_id: str):
        self.ext_resources.pop(resource_id)
        self.resources.pop(resource_id)

        for vid in list(self.variables.keys()):
            var = self.variables[vid]
            if var.location.resource_id == resource_id:
                self.remove_variable(vid)

    def has_variable(self, variable_id: str):
        return variable_id in self.variables

    def add_variable(self, variable: Variable):
        self.variables[variable.id] = variable

    def remove_variable(self, variable_id: str):
        self.variables.pop(variable_id)
        for i in range(len(self.alignments) - 1, -1, -1):
            align = self.alignments[i]
            if variable_id == align.source or variable_id == align.target:
                self.alignments.pop(i)

        if variable_id in self.semantic_model.data_nodes:
            self.semantic_model.remove_data_node(variable_id)

    def replace_variable(self, prev_var_id: str, var: Variable):
        self.variables.pop(prev_var_id)
        self.variables[var.id] = var

        for align in self.alignments:
            if prev_var_id == align.source:
                align.source = var.id
            elif prev_var_id == align.target:
                align.target = var.id

        if prev_var_id in self.semantic_model.data_nodes:
            self.semantic_model.data_nodes[prev_var_id] = self.semantic_model.data_nodes.pop(
                prev_var_id)

    def set_alignments(self, alignments: List[AlignmentEnum]):
        self.alignments = alignments

    def set_semantic_model(self, sm: SemanticModel):
        self.semantic_model = sm
