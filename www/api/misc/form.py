#!/usr/bin/python
# -*- coding: utf-8 -*-
import ujson
from enum import Enum
from typing import *

from flask import request

from api.misc.exception import BadRequest


class Validator:
    @staticmethod
    def always_valid(val: str):
        return True

    @staticmethod
    def is_not_empty(allow_whitespace: bool = False, nullable: bool = False):
        def validator(val: Optional[str]):
            if allow_whitespace:
                return val != ""
            if val is None:
                return nullable
            return val.strip() != ""

        return validator

    @staticmethod
    def is_string(val: str):
        return isinstance(val, str)

    @staticmethod
    def is_enum(enum_class: Type[Enum]):
        vals = {x.value for x in enum_class}

        def validator(val):
            return val in vals

        return validator

    @staticmethod
    def is_boolean(allow_string: bool = False):
        if allow_string:
            categories = {"true", "false", True, False}
        else:
            categories = {True, False}

        def validator(val):
            return val in categories

        return validator

    @staticmethod
    def is_in_list(categories: List[str]):
        categories = set(categories)

        def validator(val: str):
            return val in categories

        return validator

    @staticmethod
    def is_dict(val: dict):
        return isinstance(val, dict)

    @staticmethod
    def is_valid_form(form: 'Form', allow_string: bool = False):
        def validator(val):
            if not isinstance(val, dict):
                if allow_string:
                    try:
                        val = ujson.loads(val)
                    except ValueError:
                        return False
                else:
                    return False

            for field, validator in form.field2validator.items():
                if field not in val or not validator(val[field]):
                    return False

            return True

        return validator

    @staticmethod
    def is_list_form(form: "Form"):
        form_validator = Validator.is_valid_form(form)

        def validator(val: list):
            if not isinstance(val, list):
                return False
            for item in val:
                if not form_validator(item):
                    return False
            return True

        return validator


class Normalizer:
    @staticmethod
    def identity(allow_whitespace: bool = False):
        def norm(val: str):
            if not allow_whitespace and isinstance(val, str):
                return val.strip()
            return val

        return norm

    @staticmethod
    def form(form: 'Form'):
        def norm(val):
            if not isinstance(val, dict):
                val = ujson.loads(val)

            return {field: norm(val[field]) for field, norm in form.field2norm.items()}

        return norm

    @staticmethod
    def enum(enum_class: Type[Enum]):
        def norm(val):
            return enum_class(val)

        return norm


class Form:
    def __init__(self,
                 field2validator: Dict[str, Callable[[str], bool]],
                 field2norm: Dict[str, Callable[[str], Any]] = None):
        self.field2validator = field2validator
        self.field2norm = field2norm or {}

        for field, norm_func in field2validator.items():
            if field not in self.field2norm:
                self.field2norm[field] = Normalizer.identity()

    def post_form(self):
        return self.parse(request.form)

    def post_json_form(self):
        try:
            payload = ujson.loads(request.data)
        except ValueError:
            raise BadRequest(f"Invalid JSON")
        return self.parse(payload)

    def parse(self, payload):
        form = {}
        for field, validator in self.field2validator.items():
            if field not in payload:
                raise BadRequest(f"{field} is missing")

            if not validator(payload[field]):
                raise BadRequest(f"{field} is invalid")

            form[field] = self.field2norm[field](payload[field])

        return form
