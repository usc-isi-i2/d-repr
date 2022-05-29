import os
from typing import List

import numpy as np
import pytest
import orjson

from drepr.outputs.array_backend.array_backend import ArrayBackend
from drepr.outputs.base_lst_output_class import BaseLstOutputClass
from drepr.outputs.base_output_class import BaseOutputClass
from drepr.outputs.base_output_sm import BaseOutputSM
from drepr.outputs.record_id import BlankRecordID, GraphRecordID


def test_from_drepr(s01, s02, s03, s05):
    """This function does a smoke test to ensure that we can output all datasets"""
    pass


def test_get_class_by_id(s01: List[BaseOutputSM], s02: List[BaseOutputSM]):
    # has single class and get them by id
    for sm in (item for lst in [s01, s02] for item in lst):
        cls_id = sm.c("mint:Variable")[0].id
        cls = sm.cid(cls_id)
        assert isinstance(cls, BaseOutputClass)
        assert cls.id == cls_id


def test_get_classes_by_uri(s01: List[BaseOutputSM], s03: List[BaseOutputSM]):
    # for single class, directly get array class
    for sm in s01:
        lst = sm.c("mint:Variable")
        assert isinstance(lst, BaseLstOutputClass) and len(lst) == 1 and lst[0].uri == "mint:Variable"

    for sm in s03:
        lst = sm.c("mint:Variable")
        assert isinstance(lst, BaseLstOutputClass) and len(lst) == 2
        assert all(x.uri == "mint:Variable" for x in lst)


def test_get_record_by_id_blank(s01: List[BaseOutputSM]):
    for sm in s01:
        if isinstance(sm, ArrayBackend):
            record = sm.get_record_by_id(BlankRecordID((), sm.c("mint-geo:Raster")[0].id))
        else:
            record = sm.get_record_by_id(GraphRecordID("_:Raster1", sm.c("mint-geo:Raster")[0].id))

        d = record.to_dict()
        d.pop('@id')
        assert d == {
            'mint-geo:dx': [0.1],
            'mint-geo:dy': [0.1],
            'mint-geo:epsg': [4326],
            'mint-geo:x_min': [44.1],
            'mint-geo:x_slope': [0.0],
            'mint-geo:y_min': [20.1],
            'mint-geo:y_slope': [0.0]
        }

@pytest.mark.skip
def test_circular_reference(resource_dir):
    """
    This test is to make sure that D-REPR outputs don't contains circular reference and object should be freed as soon
    as it goes out of scope. Otherwise, they can hit out of memory error on some rare cases
    """
    dsmodel = str(resource_dir / "s01_synthesis" / "model.yml")
    import psutil
    process = psutil.Process(os.getpid())
    mem_hist = []

    def get_mem_usage():
        # get KB
        mem_hist.append(process.memory_info().rss / 1024)
        diff_mem = ""
        if len(mem_hist) > 1:
            diff_mem = f"Diff[-1]={mem_hist[-1] - mem_hist[-2]:.2f} Diff[0]={mem_hist[-1] - mem_hist[0]:.2f}"
        return f"KB={mem_hist[-1]:.2f} {diff_mem}"

    tempfile = resource_dir / "temp_resource_circular_reference.tmp.json"
    if not tempfile.exists():
        np.random.seed(1111)
        with open(tempfile, "wb") as f:
            x, y = 1000, 1000
            f.write(orjson.dumps({
                "lat": [20.1 + (i * 0.1) for i in range(x)],
                "long": [44.1 + (i * 0.1) for i in range(y)],
                "epsg": 4326,
                "value": np.random.randn(x, y).tolist()
            }))
    tempfile = str(tempfile)
    import weakref
    class Parent():
        def __init__(self):
            self.data = np.random.randn(100, 100)
            self.name = "fkdsl"
            self.child = Child(self)

    class Child():
        def __init__(self, parent):
            # self.parent = parent
            self.parent = weakref.ref(parent)
            # print(self.parent().name)

    a = []
    for i in range(41):
        backend = ArrayBackend.from_drepr(dsmodel, tempfile)
        # ai = np.random.randn(100, 100)
        # ai = Parent()
        # a.append(ai)
        print(i, get_mem_usage())
        # gc.collect(2)