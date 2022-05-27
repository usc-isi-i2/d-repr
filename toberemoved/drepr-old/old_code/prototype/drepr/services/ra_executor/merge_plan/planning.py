from typing import *
from uuid import uuid4

from drepr.mfunc.index_mapping_func import MappingFunc
from drepr.mfunc.mapping_graph import MappingGraph
from drepr.models import Variable, Location, IndexSlice
from drepr.models.representation import Representation
from drepr.models.variable import Slice
from drepr.models.mapping import DimensionMapping, IdenticalMapping
from drepr.models.semantic_model import SemanticModel
from drepr.services.ra_executor.planner_misc import get_norm_value_func
from drepr.services.ra_executor.preprocessing.ground_location import ground_location
from drepr.services.ra_reader.ra_reader import RAReader


class DataPropertyPlan:
    def __init__(self,
                 mfunc: MappingFunc,
                 predicate: str,
                 target_var: Variable,
                 norm_val: Callable[[Any], Any],
                 is_optional: bool = True):
        self.mfunc = mfunc
        self.predicate = predicate
        self.target_var = target_var
        self.norm_val = norm_val
        self.is_optional = is_optional


class ObjectPropertyPlan:
    def __init__(self,
                 mfunc: MappingFunc,
                 predicate: str,
                 target_class_id: str,
                 is_optional: bool = True):
        self.mfunc = mfunc
        self.predicate = predicate
        self.target_class_id = target_class_id
        self.is_optional = is_optional


class NodePlan:
    def __init__(self, primary_key: Variable, class_id: str,
                 data_properties: List[DataPropertyPlan],
                 object_properties: List[ObjectPropertyPlan]):
        self.primary_key = primary_key
        self.class_id = class_id
        self.data_properties = data_properties
        self.object_properties = object_properties

    def get_data_item_id(self, idx: List[int]) -> str:
        return f'{self.class_id}:{",".join((str(x) for x in idx))}'


def get_class2pk(repr: Representation, ra_reader: RAReader, sm: SemanticModel, mg: MappingGraph) -> Dict[str, str]:
    """Note: slice effect that repr may be changed"""
    class2pk: Dict[str, str] = {}

    for c in sm.iter_class_nodes():
        a_pk = find_pk(repr, sm, mg, c)
        if a_pk is None:
            a_pk = create_pseudo_pk(repr, ra_reader, sm, mg, c)
            if a_pk is None:
                raise Exception(
                    f"There is no primary key of class: {c}. User need to specify it explicitly")
        class2pk[c] = a_pk

    return class2pk


def gen_data_access_plan(ra_reader: RAReader, repr: Representation, sm: SemanticModel,
                         mg: MappingGraph) -> List[NodePlan]:
    class2pk: Dict[str, str] = get_class2pk(repr, ra_reader, sm, mg)

    node_plans = []
    for c, a_pk in class2pk.items():
        plan = NodePlan(
            repr.get_variable(a_pk), class_id=c, data_properties=[], object_properties=[])
        for u in get_data_properties(sm, c):
            mfunc = mg.get_mapping_exec_func(ra_reader, a_pk, u)
            norm_val = get_norm_value_func(repr.get_variable(u), sm.semantic_types[u])
            plan.data_properties.append(
                DataPropertyPlan(mfunc, sm.graph.edges[c, u]['label'], repr.get_variable(u),
                                 norm_val))

        for u in get_class_properties(sm, c):
            if not mg.has_mapping_func(a_pk, class2pk[u]):
                raise Exception(
                    f"There is no mapping function between primary keys `{a_pk}` and `{class2pk[u]}`. User need to specify it explicitly"
                )

            mfunc = mg.get_mapping_exec_func(ra_reader, a_pk, class2pk[u])
            plan.object_properties.append(
                ObjectPropertyPlan(mfunc, sm.graph.edges[c, u]['label'], u))

        node_plans.append(plan)
    return node_plans


def find_pk(repr: Representation, sm: SemanticModel, mg: MappingGraph, cid: str) -> Optional[str]:
    primary_keys = []
    for u in get_data_properties(sm, cid):
        for v in get_data_properties(sm, cid):
            if u == v:
                continue

            if not mg.has_mapping_func(u, v) or not mg.get_mapping_func(
                    u, v).is_single_value_func(repr):
                break
        else:
            primary_keys.append(u)
            break

    if len(primary_keys) == 0:
        return None
    return primary_keys[0]


def create_pseudo_pk(repr: Representation, ra_reader: RAReader, sm: SemanticModel, mg: MappingGraph,
                     cid: str) -> Optional[str]:
    for u in sm.iter_data_nodes():
        if all(
                mg.has_mapping_func(u, v) and mg.get_mapping_func(u, v).is_single_value_func(repr)
                for v in get_data_properties(sm, cid) if u != v):
            cpk = create_pseudo_pk_sub(repr, ra_reader, sm, mg, cid, u)
            if cpk is not None:
                return cpk
    return None


def create_pseudo_pk_sub(repr: Representation, ra_reader: RAReader, sm: SemanticModel, mg: MappingGraph, cid: str,
                         a: str) -> Optional[str]:
    fs = [mg.get_mapping_func(a, ai) for ai in get_data_properties(sm, cid) if ai != a]

    if all(isinstance(f, DimensionMapping) and f.is_surjective() for f in fs):
        fs: List[DimensionMapping]
        mapped_dims = [dim for f in fs for dim in f.source_dims]
        if len(mapped_dims) == len(set(mapped_dims)):
            # we did generate all combination of cid
            mapped_dims = set(mapped_dims)
            original_var = repr.get_variable(a)
            slices = []
            for i, slice in enumerate(original_var.location.slices):
                if i not in mapped_dims and slice.is_range():
                    # for non-integer dimension
                    slices.append(IndexSlice(idx=slice.start))
                else:
                    slices.append(slice)

            location = Location(original_var.location.resource_id, slices)
            var = Variable(f"{uuid4()}-{original_var.id}", original_var.value, location,
                           original_var.sorted, original_var.unique, original_var.missing_values,
                           original_var.type)

            ground_location(ra_reader, repr, location)
            repr.upsert_variable(var)
            mg.add_new_variable(var.id)

            # inherits all mapping functions from original variable
            # TODO: fix me! inherit mapping functions from original variable to other variables as well
            for f in fs:
                g = DimensionMapping(var.id, f.source_dims, f.target_var, f.target_dims)
                mg.set_mapping_func(g, var.id, f.target_var)
                repr.add_mapping(g)

            mapped_dims = sorted(mapped_dims)
            g = DimensionMapping(original_var.id, mapped_dims, var.id, mapped_dims)
            mg.set_mapping_func(g, original_var.id, var.id)

            return var.id
        else:
            return
    else:
        return None


def get_data_properties(sm: SemanticModel, cid: str) -> Generator[str, None, None]:
    for u in sm.graph.successors(cid):
        if sm.graph.nodes[u]['is_class_node']:
            continue
        yield u


def get_class_properties(sm: SemanticModel, cid: str) -> Generator[str, None, None]:
    for u in sm.graph.successors(cid):
        if sm.graph.nodes[u]['is_class_node']:
            yield u
