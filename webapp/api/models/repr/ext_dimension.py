#!/usr/bin/python
# -*- coding: utf-8 -*-
from dataclasses import dataclass
from typing import *

from api.misc.dict2instance import Dict2InstanceDeSer, get_object_map_deser, get_object_enum_deser, \
    get_list_int_deser, get_object_list_deser

ExtDimension = Union['ExtRangeDimension', 'ExtIndexDimension']


class ExtRangeDimension(Dict2InstanceDeSer):
    def __init__(self, range: List[int], values: List[Optional[ExtDimension]]):
        self.range = range
        self.values = values


class ExtIndexDimension(Dict2InstanceDeSer):
    def __init__(self, values: Dict[str, Optional[ExtDimension]]):
        self.values = values


ExtDimensionDeSer = get_object_enum_deser({"range": ExtRangeDimension, "index": ExtIndexDimension})
ExtIndexDimension.class_properties = {"values": get_object_map_deser("values", ExtDimensionDeSer, True)}
ExtRangeDimension.class_properties = {
    "range": get_list_int_deser("range"),
    "values": get_object_list_deser("values", ExtDimensionDeSer, True)
}
