from collections import Counter
from pathlib import Path

import numpy as np
from PIL import Image
from PIL.TiffTags import TAGS

from drepr.models import DRepr, ClassNode, DataNode, RangeExpr, LiteralNode
from drepr.ndarray import NDArray


def map_geotiff(ds_model: DRepr, resource_file: str):
    # http://geotiff.maptools.org/spec/geotiff2.6.html
    with Image.open(resource_file) as img:
        # read metadata
        metadata = {TAGS[key]: img.tag[key] for key in img.tag.keys()}

        # set other metadata
        _path_resource_file = Path(resource_file)
        metadata["filename"] = _path_resource_file.name
        metadata["filename_stem"] = _path_resource_file.stem

        # read data
        data = np.asarray(img)

    attrs = {}
    for attr in ds_model.attrs:
        is_meta = isinstance(attr.path.steps[0], str) and attr.path.steps[0][0] == "@"
        if is_meta:
            attrs[attr.id] = metadata[attr.path.steps[0]]
        else:
            data_ptr = data
            for step in attr.path.steps:
                assert isinstance(step, RangeExpr)
                data_ptr = data_ptr[step.start:step.end:step.step]
            attrs[attr.id] = data_ptr

    sm = ds_model.sm
    tables = {}
    relations = {}

    for nid, node in sm.nodes.items():
        if isinstance(node, ClassNode):
            outgoing_edges = list(sm.iter_outgoing_edges(nid))
            pred_counts = Counter((e.label for e in outgoing_edges))
            tables[nid] = {e.label: [] if c > 1 else None for e, c in pred_counts.items()}

            for e in outgoing_edges:
                c = sm.nodes[e.target_id]
                if isinstance(c, DataNode):
                    if pred_counts[e.label] > 1:
                        tables[nid][e.label].append(attrs[c.attr_id])
                    else:
                        tables[nid][e.label] = attrs[c.attr_id]
                elif isinstance(c, LiteralNode):
                    if pred_counts[e.label] > 1:
                        tables[nid][e.label].append(c.value)
                    else:
                        tables[nid][e.label] = c.value
                else:
                    if nid not in relations:
                        relations[nid] = {}
                    if c.node_id not in relations:
                        relations[c.node_id] = {}

                    assert False
                    # relations[nid][c.node_id] = e.label

    return NDArray(tables, relations)
