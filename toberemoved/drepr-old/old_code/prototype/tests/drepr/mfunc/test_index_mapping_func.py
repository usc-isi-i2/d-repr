from copy import copy

from drepr.mfunc.index_mapping_func import IndexMappingFunc
from drepr.models import Location, DimensionMapping
from drepr.models.parser_v1 import LocationReadableDeSerV1


def test_single_val():
    dm = DimensionMapping("x", [1], "y", [0])
    xloc = LocationReadableDeSerV1.unsafe_deserialize({'resource_id': 'default', 'slices': "5:0..10".split(":")})
    yloc = LocationReadableDeSerV1.unsafe_deserialize({'resource_id': 'default', 'slices': "1..11:6".split(":")})

    mfunc = IndexMappingFunc(dm, xloc, yloc)
    assert mfunc.is_single_value_func()
    assert mfunc.single_val_exec(None, [5, 7]) == [8, 6]
    assert mfunc.single_val_exec(None, [5, 9]) == [10, 6]


def test_set_val():
    dm = DimensionMapping("x", [1], "y", [0])
    xloc = LocationReadableDeSerV1.unsafe_deserialize({'resource_id': 'default', 'slices': "5:0..10".split(":")})
    yloc = LocationReadableDeSerV1.unsafe_deserialize({'resource_id': 'default', 'slices': "1..11:6:3..5".split(":")})

    mfunc = IndexMappingFunc(dm, xloc, yloc)
    assert mfunc.is_set_value_func()
    assert [copy(x) for x in mfunc.multiple_val_exec(None, [5, 9])] == [[10, 6, 3], [10, 6, 4]]

    dm = DimensionMapping("x", [0, 1], "y", [2, 0])
    xloc = LocationReadableDeSerV1.unsafe_deserialize({'resource_id': 'default', 'slices': "2..4:7..10".split(":")})
    yloc = LocationReadableDeSerV1.unsafe_deserialize({'resource_id': 'default', 'slices': "9..12:3..5:4..6".split(":")})

    mfunc = IndexMappingFunc(dm, xloc, yloc)
    assert mfunc.is_set_value_func()
    assert [copy(x) for x in mfunc.multiple_val_exec(None, [3, 9])] == [[11, 3, 5], [11, 4, 5]]