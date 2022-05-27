from typing import Dict, Optional, Union, Callable, Any

from drepr.mfunc.mapping_graph import MappingGraph
from drepr.misc.id2id import ID2ID
from drepr.misc.triple_graph import TripleGraph
from drepr.models import Representation, SemanticType
from drepr.query.return_format import ReturnFormat
from drepr.query.selection_interface import SelectionGraph
from drepr.services.ra_executor.merge_plan.planning import gen_data_access_plan, NodePlan
from drepr.services.ra_reader.ra_reader import RAReader


def prepare_query_plan(repr: Representation, ra_reader: RAReader, mg: MappingGraph,
                       sg: SelectionGraph, return_format: ReturnFormat) -> Dict[str, NodePlan]:
    def get_norm_value_func_ttl(var_stype: SemanticType) -> Callable[[Any], Any]:
        """Return a function that take a variable's value and normalize it correct format or value"""

        def no_func(val):
            if isinstance(val, str):
                return f'"{val}"'
            return val

        def to_string(val):
            return f'"{val}"'

        def to_decimal(val):
            return float(val)

        def to_int(val):
            return int(val)

        def to_uri(val):
            return f'<{val}>'

        if var_stype.value_type is None:
            return no_func
        elif var_stype.value_type == SemanticType.xsd_string:
            return to_string
        elif var_stype.value_type == SemanticType.xsd_int:
            return to_int
        elif var_stype.value_type == SemanticType.xsd_decimal:
            return to_decimal
        elif var_stype.value_type == SemanticType.xsd_any_uri:
            return to_uri
        else:
            raise NotImplementedError()

    sm = sg.to_sm(repr.semantic_model)
    plans = gen_data_access_plan(ra_reader, repr, sm, mg)

    for plan in plans:
        for dprop in plan.data_properties:
            dprop.is_optional = sg.graph.edges[plan.class_id, dprop.target_var.id]['is_optional']
            dprop.predicate = sg.graph.edges[plan.class_id, dprop.target_var.id]['label']

            if return_format.format == ReturnFormat.Turtle.format:
                dprop.norm_val = get_norm_value_func_ttl(sm.semantic_types[dprop.target_var.id])
        for oprop in plan.object_properties:
            oprop.is_optional = sg.graph.edges[plan.class_id, oprop.target_class_id]['is_optional']
            oprop.predicate = sg.graph.edges[plan.class_id, oprop.target_class_id]['label']

    return {p.class_id: p for p in plans}


