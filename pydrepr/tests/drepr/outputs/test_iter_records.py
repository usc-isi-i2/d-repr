from typing import List

from drepr.outputs.base_output_sm import BaseOutputSM


def test_iter_records_s01_2(s01: List[BaseOutputSM], s02: List[BaseOutputSM]):
    # basic access
    for sm in (item for lst in [s01, s02] for item in lst):
        cls = sm.c("mint:Variable")[0]
        records = list(cls.iter_records())
        assert len(records) == 25
        assert len(records) == len({r.id for r in records})
        assert len({r.s("mint:raster") for r in records}) == 1
        r = records[6]
        d = r.to_dict()
        d.pop('@id')
        d.pop('mint:raster')
        assert d == {
            "mint-geo:lat": [20.2],
            "mint-geo:long": [44.2],
            "rdf:value": [-0.43058764]
        }
        assert r.s("mint-geo:lat") == 20.2 and r.m("mint-geo:lat") == [20.2]
        assert r.s("mint-geo:long") == 44.2 and r.m("mint-geo:long") == [44.2]
        assert r.s("rdf:value") == -0.43058764 and r.m("rdf:value") == [-0.43058764]
