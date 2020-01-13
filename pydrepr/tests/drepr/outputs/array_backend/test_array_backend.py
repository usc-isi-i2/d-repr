from drepr.outputs.array_based.array_backend import ArrayBackend
from drepr.outputs.array_based.array_class import ArrayClass
from drepr.outputs.array_based.lst_array_class import LstArrayClass
from drepr.outputs.array_based.record_id import BlankRecordID


def test_get_class_by_id(ds01: ArrayBackend, ds02: ArrayBackend):
    # has single class and get them by id
    for ds in [ds01, ds02]:
        cls_id = ds.sm.c("mint:Variable")[0].node_id
        cls = ds.get_class_by_id(cls_id)
        assert isinstance(cls, ArrayClass)
        assert cls.id == cls_id


def test_get_classes_by_uri(ds01: ArrayBackend, ds03: ArrayBackend):
    # for single class, directly get array class
    cls = ds01.get_classes_by_uri("mint:Variable")
    assert isinstance(cls, ArrayClass) and cls.cls.label == "mint:Variable"

    cls = ds03.get_classes_by_uri("mint:Variable")
    assert isinstance(cls, LstArrayClass) and len(cls.classes) == 2
    assert all(x.cls.label == "mint:Variable" for x in cls.classes)


def test_get_record_by_id_blank(ds01):
    record = ds01.get_record_by_id(BlankRecordID((), ds01.sm.c("mint-geo:Raster")[0].node_id))
    assert record.to_dict() == {
        '@id': (),
        'mint-geo:dx': [0.1],
        'mint-geo:dy': [0.1],
        'mint-geo:epsg': [4326],
        'mint-geo:x_min': [44.1],
        'mint-geo:x_slope': [0.0],
        'mint-geo:y_min': [20.1],
        'mint-geo:y_slope': [0.0]
    }