def exec_query_using_merge_plan(repr: Representation,
                                ra_reader: RAReader,
                                mg: MappingGraph,
                                sg: SelectionGraph,
                                return_format: ReturnFormat):
    plans = prepare_query_plan(repr, ra_reader, mg, sg, return_format)
    class2instances: Dict[str, Dict[str, dict]] = {}
    class2linkage: Dict[str, Dict[str, Dict[str, Union[list, str]]]] = {}

    for plan in plans.values():
        plan_type = sg.graph.nodes[plan.class_id]['label']
        class2instances[plan.class_id] = {}
        class2linkage[plan.class_id] = {}

        for idx, val in ra_reader.iter_data(ra_reader.get_grounded_location(plan.primary_key.location)):
            # don't need to check it
            # if val in plan.primary_key.missing_values:
            #     continue
            object = {'@type': plan_type}
            object_pseudo_id = plan.get_data_item_id(idx)
            dismiss = False

            for dplan in plan.data_properties:
                if dplan.mfunc.is_single_value_func():
                    didx = dplan.mfunc.single_val_exec(val, idx)
                    dval = ra_reader.get_value(didx)

                    if isinstance(dval, (list, dict)) or dval not in dplan.target_var.missing_values:
                        dval = dplan.norm_val(dval)
                        object[dplan.predicate] = dval
                    elif not dplan.is_optional:
                        dismiss = True
                        break
                else:
                    result = []
                    for didx in dplan.mfunc.multiple_val_exec(val, idx):
                        dval = ra_reader.get_value(didx)
                        if dval not in dplan.target_var.missing_values:
                            result.append(dplan.norm_val(dval))

                    if len(result) == 0 and not dplan.is_optional:
                        dismiss = True
                        break

                    object[dplan.predicate] = result

            if not dismiss:
                class2instances[plan.class_id][object_pseudo_id] = object

            if len(plan.object_properties) > 0:
                links = {}
                for oplan in plan.object_properties:
                    if oplan.mfunc.is_single_value_func():
                        oidx = oplan.mfunc.single_val_exec(val, idx)
                        links[oplan.predicate] = plans[oplan.target_class_id].get_data_item_id(oidx)
                    else:
                        links[oplan.predicate] = [
                            plans[oplan.target_class_id].get_data_item_id(oidx)
                            for oidx in oplan.mfunc.multiple_val_exec(val, idx)
                        ]
                class2linkage[plan.class_id][object_pseudo_id] = links

    if return_format.format == ReturnFormat.JsonLD.format:
        cid = str(sg.main_node)
        if len(plans[cid].object_properties) == 0:
            return list(class2instances[cid].values())

        results = []
        for rid in class2instances[cid]:
            r = populate_data(cid, rid, sg, plans, class2instances, class2linkage)
            if r is not None:
                results.append(r)

        return results
    elif return_format.format == ReturnFormat.TripleGraph.format:
        g = TripleGraph()

        for cid, linkage in class2linkage.items():
            for pseudo_id, links in linkage.items():
                if pseudo_id not in class2instances[cid]:
                    continue

                source = class2instances[cid][pseudo_id]
                source_uri = source['@id'] if '@id' in source else pseudo_id
                g.add_uri(source_uri, source['@type'])

                for prop, vs in links.items():
                    if not isinstance(vs, list):
                        vs = [vs]

                    for v in vs:
                        if v in class2instances[cid]:
                            target = class2instances[cid][v]
                            target_uri = target['@id'] if '@id' in target else v

                            g.add_uri(target_uri, target['@type'])
                            g.add_object_triple(source_uri, prop, target_uri)

        for instances in class2instances.values():
            for pseudo_id, instance in instances.items():
                if '@id' is not instance:
                    uri = pseudo_id
                else:
                    uri = instance.pop('@id')

                g.add_uri(uri, instance.pop('@type'))
                for prop, value in instance.items():
                    g.add_data_triple(uri, prop, value)

        return g
    elif return_format.format == ReturnFormat.Turtle.format:
        triples = [
            "@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .",
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> ."
        ]
        id2ids = {}

        for prefix, uri in repr.semantic_model.ont_prefixes.items():
            triples.append(f"@prefix {prefix}: <{uri}> .")

        for cid, instances in class2instances.items():
            id2ids[cid] = ID2ID()

            for pseudo_id, instance in instances.items():
                if '@id' is not instance:
                    uri = f"_:{id2ids[cid].get_id(pseudo_id)}"
                else:
                    uri = f"<{instance.pop('@id')}>"

                triples.append(f"{uri} a {instance.pop('@type')} .")
                for prop, value in instance.items():
                    triples.append(f"{uri} {prop} {value} .")

        for cid, linkage in class2linkage.items():
            for pseudo_id, links in linkage.items():
                if pseudo_id not in class2instances[cid]:
                    continue

                source = class2instances[cid][pseudo_id]
                source_uri = f"<{source['@id']}>" if '@id' in source else f"_:{id2ids[cid].get_id(pseudo_id)}"
                for prop, vs in links.items():
                    if not isinstance(vs, list):
                        vs = [vs]

                    for v in vs:
                        if v in class2instances[cid]:
                            target = class2instances[cid][v]
                            target_uri = f"<{target['@id']}>" if '@id' in target else f"_:{id2ids[cid].get_id(v)}"

                            triples.append(f"{source_uri} {prop} {target_uri} .")
        return triples
    else:
        raise NotImplementedError()


def populate_data(cid: str, rid: str, sg: SelectionGraph, plans: Dict[str, NodePlan],
                  class2instances: Dict[str, Dict[str, dict]],
                  class2linkage: Dict[str, Dict[str, Union[list, str]]]) -> Optional[dict]:
    r = class2instances[cid][rid]
    if rid in class2linkage[cid]:
        links = class2linkage[cid][rid]
        dismiss = False

        for oplan in plans[cid].object_properties:
            if isinstance(links[oplan.predicate], list):
                r[oplan.predicate] = []

                for oid in links[oplan.predicate]:
                    if oid in class2instances[oplan.target_class_id]:
                        sr = populate_data(oplan.target_class_id, oid, sg, plans, class2instances,
                                           class2linkage)
                        if sr is not None:
                            r[oplan.predicate].append(sr)

                if len(r[oplan.predicate]
                       ) == 0 and not sg.graph.edges[cid, oplan.target_class_id]['is_optional']:
                    dismiss = True
                    break
            else:
                if links[oplan.predicate] in class2instances[oplan.target_class_id]:
                    r[oplan.predicate] = populate_data(oplan.target_class_id,
                                                       links[oplan.predicate], sg, plans,
                                                       class2instances, class2linkage)
                else:
                    r[oplan.predicate] = None

                if r[oplan.predicate] is None and not sg.graph.edges[cid, oplan.target_class_id][
                        'is_optional']:
                    dismiss = True
                    break

        if dismiss:
            return None

    return r


