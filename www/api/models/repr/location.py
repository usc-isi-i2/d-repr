#!/usr/bin/python
# -*- coding: utf-8 -*-

from typing import *

from api.misc.dict2instance import Dict2InstanceDeSer, DeSer, get_bool_deser, get_str_deser, get_set_deser, \
    get_object_list_deser, get_object_enum_deser
from drepr import models


class IndexSlice(Dict2InstanceDeSer):
    class_properties = {
        "idx": DeSer
    }

    def __init__(self, idx: Union[str, int]):
        self.idx = idx


class RangeSlice(Dict2InstanceDeSer):
    class_properties = {
        "start": DeSer,
        "step": DeSer,
        "end": DeSer
    }

    def __init__(self, start: Union[str, int], end: Union[None, str, int], step: Union[str, int]):
        self.start = start
        self.end = end
        self.step = step


class Location(Dict2InstanceDeSer):
    class_properties = {
        "resource_id": get_str_deser("resource_id"),
        "slices": get_object_list_deser("slices", get_object_enum_deser({"index": IndexSlice, "range": RangeSlice}))
    }

    def __init__(self, resource_id: str, slices: List[Union[IndexSlice, RangeSlice]]):
        self.resource_id = resource_id
        self.slices = slices

    @staticmethod
    def from_path(resource_id: str, path: models.Path) -> "Location":
        # TODO: fix me! not a good implementation
        slices = []
        for step in path.steps:
            if isinstance(step, models.RangeExpr):
                slices.append(RangeSlice(step.start, step.end, step.step))
            elif isinstance(step, models.IndexExpr):
                slices.append(IndexSlice(step.val))
            else:
                raise NotImplementedError()
        return Location(resource_id, slices)

    def get_path(self) -> models.Path:
        steps = []
        for slice in self.slices:
            if isinstance(slice, RangeSlice):
                steps.append(models.RangeExpr(slice.start, slice.end, slice.step))
            elif isinstance(slice, IndexSlice):
                steps.append(models.IndexExpr(slice.idx))
            else:
                raise NotImplementedError()
        return models.Path(steps)
