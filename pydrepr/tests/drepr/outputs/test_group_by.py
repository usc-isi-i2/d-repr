from typing import List, Dict, Tuple, Callable, Any, Optional

from drepr.outputs.array_backend.array_backend import ArrayBackend
from drepr.outputs.base_output_sm import BaseOutputSM
from drepr.outputs.record_id import GraphRecordID, BlankRecordID


def test_group_by_static_property(s01: List[BaseOutputSM], s02: List[BaseOutputSM]):
    for sm in (item for lst in [s01, s02] for item in lst):
        if isinstance(sm, ArrayBackend):
            raster_id = BlankRecordID((), sm.c("mint-geo:Raster")[0].id)
        else:
            raster_id = GraphRecordID("_:Raster1", sm.c("mint-geo:Raster")[0].id)

        gold_groups = [
            (raster_id, {
                "size": 25,
                "samples": {
                    6: {
                        "mint-geo:lat": [20.2],
                        "mint-geo:long": [44.2],
                        "rdf:value": [-0.43058764]
                    }
                }
            })
        ]
        for k, group in sm.c("mint:Variable")[0].group_by("mint:raster"):
            gk, g = gold_groups.pop(0)
            assert k == gk
            group = list(group.iter_records())
            assert len(group) == g['size']
            for i, record in g['samples'].items():
                assert group[i].m("mint-geo:lat") == record['mint-geo:lat']
                assert group[i].m("mint-geo:long") == record['mint-geo:long']
                assert group[i].m("rdf:value") == record['rdf:value']
