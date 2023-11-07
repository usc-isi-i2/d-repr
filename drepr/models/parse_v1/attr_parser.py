from typing import List

from drepr.utils.validator import Validator, InputError
from .path_parser import PathParser
from ..attr import Attr, Sorted, ValueType
from ..resource import Resource

class AttrParser:
    """
    Attributes have two possible schemas
    1. When an attribute has only one path
        ```
        <attr_id>: <attr_path>
        # .. other attributes ..
        ```
    2.
        ```
        <attribute_id>:
            [resource_id]: <resource_id> (default is "default")
            path: <attr_path>
            [unique]: true|false (default is false)
            [sorted]: none|ascending|descending (default is none)
            [value_type]: unspecified|int|float|str|list[int]|list[str]|list[float] (default is unspecified)
            [missing_values]: [<value0>, <value1>, ...]
        ```
    """
    SORTED_VALUES = {x.value for x in Sorted}
    VALUE_TYPE_VALUES = {x.value for x in ValueType}

    def __init__(self, path_parser: PathParser):
        self.path_parser = path_parser

    def parse(self, default_resource_id: str, resources: List[Resource], attrs: dict) -> List[Attr]:
        Validator.must_be_dict(attrs, "Parsing attributes")
        result = []
        for attr_id, attr_conf in attrs.items():
            trace = f"Parsing attribute: {attr_id}"

            if isinstance(attr_conf, (str, list)):
                attr_path = self.path_parser.parse(
                    self.path_parser.get_resource(resources, default_resource_id, trace),
                    attr_conf, trace)
                attr = Attr(attr_id, default_resource_id, attr_path, [])
            elif isinstance(attr_conf, dict):
                attr = self.parse_schema2(default_resource_id, resources, attr_id, attr_conf, trace)
            else:
                raise InputError(
                    f"{trace}\nERROR: The configuration of an attribute can either be string, list, "
                    f"or dictionary. Get {type(attr_conf)} instead")
            result.append(attr)

        return result

    def parse_schema2(self, default_resource_id: str, resources: List[Resource], attr_id: str, attr_conf: dict, parse_trace: str) -> Attr:
        resource_id = attr_conf.get("resource_id", default_resource_id)
        Validator.must_be_str(resource_id, f"{parse_trace}\nParsing `resource_id`")

        Validator.must_have(attr_conf, "path", parse_trace)
        path = self.path_parser.parse(
            self.path_parser.get_resource(resources, resource_id, parse_trace),
            attr_conf['path'], f"{parse_trace}\nParsing path of the attribute")

        if "unique" in attr_conf and not isinstance(attr_conf['unique'], bool):
            raise InputError(f"{parse_trace}\nERROR: invalid value of the `unique` attribute. "
                             f"Expected a boolean value. Get: {attr_conf['unique']}")
        unique = attr_conf.get('unique', False)

        if 'sorted' in attr_conf:
            Validator.must_in(attr_conf['sorted'], self.SORTED_VALUES,
                              f"{parse_trace}\nParsing `sorted` of the attribute")
        sorted = Sorted(attr_conf.get('sorted', Sorted.Null.value))

        if 'value_type' in attr_conf:
            Validator.must_in(attr_conf['value_type'], self.VALUE_TYPE_VALUES,
                              f"{parse_trace}\nParsing `value_type` of the attribute")
        value_type = ValueType(attr_conf.get('value_type', ValueType.Unspecified.value))

        if 'missing_values' in attr_conf:
            trace = f"{parse_trace}\nParsing missing_values of the attribute"
            Validator.must_be_list(attr_conf['missing_values'], trace)
            for val in attr_conf['missing_values']:
                if not isinstance(val, (str, int, float)):
                    raise InputError(
                        f"{trace}\nERROR: invalid value. Expected either one of string, "
                        f"integer, or float. Get f{type(val)} instead")

        missing_values = attr_conf.get('missing_values', [])
        return Attr(attr_id, resource_id, path, missing_values, unique, sorted, value_type)
