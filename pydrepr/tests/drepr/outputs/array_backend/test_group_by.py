from typing import List, Dict, Tuple, Callable, Any, Optional

from drepr.outputs.array_based.record_id import BlankRecordID


def test_group_by_static_property(ds01, ds02):
    for ds in [ds01, ds02]:
        gold_groups = [
            (BlankRecordID((), ds.sm.c("mint-geo:Raster")[0].node_id), {
                "size": 25,
                "samples": {
                    6: {
                        "@id": (1, 1),
                        "mint-geo:lat": [20.2],
                        "mint-geo:long": [44.2],
                        "mint:raster": [()],
                        "rdf:value": [-0.43058764]
                    }
                }
            })
        ]
        for k, group in ds.get_classes_by_uri("mint:Variable").group_by("mint:raster"):
            gk, g = gold_groups.pop(0)
            assert k == gk
            group = list(group.iter_records())
            assert len(group) == g['size']
            for i, record in g['samples'].items():
                assert group[i].to_dict() == record
