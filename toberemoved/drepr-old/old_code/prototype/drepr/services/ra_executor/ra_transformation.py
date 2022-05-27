from typing import List, Dict, Tuple, Callable, Any, Optional
from uuid import uuid4

import networkx as nx
from drepr.models import Representation
from drepr.models.preprocessing import TransformFunc
from drepr.services.ra_reader.ra_reader import RAReader


def exec_transformation(ra_reader: RAReader, repr: Representation):
    # build dependency_graph
    g = nx.DiGraph()
    for var in repr.get_variables():
        g.add_node(var.id)

    var2funcs = {}
    for trans_func in repr.transformation.transformations:
        if not g.has_node(trans_func.output_var):
            g.add_node(trans_func.output_var)

        for vid in trans_func.input_vars:
            if not g.has_node(vid):
                g.add_node(vid)
            g.add_edge(vid, trans_func.output_var)

        var2funcs[trans_func.output_var] = { "func": trans_func, "is_generated": False }

    try:
        next(nx.simple_cycles(g))
        raise Exception("There must be no circular dependency between variables")
    except StopIteration:
        pass

    roots = [n for n in g.nodes if g.in_degree(n) == 0 and g.out_degree(n) > 0]
    dynamic_vars = {}
    for root in roots:
        for u, v in nx.dfs_edges(g, root):
            if var2funcs[v]['is_generated']:
                continue

            # execute preprocessing to generate v
            dynamic_vars[v] = exec_trans_func(ra_reader, repr, dynamic_vars, var2funcs[v]['func'])
    return dynamic_vars


def exec_trans_func(ra_reader: RAReader, repr: Representation, dynamic_vars: dict, trans_func: TransformFunc):
    def get_value_at_index(idx: List[int]):
        # TODO: fix me!
        val = ra_reader.get_value(idx)
        return val

    sess_locals = {
        "get_value_at_index": get_value_at_index
    }

    for vid in trans_func.input_vars:
        if vid in repr.layout.variables:
            var = repr.get_variable(vid)
            assert var.get_estimated_size(upper_bound=1000) == 1

            val = ra_reader.get_value([s.idx for s in var.location.slices])
            # TODO: fix me!
            if val.isdigit():
                val = int(val)
        else:
            val = dynamic_vars[vid]

        sess_locals[vid] = val

    assert '__return__' not in trans_func.code

    fname = f"func_{str(uuid4()).replace('-', '')}"
    new_code = [f"def {fname}():"]
    for line in trans_func.code.split("\n"):
        # TODO: should detect if it is tab indented or space
        new_code.append("\t" + line)

    new_code.append(f"\n__return__ = {fname}()")
    new_code = "\n".join(new_code)
    exec(new_code, sess_locals, sess_locals)

    return sess_locals['__return__']
