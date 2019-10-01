from typing import List, Dict, Tuple, Callable, Any, Optional, Union

import ujson
import xmltodict, uuid
from drepr.models import DRepr


def patch(repr: DRepr, resources: Dict[str, str]) -> DRepr:
    """
    This patch will turn any XML resources to JSON using xmltodict
    """
    need_patch = False
    for rid, resource in repr.get_resources().items():
        if resource['type'] == 'xml':
            need_patch = True

    if need_patch:
        repr = repr.clone()
        for rid, resource in repr.get_resources().items():
            if resource['type'] == 'xml':
                with open(resources[rid], "r") as f:
                    doc = xmltodict.parse(f.read())

                resources[rid] = f"/tmp/{str(uuid.uuid4())}.json"
                resource['type'] = 'json'
                with open(resources[rid], "w") as f:
                    ujson.dump(doc, f)

    return repr