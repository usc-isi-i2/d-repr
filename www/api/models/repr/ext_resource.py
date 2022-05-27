#!/usr/bin/python
# -*- coding: utf-8 -*-
from typing import TYPE_CHECKING

from api.misc.dict2instance import Dict2InstanceDeSer, get_str_deser
from api.models.repr.ext_dimension import ExtDimensionDeSer, ExtDimension, ExtRangeDimension

if TYPE_CHECKING:
    from api.models.resource import Resource


class ExtResource(Dict2InstanceDeSer):
    class_properties = {
        "resource_db_id": get_str_deser("resource_db_id"),
        "resource_id": get_str_deser("resource_id"),
        "dimension": ExtDimensionDeSer
    }

    def __init__(self, resource_db_id: int, resource_id: str, dimension: ExtDimension):
        self.resource_db_id = resource_db_id
        self.resource_id = resource_id
        self.dimension = dimension

    @staticmethod
    def from_resource(wres: 'Resource') -> 'ExtResource':
        d = wres.get_data()
        if wres.resource['type'] == "csv":
            if len(d) == 0:
                raise ValueError("Resource is empty")
            dim = ExtRangeDimension(range=[0, len(d)], values=[ExtRangeDimension(range=[0, len(d[0])], values=[None])])
            return ExtResource(wres.db_id, wres.resource_id, dim)
        else:
            raise NotImplementedError()
