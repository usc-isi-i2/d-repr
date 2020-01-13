from typing import List, Dict, Tuple, Callable, Any, Optional

from drepr.models import Path
from drepr.outputs.array_based.array_backend import ArrayBackend
from drepr.outputs.array_based.array_class import ArrayClass


def test_iter_records_s01_2(ds01, ds02):
    # basic access
    for ds in [ds01, ds02]:
        ds: ArrayBackend

        cls = ds.get_classes_by_uri("mint:Variable")
        assert isinstance(cls, ArrayClass)
        records = list(cls.iter_records())
        assert len(records) == 25
        assert records[6].to_dict() == {
            "@id": (1, 1),
            "mint-geo:lat": [20.2],
            "mint-geo:long": [44.2],
            "mint:raster": [()],
            "rdf:value": [-0.43058764]
        }
