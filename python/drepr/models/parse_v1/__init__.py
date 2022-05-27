from collections import defaultdict
from dataclasses import asdict
from typing import TYPE_CHECKING, List

from drepr.models.parse_v1.path_parser import PathParserV1
from drepr.utils.validator import *
from .align_parser import AlignParser
from .attr_parser import AttrParser
from .preprocessing_parser import PreprocessingParser
from .resource_parser import ResourceParser
from .sm_parser import SMParser
from ..align import AlignmentType, RangeAlignment
from ..sm import SemanticModel, ClassNode, DataNode, LiteralNode

if TYPE_CHECKING:
    from ..drepr import DRepr


class ReprV1Parser:
    """
    The DREPR language version 1 has the following schema:

    ```
    version: '1'
    resources: <resources>
    [preprocessing]: <preprocessing> (default is empty list)
    attributes: <attributes>
    [alignments]: <alignments> (default is empty list)
    semantic_model: <semantic_model>
    ```
    """

    TOP_KEYWORDS = {
        "version", "resources", "preprocessing", "attributes", "alignments", "semantic_model"
    }
    DEFAULT_RESOURCE_ID = "default"

    @classmethod
    def parse(cls, raw: dict):
        from ..drepr import DRepr

        Validator.must_be_subset(
            cls.TOP_KEYWORDS,
            raw.keys(),
            setname="Keys of D-REPR configuration",
            error_msg="Parsing D-REPR configuration")

        for prop in ['version', 'resources', 'attributes']:
            Validator.must_have(raw, prop, error_msg="Parsing D-REPR configuration")

        Validator.must_equal(raw['version'], '1', "Parsing D-REPR configuration version")
        resources = ResourceParser.parse(raw['resources'])

        if len(resources) == 1:
            default_resource_id = resources[0].id
        else:
            default_resource_id = ResourceParser.DEFAULT_RESOURCE_ID

        path_parser = PathParserV1()
        preprocessing = PreprocessingParser(path_parser).parse(default_resource_id, resources, raw.get('preprocessing', []))
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
        version = '1'
        sm = OrderedDict([("data_nodes", OrderedDict()), ("relations", []), ("literal_nodes", []),
                          ("subjects", OrderedDict([])), ("prefixes", drepr.sm.prefixes)])

        class_ids: Dict[str, Dict[str, str]] = defaultdict(lambda: {})
        for node in drepr.sm.nodes.values():
            if isinstance(node, ClassNode):
                class_ids[node.label][
                    node.node_id] = f"{node.label}:{len(class_ids[node.label]) + 1}"

        for node in drepr.sm.nodes.values():
            if isinstance(node, DataNode):
                edge = [e for e in drepr.sm.edges.values() if e.target_id == node.node_id][0]
                sm['data_nodes'][
                    node.
                    attr_id] = f"{class_ids[drepr.sm.nodes[edge.source_id].label][edge.source_id]}--{edge.label}"
                if node.data_type is not None:
                    sm['data_nodes'][node.attr_id] += f"^^{node.data_type.value}"

            if isinstance(node, LiteralNode):
                edge = [e for e in drepr.sm.edges if e.target_id == node.node_id][0]
                sm['literal_nodes'].append(
                    f"{class_ids[drepr.sm.nodes[edge.source_id].label][edge.source_id]}--{edge.label}--{node.value}"
                )
                if node.data_type is not None:
                    sm['literal_nodes'][-1] += f"^^{node.data_type.value}"

        for edge in drepr.sm.edges.values():
            if isinstance(drepr.sm.nodes[edge.source_id], ClassNode) and isinstance(
                    drepr.sm.nodes[edge.target_id], ClassNode):
                sm['relations'].append(
                    f"{class_ids[drepr.sm.nodes[edge.source_id].label][edge.source_id]}--{edge.label}--{class_ids[drepr.sm.nodes[edge.target_id].label][edge.target_id]}"
                )

            if edge.is_subject:
                sm['subjects'][class_ids[drepr.sm.nodes[edge.source_id].label][
                    edge.source_id]] = drepr.sm.nodes[edge.target_id].attr_id

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
