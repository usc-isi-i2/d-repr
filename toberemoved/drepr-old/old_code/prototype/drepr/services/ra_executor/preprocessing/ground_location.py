from typing import List, Dict, Tuple, Callable, Any, Optional

from drepr.models import Representation, Location, DynamicRangeSlice, DynamicIndexSlice
import ast
import parser
import networkx as nx

from drepr.services.ra_reader.ra_reader import RAReader


def get_dependent_variables(repr: Representation, interpolated_str: str) -> List[str]:
    # to make sure that it is expression
    parser.expr(interpolated_str)

    st = ast.parse(interpolated_str)
    variable_names = []

    for node in ast.walk(st):
        if type(node) is ast.Name:
            var_id = node.id
            assert var_id in repr.variables, f"Variable {var_id} doesnt exist"
            variable_names.append(var_id)

    return variable_names


def get_dynamic_index(ra_reader: RAReader, repr: Representation, interpolated_str: str) -> int:
    dependent_vars = get_dependent_variables(repr, interpolated_str[2:-1])
    sess_globals = {}
    sess_locals = {}
    for vid in dependent_vars:
        var = repr.get_variable(vid)
        assert all(not s.is_range() for s in var.location.slices), "cannot be multiple-value"

        if not ra_reader.has_grounded_location(var.location):
            # this happen when preprocessing called ground_location, and location of a variable
            # is not grounded (it will be after preprocessing, but because preprocessing can use variable location
            # as the input, we have to ground the one used in preprocessing first)
            try:
                ground_location(ra_reader, repr, var.location)
            except Exception as e:
                raise Exception("The order of preprocessing step is not correct") from e

        var_loc = ra_reader.get_grounded_location(var.location)
        index, val = next(ra_reader.iter_data(var_loc))
        sess_locals[vid] = var.parse_value(val)

    return eval(interpolated_str[2:-1], sess_globals, sess_locals)


def ground_location(ra_reader: RAReader, repr: Representation, loc: Location) -> Location:
    ground_loc = loc.clone()
    for slice in ground_loc.slices:
        if isinstance(slice, DynamicRangeSlice):
            if type(slice.dyn_start) is str and slice.dyn_start.startswith("${"):
                slice.start = get_dynamic_index(ra_reader, repr, slice.dyn_start)
            if type(slice.dyn_end) is str and slice.dyn_end.startswith("${"):
                slice.end = get_dynamic_index(ra_reader, repr, slice.dyn_end)
            if type(slice.dyn_step) is str and slice.dyn_step.startswith("${"):
                slice.step = get_dynamic_index(ra_reader, repr, slice.dyn_step)
        elif isinstance(slice, DynamicIndexSlice):
            slice.idx = get_dynamic_index(ra_reader, repr, slice.dyn_idx)

    ra_reader.ground_location_mut(ground_loc)
    ra_reader.set_grounded_location(loc, ground_loc)
    return ground_loc


def ground_variable_locations(ra_reader: RAReader, repr: Representation):
    dyn_loc_vars = []
    fixed_loc_vars = []
    for var in repr.variables.values():
        if any(isinstance(s, (DynamicRangeSlice, DynamicIndexSlice)) for s in var.location.slices):
            dyn_loc_vars.append(var)
        else:
            fixed_loc_vars.append(var)

    # build a dependency graph
    g = nx.DiGraph()
    for var in dyn_loc_vars:
        g.add_node(var.id)

        for slice in var.location.slices:
            dependent_vars = []
            if isinstance(slice, DynamicRangeSlice):
                for x in [slice.dyn_start, slice.dyn_end, slice.dyn_step]:
                    if type(x) is str and x.startswith("${"):
                        dependent_vars.extend(get_dependent_variables(repr, x[2:-1]))
            elif isinstance(slice, DynamicIndexSlice):
                if type(slice.dyn_idx) is str and slice.dyn_idx.startswith("${"):
                    dependent_vars.extend(get_dependent_variables(repr, slice.dyn_idx[2:-1]))

            for dv in dependent_vars:
                if not g.has_node(dv):
                    g.add_node(dv)

                g.add_edge(dv, var.id)

    # travel the dependency graph to build the location one by one
    try:
        next(nx.simple_cycles(g))
        raise Exception("There must be no circular dependency between variables")
    except StopIteration:
        pass

    roots = [n for n in g.nodes if g.in_degree(n) == 0]
    for root in roots:
        var = repr.get_variable(root)
        if not ra_reader.has_grounded_location(var.location):
            ground_location(ra_reader, repr, var.location)

        for u, v in nx.dfs_edges(g, root):
            var = repr.get_variable(v)
            if not ra_reader.has_grounded_location(var.location):
                ground_location(ra_reader, repr, var.location)

    for var in fixed_loc_vars:
        if not ra_reader.has_grounded_location(var.location):
            ground_location(ra_reader, repr, var.location)
