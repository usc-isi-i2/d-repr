from typing import List, Dict, Tuple, Callable, Any, Optional
import numpy as np

from drepr.engine import complete_description
from drepr.executors.cf_convention_map.cf_convention_map import CFConventionNDArrayMap
from drepr.models import Path, DRepr, ujson
from drepr.outputs.array_based.array_attr import ArrayAttr, ScalarAttr
from drepr.outputs.array_based.array_backend import ArrayBackend
from drepr.outputs.array_based.indexed_sm import IndexedSM


def test_array_backend():
    ds_model = DRepr.parse_from_file("/workspace/d-repr/pydrepr/tests/resources/synthesis_s2/model.yml")
    resource_file = "/workspace/d-repr/pydrepr/tests/resources/synthesis_s2/resource.json"
    plan = complete_description(ds_model)
    result, attrs = CFConventionNDArrayMap.execute(ds_model, resource_file)
    sm = IndexedSM(plan.sm)
    # alignments = {
    #     (f"dnode:{s}", f"dnode:{t}"): aligns
    #     for (s, t), aligns in plan.alignments.items()
    # }
    alignments = plan.alignments

    backend = ArrayBackend(sm, attrs, alignments)
    for cls in backend.iter_classes_by_name("mint:Variable"):
        for record in cls.iter_records():
            print(record.to_dict())
    # c = backend.c("mint:Variable")
    # c.p("mint-geo:raster", "mint-geo:Raster")
    # for raster, sub_c in c.group_by(backend.sm.c("mint-geo:Raster")):
    #     sub_c.p('rdf:value').as_ndarray(sub_c.p("mint-geo:lat"), sub_c.p("mint-geo:long"))
    # sub_c.p("rdf:value")

    # ArrayBackend()