#!/usr/bin/python
# -*- coding: utf-8 -*-
from enum import Enum
from typing import *

T = TypeVar("T", bound="Parent")


class DeSer:
    DeserializeErrorClass = KeyError

    @staticmethod
    def serialize_func(object) -> Any:
        return object

    @classmethod
    def deserialize(cls, o: Any) -> Any:
        return o

    @classmethod
    def unsafe_deserialize(cls, o: Any) -> Any:
        return o


class Dict2InstanceDeSer(DeSer):
    # storing properties and its 3 deser functions: safe-deser (fast), unsafe-deser (slow), and ser
    class_properties: Dict[str, Type[DeSer]] = {}
    # if it has different names it can be rename here
    class_rename_props: Dict[str, str] = {}
    # use to check possible values of properties
    class_property_possible_values: Dict[str, Set[Any]] = {}
    # default values in case it is missing
    class_property_default_values: Dict[str, Any] = {}

    def serialize(self) -> dict:
        return self.serialize_func(self)

    @staticmethod
    def serialize_func(self: 'Dict2InstanceDeSer'):
        json = {}
        for prop, DeserClass in self.class_properties.items():
            json[prop] = DeserClass.serialize_func(self.__dict__[self.class_rename_props.get(
                prop, prop)])
        return json

    @classmethod
    def deserialize(cls, json_obj: dict):
        """Quickly deserialize the data, assuming that the data is incorrect format"""
        props = {}
        for prop, DeSerClass in cls.class_properties.items():
            if prop not in json_obj:
                try:
                    props[cls.class_rename_props.get(prop, prop)] = cls.class_property_default_values[prop]
                except KeyError:
                    raise cls.DeserializeErrorClass(f"Invalid data. Missing value for key {prop} in {cls.__name__}")
            else:
                props[cls.class_rename_props.get(prop, prop)] = DeSerClass.deserialize(json_obj[prop])

        # noinspection PyArgumentList
        instance = cls(**props)
        cls.post_deserialize(instance)
        return instance

    @classmethod
    def unsafe_deserialize(cls, json_obj: dict):
        """Deserialize the object on the data and check every possible errors in the json_obj"""
        if not isinstance(json_obj, dict):
            raise cls.DeserializeErrorClass(f"Invalid data. Expecting a map in {cls.__name__}")

        extra_keys = [key for key in json_obj.keys() if key not in cls.class_properties]
        if len(extra_keys) > 0:
            raise cls.DeserializeErrorClass(
                f'Having extra properties: [{",".join(extra_keys)}] in {cls.__name__}')

        try:
            props = {}
            for prop, DeSerClass in cls.class_properties.items():
                if prop not in json_obj and prop in cls.class_property_default_values:
                    val = cls.class_property_default_values[prop]
                else:
                    val = cls.check_deser_value(prop, DeSerClass.unsafe_deserialize(json_obj[prop]))

                props[cls.class_rename_props.get(prop, prop)] = val

            # noinspection PyArgumentList
            instance = cls(**props)
            cls.post_deserialize(instance)
            return instance
        except KeyError as e:
            raise cls.DeserializeErrorClass(f"Missing property: {e} in {cls.__name__}")

    @classmethod
    def post_deserialize(cls, instance) -> None:
        pass

    @classmethod
    def check_deser_value(cls, prop: str, val):
        if prop in cls.class_property_possible_values and val not in cls.class_property_possible_values[
                prop]:
            raise cls.DeserializeErrorClass(f"Invalid value for property: {prop} in {cls.__name__}")

        return val


def get_bool_deser(prop: str, ErrorClass: Type[Exception] = KeyError):
    class BoolDeSer(DeSer):
        @classmethod
        def unsafe_deserialize(cls, o: Any):
            if not isinstance(o, bool):
                raise ErrorClass(f"Expect bool for property: {prop}")
            return o

    return BoolDeSer


def get_int_deser(prop: str, nullable: bool=False, ErrorClass: Type[Exception] = KeyError):
    class IntDeSer(DeSer):
        @classmethod
        def unsafe_deserialize(cls, o: Any):
            if not isinstance(o, int):
                raise ErrorClass(f"Expect int for property: {prop}")
            return o

    class NullableIntDeSer(DeSer):
        @classmethod
        def unsafe_deserialize(cls, o: Any):
            if o is not None and not isinstance(o, int):
                raise ErrorClass(f"Expect int for property: {prop}")
            return o

    return NullableIntDeSer if nullable else IntDeSer


def get_str_deser(prop: str, nullable: bool=False, ErrorClass: Type[Exception] = KeyError):
    class StrDeSer(DeSer):
        @classmethod
        def unsafe_deserialize(cls, obj: Any):
            if not isinstance(obj, str):
                raise ErrorClass(f"Expect str for property: {prop}")

            return obj

    class NullableStrDeSer(DeSer):
        @classmethod
        def unsafe_deserialize(cls, obj: Any):
            if obj is not None and not isinstance(obj, str):
                raise ErrorClass(f"Expect str for property: {prop}")

            return obj

    return NullableStrDeSer if nullable else StrDeSer


