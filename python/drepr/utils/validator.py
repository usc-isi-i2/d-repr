import abc
import re
from collections import OrderedDict
from typing import Any, Set, Iterable, Dict, Union

import ujson


class InputError(Exception):
    pass


class Validator:
    @staticmethod
    def must_be_dict(val: Any, error_msg: str):
        if not isinstance(val, (dict, OrderedDict)):
            raise InputError(f"{error_msg}\nERROR: Expect a dictionary. Get: {type(val)}")

    @staticmethod
    def must_be_list(val: Any, error_msg: str):
        if not isinstance(val, list):
            raise InputError(f"{error_msg}\nERROR: Expect a list. Get: {type(val)}")

    @staticmethod
    def must_be_str(val: Any, error_msg: str):
        if not isinstance(val, str):
            raise InputError(f"{error_msg}\nERROR: Expect a str. Get: {type(val)}")

    @staticmethod
    def must_be_int(val: Any, error_msg: str):
        if not isinstance(val, int):
            raise InputError(f"{error_msg}\nERROR: Expect a int. Get: {type(val)}")

    @staticmethod
    def must_be_bool(val: Any, error_msg: str):
        if not isinstance(val, bool):
            raise InputError(f"{error_msg}\nERROR: Expect a bool. Get: {type(val)}")

    @staticmethod
    def must_be_subset(parent: Set[Any], subset: Iterable[Any], setname: str, error_msg: str):
        if not parent.issuperset(subset):
            raise InputError(f"{error_msg}\nERROR: {setname.capitalize()} must be a subset of {parent}. Get: {subset}")

    @staticmethod
    def must_in(val: Any, choices: Set[str], error_msg: str):
        if val not in choices:
            raise InputError(
                f"{error_msg}\nERROR: Get `{val}` while possible values are {choices}"
            )

    @staticmethod
    def must_have(odict: dict, attr: str, error_msg: str):
        if attr not in odict:
            raise InputError(
                f"{error_msg}\nERROR: The attribute `{attr}` is missing in the object: `{ujson.dumps(odict, indent=4)}`")

    @staticmethod
    def must_equal(val: Any, expected_val: Any, error_msg: str):
        if val != expected_val:
            raise InputError(f"{error_msg}\nERROR: The value should be: {expected_val}, get: {val} instead")


class SchemaValidator(abc.ABC):

    def __init__(self, is_optional: bool):
        self.is_optional = is_optional

    @abc.abstractmethod
    def validate(self, value):
        raise NotImplementedError()

    @abc.abstractmethod
    def raise_error(self, value):
        raise NotImplementedError()

    @abc.abstractmethod
    def to_string(self):
        raise NotImplementedError()


class DictValidator(SchemaValidator):
    REG_PT = re.compile(r"^int|str|float|any$")
    REG_OPTIONAL = re.compile(r"^optional\((.+)\)$")
    REG_LIST = re.compile(r"^list\((.+)\)$")

    def __init__(self, cls: str, is_optional: bool, **kwargs):
        super().__init__(is_optional)
        self.cls = cls
        self.attrs: Dict[str, SchemaValidator] = {}
        for kw, arg in kwargs.items():
            if isinstance(arg, str):
                m = self.REG_OPTIONAL.match(arg)
                is_optional = False
                if m is not None:
                    is_optional = True
                    arg = m.group(1)

                m = self.REG_LIST.match(arg)
                is_list = False
                if m is not None:
                    is_list = True
                    arg = m.group(1)

                if is_list:
                    m = self.REG_OPTIONAL.match(arg)
                    is_elem_optional = False
                    if m is not None:
                        is_elem_optional = True
                        arg = m.group(1)

                    if arg == "any":
                        self.attrs[kw] = ListValidator(AnyValidator(False), is_optional)
                    else:
                        self.attrs[kw] = ListValidator(
                            PrimitiveValidator(arg, is_elem_optional), is_optional)
                else:
                    if arg == "any":
                        self.attrs[kw] = AnyValidator(False)
                    else:
                        self.attrs[kw] = PrimitiveValidator(arg, is_optional)
            else:
                self.attrs[kw] = arg

        self.attr_names = set(self.attrs.keys())

    def validate(self, odict):
        if self.is_optional and odict is None:
            return

        if not isinstance(odict, (dict, OrderedDict)):
            self.raise_error(odict)

        if not self.attr_names.issuperset(odict.keys()):
            self.raise_error(odict)

        for name, attr in self.attrs.items():
            if name not in odict:
                if not attr.is_optional:
                    self.raise_error(odict)
            else:
                try:
                    attr.validate(odict[name])
                except InputError:
                    self.raise_error(odict)

    def raise_error(self, odict):
        raise InputError(
            f"The schema of object: {odict} does not match with the desired schema: {self.to_string()}"
        )

    def to_string(self):
        return ujson.dumps({k: v.to_string() for k, v in self.attrs.items()}, indent=4)


class AnyValidator(SchemaValidator):

    def validate(self, value):
        pass

    def raise_error(self, value):
        pass

    def to_string(self):
        return "any"


class PrimitiveValidator(SchemaValidator):
    def __init__(self, type_name: str, is_optional: bool):
        super().__init__(is_optional)
        self.type_name = type_name

        if type_name == "str":
            self.type_value = str
        elif type_name == "int":
            self.type_value = int
        elif type_name == "float":
            self.type_value = float
        else:
            raise Exception("Unreachable!")

    def validate(self, value):
        if self.is_optional and value is None:
            return

        if not isinstance(value, self.type_value):
            self.raise_error(value)

    def raise_error(self, odict):
        raise InputError(
            f"The schema of object: {odict} does not match with the desired schema: {self.to_string()}"
        )

    def to_string(self):
        if self.is_optional:
            return f"optional({self.type_name})"
        return self.type_name


class ListValidator(SchemaValidator):
    def __init__(self, element_type: SchemaValidator, is_optional: bool):
        super().__init__(is_optional)
        self.element_type = element_type

    def validate(self, value):
        if self.is_optional and value is None:
            return

        if not isinstance(value, list):
            self.raise_error(value)

        try:
            for v in value:
                self.element_type.validate(v)
        except InputError:
            self.raise_error(value)

    def raise_error(self, odict):
        raise InputError(
            f"The schema of object: {odict} does not match with the desired schema: {self.to_string()}"
        )

    def to_string(self):
        if self.is_optional:
            return f"optional(list({self.element_type.to_string()}))"
        return f"list({self.element_type.to_string()})"
