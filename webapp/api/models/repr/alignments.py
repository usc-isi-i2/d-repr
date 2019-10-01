#!/usr/bin/python
# -*- coding: utf-8 -*-
from typing import Union, List

from api.misc.dict2instance import Dict2InstanceDeSer, get_str_deser, DeSer, get_object_enum_deser


class DimAlignment(Dict2InstanceDeSer):
    class_properties = {
        "source": get_str_deser("source"),
        "target": get_str_deser("target"),
        "aligned_dims": DeSer,
    }

    def __init__(self, source: str, target: str, aligned_dims: List[dict]):
        self.source = source
        self.target = target
        self.aligned_dims = aligned_dims


class ValueAlignment(Dict2InstanceDeSer):
    class_properties = {
        "source": get_str_deser("source"),
        "target": get_str_deser("target"),
    }

    def __init__(self, source: str, target: str):
        self.source = source
        self.target = target


AlignmentDeSer = get_object_enum_deser({"dimension": DimAlignment, "value": ValueAlignment})
AlignmentEnum = Union[DimAlignment, ValueAlignment]
