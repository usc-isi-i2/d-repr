#!/usr/bin/python
# -*- coding: utf-8 -*-

from typing import *

from api.misc.dict2instance import Dict2InstanceDeSer, get_bool_deser, get_str_deser, get_set_deser
from api.models.repr.location import Location


class Variable(Dict2InstanceDeSer):
    class_properties = {
        "id": get_str_deser("id"),
        "location": Location,
        "unique": get_bool_deser("unique"),
        "sorted": get_str_deser("sorted"),
        "value_type": get_str_deser("value_type"),
        "missing_values": get_set_deser("missing_values")
    }

    def __init__(self, id: str, location: Location, unique: bool, sorted: str, value_type: str, missing_values: Set[str]):
        self.id = id
        self.missing_values = missing_values
        self.sorted = sorted
        self.value_type = value_type
        self.unique = unique
        self.location = location
