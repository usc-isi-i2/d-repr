from typing import List, Dict, Tuple, Callable, Any, Optional

from drepr import DRepr, outputs
from drepr.engine import complete_description


def test_complete_description(d_s04: DRepr):
    model2subjects = {
        "d_s04": {
            ("mint:Variable:1", "dnode:Rainf_f_tavg"),
            ("mint:Variable:2", "dnode:Albedo_inst")
        }
    }

    for ds_model in [d_s04]:
        plan = complete_description(ds_model)

        # check all subjects are inferred correctly (which shows that all inferred alignments are correct)
        subjects = model2subjects[ds_model.__name__]
        assert len(subjects) > 0

        for edge in plan.sm.edges.values():
            pair = (edge.source_id, edge.target_id)
            if pair in subjects:
                assert edge.is_subject is True
                subjects.remove(pair)
        assert len(subjects) == 0
