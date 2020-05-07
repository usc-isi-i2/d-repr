from typing import List

from drepr.outputs.array_backend.array_backend import ArrayBackend
from drepr.outputs.array_backend.array_class import ArrayClass
from drepr.outputs.array_backend.lst_array_class import LstArrayClass
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
