from drepr.models import Representation
from drepr.query.return_format import ReturnFormat
from drepr.query.selection_interface import SelectionGraph
from drepr.services.ra_executor.merge_plan.executor import exec_query_using_merge_plan
from drepr.services.ra_reader.ra_reader import RAReader


def exec_ra_query(repr: Representation, ra_reader: RAReader, sg: SelectionGraph, return_format: ReturnFormat):
    # step 1: align selection graph with semantic model so that we know exact variables to query
    sm = repr.semantic_model
    sg.align_with_semantic_model(sm)

    if not return_format.use_full_uri:
        sg.use_short_uri()

    # step 2: generate a query plan and execute it
    mg = repr.get_mapping_graph()
    return exec_query_using_merge_plan(repr, ra_reader, mg, sg, return_format)
