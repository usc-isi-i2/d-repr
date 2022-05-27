#!/usr/bin/python
# -*- coding: utf-8 -*-
from typing import Union

from api.misc.dict2instance import Dict2InstanceDeSer, get_str_deser, get_object_enum_deser


class CSVResource(Dict2InstanceDeSer):
    class_properties = {
        "id": get_str_deser("id"),
        "delimiter": get_str_deser("delimiter")
    }
    class_property_default_values = {
        "delimiter": ","
    }

    def __init__(self, id: str, delimiter: str):
        self.id = id
        self.delimiter = delimiter


class JSONResource(Dict2InstanceDeSer):
    class_properties = {
        "id": get_str_deser("id")
    }

    def __init__(self, id: str):
        self.id = id


ResourceDeSer = get_object_enum_deser({"csv": CSVResource, "json": JSONResource})
ResourceEnum = Union[CSVResource, JSONResource]