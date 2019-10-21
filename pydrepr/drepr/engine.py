from dataclasses import dataclass, asdict
from enum import Enum
from typing import Dict, Union

import ujson, traceback

# noinspection PyUnresolvedReferences
from drepr import drepr_engine
from drepr.version import __engine_version__
from drepr.models import DRepr, DEFAULT_RESOURCE_ID
from drepr.patches import xml_patch, jp_propname_patch

assert drepr_engine.__version__ == __engine_version__, f"You are using a different version of D-REPR" \
                                                          f": {drepr_engine.__version__}. The correct one is" \
                                                          f": {__engine_version__}"


def execute(ds_model: DRepr,
            resources: Union[str, Dict[str, str]],
            output: "Output",
            debug: bool = False):
    ptr = None

    if type(resources) is str:
        resources = {DEFAULT_RESOURCE_ID: resources}

    # ds_model = xml_patch.patch(ds_model, resources)
    # ds_model = jp_propname_patch.patch(ds_model, resources)

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
                {"file": resources[rid]}
                for rid in sorted(resources.keys(), key=lambda k: ds_model.resource_idmap[k])
            ],
            "output": engine_output,
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