def get_primitive_deser(prop: str, primitive_types: Tuple[type, ...], ErrorClass: Type[Exception] = KeyError):
    primitive_types_str = ", ".join([
        str(t)[str(t).find("'") + 1:-2]
        for t in primitive_types
    ])

    class PrimitiveDeSer(DeSer):
        @classmethod
        def unsafe_deserialize(cls, o: Any):
            if not isinstance(o, primitive_types):
                raise ErrorClass(f"Expect {primitive_types_str} for property: {prop}")
            return o

    return PrimitiveDeSer


def get_union_deser(types: Callable[[Any], Type[DeSer]]):
    class UnionDeSer(DeSer):
        @classmethod
        def deserialize(cls, o: Any):
            DeSerClass = types(o)
            return DeSerClass.deserialize(o)

        @classmethod
        def unsafe_deserialize(cls, o: Any):
            DeSerClass = types(o)
            return DeSerClass.unsafe_deserialize(o)

    return UnionDeSer


def get_list_int_deser(prop: str, ErrorClass: Type[Exception] = KeyError):
    class ListIntDeSer(DeSer):
        @classmethod
        def unsafe_deserialize(cls, obj: Any):
            if not isinstance(obj, list):
                raise ErrorClass(f"Expect list for property: {prop}")

            for value in obj:
                if not isinstance(value, int):
                    raise ErrorClass(f"Expect items of property: {prop} should have type integer")

            return obj

    return ListIntDeSer


def get_set_deser(prop: str, ErrorClass: Type[Exception] = KeyError):
    class SetDeSer(DeSer):
        @classmethod
        def deserialize(cls, obj: List[Any]):
            return set(obj)

        @classmethod
        def unsafe_deserialize(cls, obj: Any):
            if not isinstance(obj, list):
                raise ErrorClass(f"Expect list for property: {prop}")

            for value in obj:
                if not isinstance(value, (int, float, str)):
                    raise ErrorClass(f"Expect items of property: {prop} should be primitive type")

            return set(obj)

        @staticmethod
        def serialize_func(object: Set[Any]):
            return list(object)

    return SetDeSer


def get_enum_deser(prop: str, EnumClass: Type[Enum], ErrorClass: Type[Exception] = KeyError):
    possible_values = {x.value for x in EnumClass}

    class EnumDeSer(DeSer):
        @classmethod
        def deserialize(cls, o: Any):
            return EnumClass(o)

        @classmethod
        def unsafe_deserialize(cls, o: Any):
            if o not in possible_values:
                raise ErrorClass(f"Invalid value for property: {prop}")
            return EnumClass(o)

        @staticmethod
        def serialize_func(object):
            return object.value

    return EnumDeSer


def get_set_int_deser(prop: str, ErrorClass: Type[Exception] = KeyError):
    class SetIntDeSer(DeSer):
        @classmethod
        def deserialize(cls, obj: Any):
            return set(obj)

        @classmethod
        def unsafe_deserialize(cls, obj: Any):
            if not isinstance(obj, list):
                raise ErrorClass(f"Expect list for property: {prop}")

            for value in obj:
                if not isinstance(value, int):
                    raise ErrorClass(f"Expect items of property: {prop} should have type integer")

            return set(obj)

    return SetIntDeSer


def get_str2str_deser(prop: str, ErrorClass: Type[Exception] = KeyError):
    class Str2StrDeSer(DeSer):
        @classmethod
        def unsafe_deserialize(cls, obj: Any):
            if not isinstance(obj, dict):
                raise ErrorClass(f"Invalid data. Expecting a map for {prop}")
            for key, val in obj.items():
                if not isinstance(val, str):
                    raise ErrorClass(f"Expect a string for property: {key} in {prop}")

            return obj

    return Str2StrDeSer


def get_object_list_deser(prop: str, ObjectClass: Type[DeSer], nullable: bool = False):
    class ObjectListDeSer(DeSer):
        @classmethod
        def deserialize(cls, obj: list):
            return [ObjectClass.deserialize(v) for v in obj]

        @classmethod
        def unsafe_deserialize(cls, obj: Any):
            if not isinstance(obj, list):
                raise ObjectClass.DeserializeErrorClass(
                    f"Invalid data. Expecting a list for {prop}")
            return [ObjectClass.unsafe_deserialize(v) for v in obj]

        @staticmethod
        def serialize_func(self: List[DeSer]):
            return [ObjectClass.serialize_func(v) for v in self]

    class ObjectListNullableDeSer(DeSer):
        @classmethod
        def deserialize(cls, obj: list):
            return [ObjectClass.deserialize(v) if v is not None else v for v in obj]

        @classmethod
        def unsafe_deserialize(cls, obj: Any):
            if not isinstance(obj, list):
                raise ObjectClass.DeserializeErrorClass(
                    f"Invalid data. Expecting a list for {prop}")
            return [ObjectClass.unsafe_deserialize(v) if v is not None else v for v in obj]

        @staticmethod
        def serialize_func(self: List[Optional[DeSer]]):
            return [ObjectClass.serialize_func(v) if v is not None else v for v in self]

    return ObjectListNullableDeSer if nullable else ObjectListDeSer


