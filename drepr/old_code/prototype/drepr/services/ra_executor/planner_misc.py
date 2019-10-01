from typing import *
from typing import Callable, Any

from drepr.models.representation import Representation
from drepr.models import Variable, SemanticType
from drepr.query.selection_interface import SelectionGraph


def select_start_data_node(repr: Representation, sg: SelectionGraph, u: str) -> Variable:
    """Return a data node, which number of values are equal to number of instances of class `u`"""
    best_node = None
    best_non_optional_node = None

    for v in sg.graph.successors(u):
        if not sg.graph.nodes[v]['is_class_node']:
            if best_node is None:
                best_node = repr.get_variable(v)
            elif best_node.cmp_size(repr.get_variable(v)) > 0:
                best_node = repr.get_variable(v)

            if not sg.graph.edges[u, v]['is_optional']:
                if best_non_optional_node is None:
                    best_non_optional_node = repr.get_variable(v)
                elif best_non_optional_node.cmp_size(repr.get_variable(v)) > 0:
                    best_non_optional_node = repr.get_variable(v)

    assert best_non_optional_node.cmp_size(best_node) == 0, "we cannot have optional on uri properties"
    return best_non_optional_node


def get_norm_value_func(var: Variable, var_stype: SemanticType) -> Callable[[Any], Any]:
    """Return a function that take a variable's value and normalize it correct format or value"""
    def no_func(val):
        return val

    def to_string(val):
        return str(val)

    def to_decimal(val):
        return float(val)

    def to_int(val):
        return int(val)

    if var_stype.value_type is None:
        return no_func
    elif var_stype.value_type == SemanticType.xsd_string:
        return to_string
    elif var_stype.value_type == SemanticType.xsd_int:
        return to_int
    elif var_stype.value_type == SemanticType.xsd_decimal:
        return to_decimal
    elif var_stype.value_type == SemanticType.xsd_any_uri:
        return to_string
    else:
        raise NotImplementedError()


def get_size_class_node(repr: Representation, sg: SelectionGraph, u: str) -> int:
    """Get number of individuals of class `u` in the dataset"""
    size = 0
    upper_bound = 10000

    for v in sg.graph.successors(u):
        if not sg.graph.nodes[v]['is_class_node']:
            v_size = repr.get_variable(v).get_estimated_size(upper_bound)
            if v_size > size:
                size = v_size

    return size
