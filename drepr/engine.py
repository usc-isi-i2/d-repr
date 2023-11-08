import copy
from dataclasses import dataclass
from enum import Enum
from importlib.metadata import PackageNotFoundError
from importlib.resources import Resource
from pathlib import Path
from typing import Dict, List, Tuple, Union
from uuid import uuid4

import orjson

from drepr.core import Engine
from drepr.core import complete_description as rust_complete_description
from drepr.models import (
    DEFAULT_RESOURCE_ID,
    AlignedStep,
    Alignment,
    AlignmentType,
    ClassNode,
    DRepr,
    LiteralNode,
    RangeAlignment,
    ResourceData,
    ResourceDataFile,
    SemanticModel,
    ValueAlignment,
)
from drepr.models.resource import ResourceDataString
from drepr.patches import jp_propname_patch, static_class_patch, xml_patch

try:
    from drepr.patches import nc_patch
except ModuleNotFoundError:
    nc_patch = None


def execute(
    ds_model: DRepr,
    resources: Union[str, Dict[str, Union[str, ResourceData]]],
    output: "Output",
    debug: bool = False,
):
    if isinstance(resources, str):
        resources = {DEFAULT_RESOURCE_ID: resources}

    # normalize resources so that we know which one is from files and which one is from string. Below is the schema
    # resources = {
    #   <resource_id>: { "file"|"string": <value> }
    # }
    norm_resources = {
        rid: ResourceDataFile(resource) if isinstance(resource, str) else resource
        for rid, resource in resources.items()
    }

    if nc_patch is not None:
        ds_model = nc_patch.patch(ds_model, norm_resources)
    ds_model = xml_patch.patch(ds_model, norm_resources)
    ds_model = jp_propname_patch.patch(ds_model, norm_resources)
    ds_model, norm_resources = static_class_patch.patch(ds_model, norm_resources)

    if isinstance(output, FileOutput):
        engine_output = {
            "file": {"fpath": str(output.fpath), "format": output.format.value}
        }
    elif isinstance(output, MemoryOutput):
        engine_output = {"memory": {"format": output.format.value}}
    else:
        raise NotImplementedError()

    engine_model = ds_model.to_engine_format()
    if debug:
        tmpdir = Path(f"/tmp/d-repr-{str(uuid4())}")
        tmpdir.mkdir()

        print(f">>> the D-REPR model and data is stored in {str(tmpdir)}")

        (tmpdir / "model.yml").write_text(ds_model.to_lang_yml())
        for rid, resource in norm_resources.items():
            if isinstance(resource, ResourceDataFile):
                continue
            assert isinstance(resource, ResourceDataString)
            (tmpdir / f"{rid}.dat").write_bytes(
                resource.value
                if isinstance(resource.value, bytes)
                else resource.value.encode()
            )

    engine = Engine(
        orjson.dumps(
            {
                "resources": [
                    norm_resources[rid].to_dict()
                    for rid in sorted(
                        norm_resources.keys(),
                        key=lambda k: engine_model.resource_idmap[k],
                    )
                ],
                "output": engine_output,
                "edges_optional": engine_model.edges_optional,
                "description": engine_model.model,
            }
        )
    )
    if debug:
        print(
            f"""
******************************************
>>> Execution plan:
{engine.get_exec_plan()}
******************************************
"""
        )

    result = engine.run()
    if isinstance(output, MemoryOutput) and output.format == OutputFormat.GraphPy:
        class2nodes = {}
        for u in ds_model.sm.iter_class_nodes():
            class2nodes[u.node_id] = result[engine_model.sm_node_idmap[u.node_id]]
        return class2nodes
    return result


def complete_description(ds_model: DRepr) -> "CompleteDescription":
    patched_model = jp_propname_patch.patch(ds_model, {})
    # Note: do not patch the static class because we do post-update
    # description for the static class
    # patched_model, resources = static_class_patch.patch(ds_model, resources)

    engine_model = patched_model.to_engine_format()
    extra_info = rust_complete_description(orjson.dumps(engine_model.model))

    if ds_model is patched_model:
        sm = copy.deepcopy(ds_model.sm)
    else:
        sm = patched_model.sm

    # update subjects
    subjs = {}
    for n in sm.iter_class_nodes():
        class_id = engine_model.sm_node_idmap[n.node_id]
        aidx = extra_info["class2subj"][class_id]
        if aidx is None:
            # just need to pick a random literal node
            for e in sm.iter_outgoing_edges(n.node_id):
                if isinstance(sm.nodes[e.target_id], LiteralNode):
                    subjs[n.node_id] = e.target_id
                    # subjs.add((n.node_id, e.target_id))
                    break
        else:
            subjs[n.node_id] = f"dnode:{patched_model.attrs[aidx].id}"
            # subjs.add((n.node_id, f'dnode:{new_ds_model.attrs[aidx].id}'))

    for e in sm.edges.values():
        if subjs.get(e.source_id, None) == e.target_id:
            e.is_subject = True

    # update alignments
    alignments = dict()
    for align in patched_model.aligns:
        alignments[(f"dnode:{align.source}", f"dnode:{align.target}")] = [align]

    for (source, target), aligns_lst in extra_info["aligned_funcs"].items():
        source = f"dnode:{ds_model.attrs[source].id}"
        target = f"dnode:{ds_model.attrs[target].id}"
        aligns = []
        for align in aligns_lst:
            align_type = AlignmentType(align["type"])
            if align_type == AlignmentType.range:
                aligns.append(
                    RangeAlignment(
                        ds_model.attrs[align["source"]].id,
                        ds_model.attrs[align["target"]].id,
                        [AlignedStep(**o) for o in align["aligned_dims"]],
                    )
                )
            elif align_type == AlignmentType.value:
                aligns.append(
                    ValueAlignment(
                        ds_model.attrs[align["source"]].id,
                        ds_model.attrs[align["target"]].id,
                    )
                )
            else:
                raise NotImplementedError()
        alignments[source, target] = aligns

    for n in sm.iter_class_nodes():
        source = subjs[n.node_id]
        for e in sm.iter_outgoing_edges(n.node_id):
            v = sm.nodes[e.target_id]
            if isinstance(v, LiteralNode) and v.node_id != source:
                alignments[source, e.target_id] = [
                    RangeAlignment(source, e.target_id, [])
                ]
            elif (
                isinstance(v, ClassNode)
                and (source, subjs[e.target_id]) not in alignments
            ):
                # link to literal class
                assert isinstance(sm.nodes[subjs[e.target_id]], LiteralNode)
                alignments[source, subjs[e.target_id]] = [
                    RangeAlignment(source, subjs[e.target_id], [])
                ]

    return CompleteDescription(sm, alignments)


@dataclass
class CompleteDescription:
    sm: SemanticModel
    alignments: Dict[Tuple[str, str], List[Alignment]]


class OutputFormat(Enum):
    TTL = "ttl"
    GraphJSON = "graph_json"
    GraphPy = "graph_py"


@dataclass
class FileOutput:
    fpath: str
    format: OutputFormat


@dataclass
class MemoryOutput:
    format: OutputFormat


Output = Union[FileOutput, MemoryOutput]