def get_object_map_deser(prop: str,
                         ObjectClass: Type[DeSer],
                         nullable: bool = False,
                         add_key_as_prop: Optional[str] = None):
    def add_key_and_return(d: dict, key: str, value: Any):
        d[key] = value
        return d

    class ObjectMapDeSer(DeSer):
        @classmethod
        def deserialize(cls, obj: dict):
            return {k: ObjectClass.deserialize(v) for k, v in obj.items()}

        @classmethod
        def unsafe_deserialize(cls, obj: Any):
            if not isinstance(obj, dict):
                raise ObjectClass.DeserializeErrorClass(f"Invalid data. Expecting a map for {prop}")
            return {k: ObjectClass.unsafe_deserialize(v) for k, v in obj.items()}

        @staticmethod
        def serialize_func(self: Dict[str, DeSer]):
            return {k: ObjectClass.serialize_func(v) for k, v in self.items()}

    class ObjectMapNullableDeSer(DeSer):
        @classmethod
        def deserialize(cls, obj: dict):
            return {
                k: ObjectClass.deserialize(v) if v is not None else None
                for k, v in obj.items()
            }

        @classmethod
        def unsafe_deserialize(cls, obj: Any):
            if not isinstance(obj, dict):
                raise ObjectClass.DeserializeErrorClass(f"Invalid data. Expecting a map for {prop}")
            return {
                k: ObjectClass.unsafe_deserialize(v) if v is not None else None
                for k, v in obj.items()
            }

        @staticmethod
        def serialize_func(self: Dict[str, DeSer]):
            return {
                k: ObjectClass.serialize_func(v) if v is not None else None
                for k, v in self.items()
            }

    class ObjectMapAddKeyDeSer(DeSer):
        @classmethod
        def deserialize(cls, obj: dict):
            return {
                k: ObjectClass.deserialize(add_key_and_return(v, add_key_as_prop, k))
                for k, v in obj.items()
            }

        @classmethod
        def unsafe_deserialize(cls, obj: Any):
            if not isinstance(obj, dict):
                raise ObjectClass.DeserializeErrorClass(f"Invalid data. Expecting a map for {prop}")
            return {
                k: ObjectClass.unsafe_deserialize(add_key_and_return(v, add_key_as_prop, k))
                for k, v in obj.items()
            }

        @staticmethod
        def serialize_func(self: Dict[str, DeSer]):
            return {k: ObjectClass.serialize_func(v) for k, v in self.items()}

    class ObjectMapNullableAddKeyDeSer(DeSer):
        @classmethod
        def deserialize(cls, obj: dict):
            return {
                k: ObjectClass.deserialize(add_key_and_return(v, add_key_as_prop, k))
                if v is not None else None
                for k, v in obj.items()
            }

        @classmethod
        def unsafe_deserialize(cls, obj: Any):
            if not isinstance(obj, dict):
                raise ObjectClass.DeserializeErrorClass(f"Invalid data. Expecting a map for {prop}")
            return {
                k: ObjectClass.unsafe_deserialize(add_key_and_return(v, add_key_as_prop, k))
                if v is not None else None
                for k, v in obj.items()
            }

        @staticmethod
        def serialize_func(self: Dict[str, DeSer]):
            return {
                k: ObjectClass.serialize_func(v) if v is not None else None
                for k, v in self.items()
            }

    if add_key_as_prop is None:
        return ObjectMapNullableDeSer if nullable else ObjectMapDeSer
    else:
        return ObjectMapNullableAddKeyDeSer if nullable else ObjectMapAddKeyDeSer


def get_object_enum_deser(variants: Dict[str, Type[DeSer]], variant_prop_name: str = "type"):
    variant2types = {v: k for k, v in variants.items()}

    class EnumDeSer(Dict2InstanceDeSer):
        DeserializeErrorClass = KeyError

        _variant_prop_name_: str = "type"
        _variants_: Dict[str, Type[DeSer]] = {}

        def serialize(self) -> dict:
            return self.serialize_func(self)

        @staticmethod
        def serialize_func(self: 'DeSer'):
            # noinspection PyUnresolvedReferences
            obj = self._origin_serialize_func_(self)
            obj[variant_prop_name] = variant2types[self.__class__]
            return obj

        @classmethod
        def deserialize(cls, o: dict):
            return variants[o[variant_prop_name]].deserialize(o)

        @classmethod
        def unsafe_deserialize(cls, o: Any):
            if not isinstance(o, dict):
                raise cls.DeserializeErrorClass(f"Invalid data. Expecting a map in {cls.__name__}")

            variant = o.pop(variant_prop_name, None)
            if variant not in variants:
                raise cls.DeserializeErrorClass(
                    f"Invalid {variant_prop_name}: {variant}, it should be one of {','.join(variants.keys())}"
                )
            return variants[variant].unsafe_deserialize(o)

    # duck typing `serialize_func` to avoid recursive call
    for VariantClass in variant2types.keys():
        VariantClass._origin_serialize_func_ = staticmethod(VariantClass.serialize_func)
        VariantClass.serialize_func = staticmethod(EnumDeSer.serialize_func)

    return EnumDeSer
