import numpy as np
from collections import Counter

from drepr.executors.preprocessing.context import Context
from drepr.executors.preprocessing.py_exec import PyExec
from drepr.executors.readers.netcdf import NetCDF4Reader
from drepr.models import DRepr, ResourceType, PreprocessingType, IndexExpr, ClassNode, DataNode, LiteralNode
from drepr import ndarray


def map_netcdf(ds_model: DRepr, resource_file: str):
    """
    Execution steps:

    1. create resource reader (only support one resource)
    2. execute preprocessing function (which is map function and only mutate the value)
    3. gather attributes
    4. Create table:
        a. If there are duplicated predicates (duplicated column_id), then the resulted array will have an extra dimension at
           the end, which size is equal to the number of duplicated columns of `column_id`. (see the ndarray documentation)
    """
    # 1st: create resource reader
    resource = ds_model.resources[0]
    if resource.type == ResourceType.NetCDF4:
        reader = NetCDF4Reader.from_file(resource_file)
    else:
        raise NotImplementedError()

    # 2nd: execute preprocessing function
    context = Context(reader)
    for preprocess_fn in ds_model.preprocessing:
        if preprocess_fn.type == PreprocessingType.pmap:
            steps = preprocess_fn.value.path.steps
            assert all(isinstance(step, IndexExpr) for step in steps), "Range selection should use numpy map"

            index = [step.val for step in steps]
            fn = PyExec.compile(preprocess_fn.value.code)
            value = fn.exec(reader.get_value(index), index, context)
            reader.set_value(index, value)
        else:
            raise Exception("You found a bug")

    # 3rd: gather attributes
    attrs = {}
    for attr in ds_model.attrs:
        attrs[attr.id] = reader.select(attr.path.steps)
        # assert all(isinstance(step, IndexExpr) for step in attr.path.steps)
        # index = [step.val for step in steps]
        # attrs[attr.id] = reader.get_value(index)

    # 4th: create tables from the semantic model
    sm = ds_model.sm
    tables = {}
    table_shps = {}
    relations = {}

    for nid, node in sm.nodes.items():
        if isinstance(node, ClassNode):
            outgoing_edges = list(sm.iter_outgoing_edges(nid))
            pred_counts = Counter((e.label for e in outgoing_edges))
            assert all(pred_counts[k] == 1 for k in pred_counts.keys())
            for e in outgoing_edges:
                c = sm.nodes[e.target_id]
                if isinstance(c, DataNode):
                    index_by = [
                        align.target if align.target != c.attr_id else align.source
                        for align in ds_model.aligns
                        if c.attr_id in [align.target, align.source]
                    ]
                    tables[nid][e.label] = ndarray.ColArray(attrs[c.attr_id], index_by, attrs[c.attr_id].shape)
                elif isinstance(c, LiteralNode):
                    tables[nid][e.label] = ndarray.ColSingle(c.value)
                else:
                    if nid not in relations:
                        relations[nid] = {}
                    if c.node_id not in relations:
                        relations[c.node_id] = {}

                    relations[id][c.node_id] = e.label

        table_shps[nid] = []
        for col in tables[nid].values():
            if isinstance(col, ndarray.ColArray) and np.prod(table_shps[nid]) < col.get_original_size():
                table_shps[nid] = col.shape

    return ndarray.NDArrayTables(tables, table_shps, relations)