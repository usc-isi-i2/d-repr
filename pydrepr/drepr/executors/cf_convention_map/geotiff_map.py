from collections import Counter
from pathlib import Path

import numpy as np
from PIL import Image
from PIL.TiffTags import TAGS

from drepr.models import DRepr, ClassNode, DataNode, RangeExpr, LiteralNode, IndexExpr
import drepr.ndarray as ndarray


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
            # if they select all, then we don't need to do anything
            if all(isinstance(step, RangeExpr) and step.start == 0 and step.end is None and step.step == 1 for step in attr.path.steps):
                attrs[attr.id] = data
            else:
                slices = []
                for step in attr.path.steps:
                    if isinstance(step, RangeExpr):
                        # TODO: checking for expression
                        assert type(step.start) is int and type(step.step) is int and (step.end is None or type(step.end) is int)
                        slices.append(slice(step.start, step.end, step.step))
                    elif isinstance(step, IndexExpr):
                        assert isinstance(step.val, int)
                        slices.append(step.val)
                    else:
                        raise NotImplementedError()
                attrs[attr.id] = data[slices]

    sm = ds_model.sm
    tables = {}
    table_shps = {}
    relations = {}

    # TODO: generate correct shape and relations
    for nid, node in sm.nodes.items():
        if isinstance(node, ClassNode):
            outgoing_edges = list(sm.iter_outgoing_edges(nid))
            pred_counts = Counter((e.label for e in outgoing_edges))
            tables[nid] = {e: [] if c > 1 else None for e, c in pred_counts.items()}
            for e in outgoing_edges:
                c = sm.nodes[e.target_id]
                if isinstance(c, DataNode):
                    if pred_counts[e.label] > 1:
                        tables[nid][e.label].append(attrs[c.attr_id])
                        raise NotImplementedError()
                    else:
                        index_by = [align.target if align.target != c.attr_id else align.source for align in ds_model.aligns if c.attr_id in [align.target, align.source]]
                        tables[nid][e.label] = ndarray.ColArray(attrs[c.attr_id], index_by, attrs[c.attr_id].shape)
                elif isinstance(c, LiteralNode):
                    if pred_counts[e.label] > 1:
                        tables[nid][e.label].append(ndarray.ColSingle(c.value))
                    else:
                        tables[nid][e.label] = ndarray.ColSingle(c.value)
                    raise NotImplementedError("TODO: handle ColSingle correctly")
                else:
                    if nid not in relations:
                        relations[nid] = {}
                    if c.node_id not in relations:
                        relations[c.node_id] = {}

                    assert False
                    # relations[nid][c.node_id] = e.label

            # TODO: fix me! a stupid simple heuristic
            table_shps[nid] = []
            for col in tables[nid].values():
                if isinstance(col, ndarray.ColArray) and np.prod(table_shps[nid]) < col.get_original_size():
                    table_shps[nid] = col.shape

    return ndarray.NDArrayTables(tables, table_shps, relations)


if __name__ == '__main__':
    from drepr.models import yaml

    resource_file = "/Users/rook/workspace/MINT/MINT-Transformation/examples/scotts_transformations/soil/CLYPPT_M_sl1_1km.tiff"
    ds_model = """
version: '1'
resources: geotiff
attributes:
    band_1: [.., ..]
alignments: []
semantic_model:
    data_nodes:
        band_1: qb:Observation:1--rdf:value^^xsd:decimal
    prefixes:
        qb: https://example.org/
    """
    tbl = map_geotiff(DRepr.parse(yaml.load(ds_model)), resource_file)
    print(">>>")
