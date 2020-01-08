import copy
from dataclasses import dataclass, asdict
from enum import Enum
from typing import Dict, Union, Tuple, NamedTuple, List

import ujson, traceback

# noinspection PyUnresolvedReferences
from drepr import drepr_engine
from drepr.version import __engine_version__
from drepr.models import DRepr, DEFAULT_RESOURCE_ID, SemanticModel, Alignment, AlignmentType, RangeAlignment, \
    AlignedStep, ValueAlignment, LiteralNode, ClassNode
from drepr.patches import ResourceData, ResourceDataFile, ResourceDataString, xml_patch, jp_propname_patch, nc_patch

assert drepr_engine.__version__ == __engine_version__, f"You are using a different version of D-REPR" \
                                                          f": {drepr_engine.__version__}. The correct one is" \
                                                          f": {__engine_version__}"


def execute(ds_model: DRepr,
            resources: Union[str, Dict[str, Union[str, ResourceData]]],
            output: "Output",
            debug: bool = False):
    ptr = None

    if isinstance(resources, (str, tuple)):
        resources = {DEFAULT_RESOURCE_ID: resources}

    # normalize resources so that we know which one is from files and which one is from string. Below is the schema
    # resources = {
    #   <resource_id>: { "file"|"string": <value> }
    # }
    resources = {
        rid: ResourceDataFile(resource) if type(resource) is str else resource
        for rid, resource in resources.items()
    }

    ds_model = nc_patch.patch(ds_model, resources)
    ds_model = xml_patch.patch(ds_model, resources)
    ds_model = jp_propname_patch.patch(ds_model, resources)

    if isinstance(output, FileOutput):
        engine_output = {
            "file": {
                "fpath": str(output.fpath),
                "format": output.format.value
            }
        }
    elif isinstance(output, StringOutput):
        engine_output = {
            "string": {
                "format": output.format.value
            }
        }
    else:
        raise NotImplementedError()

    try:
        ds_model = ds_model.to_engine_format()
        if debug:
            print(">>> the engine is going to execute the below drepr model")
            print(ujson.dumps(ds_model.model, indent=4))

        ptr = drepr_engine.create_executor(ujson.dumps({
            "resources": [
                {"file": resources[rid].file_path} if isinstance(resources[rid], ResourceDataFile) else {"string": resources[rid].value}
                for rid in sorted(resources.keys(), key=lambda k: ds_model.resource_idmap[k])
            ],
            "output": engine_output,
            "edges_optional": ds_model.edges_optional,
            "description": ds_model.model
        }))
        if debug:
            print(f"""
******************************************
>>> Execution plan:
{drepr_engine.get_exec_plan(ptr)}
******************************************
""")

        results = drepr_engine.run_executor(ptr, ujson.dumps(engine_output))
        return results
    finally:
        if ptr is not None:
            drepr_engine.destroy_executor(ptr)


def complete_description(ds_model: DRepr) -> 'CompleteDescription':
    new_ds_model = jp_propname_patch.patch(ds_model, None)

    engine_ds_model = new_ds_model.to_engine_format()
    extra_info = drepr_engine.complete_description(ujson.dumps(engine_ds_model.model))

    if ds_model is new_ds_model:
        sm = copy.deepcopy(ds_model.sm)
    else:
        sm = new_ds_model.sm

    # update subjects
    subjs = {}
    for n in sm.iter_class_nodes():
        class_id = engine_ds_model.sm_node_idmap[n.node_id]
        aidx = extra_info['class2subj'][class_id]
        if aidx == -1:
            # just need to pick a random literal node
            for e in sm.iter_outgoing_edges(n.node_id):
                if isinstance(sm.nodes[e.target_id], LiteralNode):
                    subjs[n.node_id] = e.target_id
                    # subjs.add((n.node_id, e.target_id))
                    break
        else:
            subjs[n.node_id] = f'dnode:{new_ds_model.attrs[aidx].id}'
            # subjs.add((n.node_id, f'dnode:{new_ds_model.attrs[aidx].id}'))

    for e in sm.edges.values():
        if subjs.get(e.source_id, None) == e.target_id:
            e.is_subject = True

    # update alignments
    alignments = dict()
    for align in new_ds_model.aligns:
        alignments[(f"dnode:{align.source}", f"dnode:{align.target}")] = [align]

    for (source, target), aligns_lst in extra_info['aligned_funcs'].items():
        source = f"dnode:{ds_model.attrs[source].id}"
        target = f"dnode:{ds_model.attrs[target].id}"
        aligns = []
        for align in aligns_lst:
            align_type = AlignmentType(align['type'])
            if align_type == AlignmentType.range:
                aligns.append(RangeAlignment(
                    ds_model.attrs[align['source']].id,
                    ds_model.attrs[align['target']].id,
                    [AlignedStep(**o) for o in align['aligned_dims']]
                ))
            elif align_type == AlignmentType.value:
                aligns.append(ValueAlignment(
                    ds_model.attrs[align['source']].id,
                    ds_model.attrs[align['target']].id,
                ))
            else:
                raise NotImplementedError()
        alignments[source, target] = aligns

    for n in sm.iter_class_nodes():
        source = subjs[n.node_id]
        for e in sm.iter_outgoing_edges(n.node_id):
            v = sm.nodes[e.target_id]
            if isinstance(v, LiteralNode) and v.node_id != source:
                alignments[source, e.target_id] = [RangeAlignment(source, e.target_id, [])]
            elif isinstance(v, ClassNode) and (source, e.target_id) not in alignments:
                # link to literal class
                alignments[source, e.target_id] = [RangeAlignment(source, e.target_id, [])]

    return CompleteDescription(sm, alignments)


@dataclass
class CompleteDescription:
    sm: SemanticModel
    alignments: Dict[Tuple[str, str], List[Alignment]]


class OutputFormat(Enum):
    TTL = "ttl"
    GraphJSON = "graph_json"
    NDArray = "ndarray"


@dataclass
class FileOutput:
    fpath: str
    format: OutputFormat


@dataclass
class StringOutput:
    format: OutputFormat


Output = Union[FileOutput, StringOutput]
