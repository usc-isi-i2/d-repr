from collections import defaultdict
from dataclasses import asdict
from typing import List

from drepr.models.parse_v2.path_parser import PathParserV2
from drepr.utils.validator import *
from ..align import AlignmentType, RangeAlignment

from ..parse_v1.align_parser import AlignParser
from ..parse_v1.attr_parser import AttrParser
from ..parse_v1.preprocessing_parser import PreprocessingParser
from ..parse_v1.resource_parser import ResourceParser

from .sm_parser import SMParser
from ..sm import SemanticModel, ClassNode, DataNode, LiteralNode


class ReprV2Parser:
    """
    The D-REPR language version 2 has similar to the schema of the first version.

    Difference with previous features:
    1. For spreadsheet columns, they can the letter instead of number
    2. Semantic model configuration is changed to focus on classes
    """

    TOP_KEYWORDS = {
        "version", "resources", "preprocessing", "attributes", "alignments", "semantic_model"
    }
    DEFAULT_RESOURCE_ID = "default"

    @classmethod
    def parse(cls, raw: dict):
        from ..drepr import DRepr

        Validator.must_be_subset(cls.TOP_KEYWORDS,
                                 raw.keys(),
                                 setname="Keys of D-REPR configuration",
                                 error_msg="Parsing D-REPR configuration")

        for prop in ['version', 'resources', 'attributes']:
            Validator.must_have(raw, prop, error_msg="Parsing D-REPR configuration")

        Validator.must_equal(raw['version'], '2', "Parsing D-REPR configuration version")
        resources = ResourceParser.parse(raw['resources'])

        if len(resources) == 1:
            default_resource_id = resources[0].id
        else:
            default_resource_id = ResourceParser.DEFAULT_RESOURCE_ID

        path_parser = PathParserV2()
        preprocessing = PreprocessingParser(path_parser).parse(default_resource_id, resources,
                                                               raw.get('preprocessing', []))
        attrs = AttrParser(path_parser).parse(default_resource_id, resources, raw['attributes'])
        aligns = AlignParser.parse(raw.get('alignments', []))

        if 'semantic_model' in raw:
            sm = SMParser.parse(raw['semantic_model'])
            sm.prefixes.update(SemanticModel.get_default_prefixes())
        else:
            sm = None

        return DRepr(resources, preprocessing, attrs, aligns, sm)

    @classmethod
    def dump(cls, drepr: 'DRepr', simplify: bool = True, use_json_path: bool = False):
        version = '2'
        sm = OrderedDict()

        class_counter = defaultdict(int)
        class_ids: Dict[str, str] = {}
        for node in drepr.sm.nodes.values():
            if isinstance(node, ClassNode):
                class_counter[node.label] += 1
                class_ids[node.node_id] = f"{node.label}:{class_counter[node.label]}"
                sm[class_ids[node.node_id]] = OrderedDict([
                    ("properties", []),
                    ("static_properties", []),
                    ("links", [])
                ])

        for node in drepr.sm.nodes.values():
            if isinstance(node, DataNode):
                edge = [e for e in drepr.sm.edges.values() if e.target_id == node.node_id][0]
                if node.data_type is not None:
                    prop = (edge.label, node.attr_id, node.data_type.value)
                else:
                    prop = (edge.label, node.attr_id)
                sm[class_ids[edge.source_id]]['properties'].append(prop)

            if isinstance(node, LiteralNode):
                edge = [e for e in drepr.sm.edges if e.target_id == node.node_id][0]
                if node.data_type is not None:
                    prop = (edge.label, node.value, node.data_type.value)
                else:
                    prop = (edge.label, node.value)
                sm[class_ids[edge.source_id]]['static_properties'].append(prop)

        for edge in drepr.sm.edges.values():
            if isinstance(drepr.sm.nodes[edge.source_id], ClassNode) and isinstance(
                    drepr.sm.nodes[edge.target_id], ClassNode):
                sm[class_ids[edge.source_id]]['links'].append((edge.label, class_ids[edge.target_id]))
            if edge.is_subject:
                sm[class_ids[edge.source_id]]['subject'] = drepr.sm.nodes[edge.target_id].attr_id

        sm['prefixes'] = drepr.sm.prefixes

        preprocessing: List[dict] = []
        for prepro in drepr.preprocessing:
            preprocessing.append(OrderedDict([("type", prepro.type.value)]))
            for k, v in asdict(prepro.value).items():
                preprocessing[-1][k] = v
            preprocessing[-1]["path"] = prepro.value.path.to_lang_format(use_json_path)

        return OrderedDict(
            [("version", version),
             ("resources",
              OrderedDict(
                  [(res.id,
                    OrderedDict([("type", res.type.value)] + (
                        [(k, v)
                         for k, v in asdict(res.prop).items()] if res.prop is not None else [])))
                   for res in drepr.resources])), ("preprocessing", preprocessing),
             ("attributes",
              OrderedDict([(attr.id,
                            OrderedDict([("resource_id", attr.resource_id),
                                         ("path", attr.path.to_lang_format(use_json_path)),
                                         ("unique", attr.unique), ("sorted", attr.sorted.value),
                                         ("value_type", attr.value_type.value),
                                         ("missing_values", attr.missing_values)]))
                           for attr in drepr.attrs])),
             ("alignments", [
                 OrderedDict([("type", AlignmentType.range.value), ("source", align.source),
                              ("target", align.target),
                              ("aligned_dims", [
                                  OrderedDict([
                                      ("source", step.source_idx),
                                      ("target", step.target_idx),
                                  ]) for step in align.aligned_steps
                              ])]) if isinstance(align, RangeAlignment) else
                 OrderedDict([("type", AlignmentType.value.value), ("source", align.source),
                              ("target", align.target)]) for align in drepr.aligns
             ]), ("semantic_model", sm)])