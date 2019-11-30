from dataclasses import dataclass, asdict
from enum import Enum
from typing import Dict, Union, Tuple, NamedTuple

import ujson, traceback

# noinspection PyUnresolvedReferences
from drepr import drepr_engine
from drepr.version import __engine_version__
from drepr.models import DRepr, DEFAULT_RESOURCE_ID
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
