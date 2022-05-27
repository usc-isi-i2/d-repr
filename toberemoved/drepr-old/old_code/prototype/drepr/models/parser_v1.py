import re
from typing import Any, Type, List, Dict, Optional

from drepr.misc.dict2instance import Dict2InstanceDeSer, get_object_map_deser, get_object_list_deser, \
    get_object_enum_deser, DeSer, get_str2str_deser, get_str_deser
from drepr.models import InvalidReprException, DimensionMapping, ResourceDeSer, Variable, SemanticModel, ClassID, \
    SemanticType, SemanticRelation, Resource, IndexSlice, DynamicRangeSlice, RangeSlice, DynamicIndexSlice, Location, \
    OntNS
from drepr.models.preprocessing import TransformFunc


class SliceReadableDeSerV1(DeSer):
    regex = re.compile(
        r"^(?:(\d+|(?:\${[^}]+}))?\.\.(\d+|(?:\${[^}]+}))?(?:;(\d+|(?:\${[^}]+})))?)|(\d+|(?:\${[^}]+}))|([a-zA-Z][a-zA-Z0-9]*)$"
    )
    range_regex = re.compile(
        r"^(?:(\d+|(?:\${[^}]+}))?\.\.(\d+|(?:\${[^}]+}))?(?:;(\d+|(?:\${[^}]+})))?)$")
    index_regex = re.compile(r"^(\d+|(?:\${[^}]+})|(?:[a-zA-Z][a-zA-Z0-9]*))$")

    @staticmethod
    def regex_group_get_int(match, group_idx: int, default: Optional[int]):
        if match.group(group_idx) is None:
            return default

        if match.group(group_idx).startswith("${"):
            return match.group(group_idx)

        return int(match.group(group_idx))

    @classmethod
    def unsafe_deserialize(cls, o: Any):
        if isinstance(o, int):
            return IndexSlice(o)

        if not isinstance(o, str):
            raise InvalidReprException(f"Invalid object type, expect string or int")

        match = SliceReadableDeSerV1.range_regex.match(o)
        if match is not None:
            if any(match.group(i).startswith("${") for i in range(1, 4) if match.group(i) is not None):
                return DynamicRangeSlice(
                    SliceReadableDeSerV1.regex_group_get_int(match, 1, 0),
                    SliceReadableDeSerV1.regex_group_get_int(match, 2, None),
                    SliceReadableDeSerV1.regex_group_get_int(match, 3, 1))

            return RangeSlice(
                SliceReadableDeSerV1.regex_group_get_int(match, 1, 0),
                SliceReadableDeSerV1.regex_group_get_int(match, 2, None),
                SliceReadableDeSerV1.regex_group_get_int(match, 3, 1))

        match = SliceReadableDeSerV1.index_regex.match(o)
        if match is not None:
            idx = match.group(1)
            if idx.startswith("${"):
                return DynamicIndexSlice(idx)
            return IndexSlice(int(idx) if idx.isdigit() else idx)

        raise InvalidReprException(f"Invalid slice {o}")


class LocationReadableDeSerV1(Dict2InstanceDeSer):
    DeserializeErrorClass = InvalidReprException
    class_properties = {
        "resource_id": get_str_deser("resource_id", InvalidReprException),
        "slices": get_object_list_deser("slices", SliceReadableDeSerV1)
    }

    def __init__(self, resource_id, slices):
        self.resource_id = resource_id
        self.slices = slices

    def to_loc(self) -> Location:
        return Location(self.resource_id, self.slices)

    def to_raw_loc(self) -> dict:
        return Location(self.resource_id, self.slices).serialize()


class ResourceReadableDeSerV1(DeSer):
    @classmethod
    def unsafe_deserialize(cls, json_obj: dict):
        if isinstance(json_obj, str):
            json_obj = {'default': json_obj}

        if not isinstance(json_obj, dict):
            raise InvalidReprException("Expect resources to be a map")

        resources = {}

        for resource_id, resource_type in json_obj.items():
            if isinstance(resource_type, str):
                o = {'id': resource_id, 'type': resource_type}
            elif isinstance(resource_type, dict):
                o = dict(resource_type.items())
                o['id'] = resource_id
            else:
                raise InvalidReprException(
                    "Invalid resource type, expect to be a string or dictionary")

            resources[resource_id] = ResourceDeSer.unsafe_deserialize(o)
        return resources


class LayoutReadableDeSerV1(DeSer):
    @classmethod
    def unsafe_deserialize(cls, o: Any):
        if not isinstance(o, dict):
            raise InvalidReprException("Expect map for layout")

        results = {}
        for var_id, var_conf in o.items():
            if isinstance(var_conf, str):
                var_conf = {"location": var_conf}
            elif not isinstance(var_conf, dict):
                raise InvalidReprException("Expect map for variable")

            var_conf['id'] = var_id

            if 'location' not in var_conf:
                raise InvalidReprException("Missing location of variable")

            if isinstance(var_conf['location'], str):
                try:
                    if '@' in var_conf['location']:
                        resource_id, slices = var_conf['location'].split("@")
                    else:
                        resource_id = "--null--"
                        slices = var_conf['location']
                except:
                    raise InvalidReprException("Invalid location string")
                var_conf['location'] = LocationReadableDeSerV1.unsafe_deserialize({
                    "resource_id": resource_id,
                    "slices": slices.split(":")
                }).to_raw_loc()
            results[var_id] = Variable.unsafe_deserialize(var_conf)
        return results


class SemanticTypeReadableDeSerV1(DeSer):
    stype_reg = re.compile(r"^((?:.(?!--))*.)--((?:.(?!\^\^))*[^\^])(?:\^\^(.+))?$")

    @classmethod
    def unsafe_deserialize(cls: Type['DeSer'], raw_repr: Any):
        try:
            if isinstance(raw_repr, str):
                match = SemanticTypeReadableDeSerV1.stype_reg.match(raw_repr)
                if match is None:
                    raise InvalidReprException(f"Invalid semantic type: {raw_repr}")
                node_id, predicate, value_type = match.group(1), match.group(2), match.group(3)
                node_id = ClassID.create(node_id)

                return SemanticType(node_id, node_id.uri, predicate, value_type)
        except KeyError as e:
            raise InvalidReprException(f"Missing property: {e} in semantic_type")

        return SemanticType.unsafe_deserialize(raw_repr)


class SemanticRelationReadableDeSerV1(DeSer):
    @classmethod
    def unsafe_deserialize(cls: Type['DeSer'], obj: Any):
        if isinstance(obj, str):
            try:
                s, p, o = obj.split("--")
                s = ClassID.create(s)
                o = ClassID.create(o)
                return SemanticRelation(s, p, o)
            except:
                raise InvalidReprException(f"Invalid semantic relation: {obj}")

        return SemanticRelation.unsafe_deserialize(obj)


class DimensionMappingReadableDeSerV1(DeSer):
    dim_map_pattern = re.compile(r"^([^:]+):(\d+) *<-> *([^:]+):(\d+)$")

    @classmethod
    def unsafe_deserialize(cls, o: Any):
        if 'value' in o:
            match = DimensionMappingReadableDeSerV1.dim_map_pattern.match(o['value'])
            if match is None:
                raise InvalidReprException(f"Invalid one to one mapping: {o['value']}")

            return DimensionMapping(
                match.group(1), [int(match.group(2))], match.group(3), [int(match.group(4))])
        return DimensionMapping.unsafe_deserialize(o)


class SemanticModelReadableDeSerV1(Dict2InstanceDeSer):
    DeserializeErrorClass = InvalidReprException
    class_properties = {
        "semantic_types": get_object_map_deser("semantic_types", SemanticTypeReadableDeSerV1),
        "semantic_relations": get_object_list_deser("semantic_relations",
                                                    SemanticRelationReadableDeSerV1),
        "ontology_prefixes": get_str2str_deser("ontology_prefixes", InvalidReprException)
    }
    class_rename_props = SemanticModel.class_rename_props

    def __init__(self, semantic_types: Dict[str, SemanticType], semantic_relations: List[SemanticRelation], ont_prefixes):
        ont = OntNS(ont_prefixes)

        for stype in semantic_types.values():
            stype.set_predicate_uri(ont.full_uri(stype.predicate_uri))
            stype.set_class_uri(ont.full_uri(stype.class_uri))
            stype.node_id.set_uri(stype.class_uri)

        for stype in semantic_relations:
            stype.source.set_uri(ont.full_uri(stype.source.uri))
            stype.target.set_uri(ont.full_uri(stype.target.uri))
            stype.predicate = ont.full_uri(stype.predicate)

        self.args = {
            "semantic_types": semantic_types,
            "semantic_relations": semantic_relations,
            "ont_prefixes": ont_prefixes,
        }

    @classmethod
    def unsafe_deserialize(cls, json_obj: dict):
        o = super(SemanticModelReadableDeSerV1, cls).unsafe_deserialize(json_obj)
        return SemanticModel(**o.args)


class TransformFuncReadableDeSerV1(DeSer):
    @classmethod
    def unsafe_deserialize(cls, o: Any):
        if 'location' in o:
            if '@' in o['location']:
                resource_id, slices = o['location'].split("@")
            else:
                resource_id = "--null--"
                slices = o['location']

            o['location'] = LocationReadableDeSerV1.unsafe_deserialize(
                {'resource_id': resource_id, 'slices': slices.split(":")}).to_raw_loc()

        return TransformFunc.unsafe_deserialize(o)


class ReprReadableDeSerV1(Dict2InstanceDeSer):
    DeserializeErrorClass = InvalidReprException
    class_properties = {
        "resources": ResourceReadableDeSerV1,
        "preprocessing": get_object_list_deser("preprocessing", TransformFuncReadableDeSerV1),
        "layout": LayoutReadableDeSerV1,
        "mappings": get_object_list_deser(
            "mappings",
            get_object_enum_deser({
                "dimension_mapping": DimensionMappingReadableDeSerV1,
            })),
        "semantic_model": SemanticModelReadableDeSerV1,
    }
    class_rename_props = {"layout": "variables"}

    def __init__(self, resources: Dict[str, Resource], variables: Dict[str, Variable],
                 preprocessing: List[TransformFunc], mappings, semantic_model):
        if len(resources) == 1:
            resource_id = next(iter(resources.keys()))
            if resource_id != "--null--":
                # "--null--" is a placeholder for None so that we can set resource_id correctly, but if
                # someone create resource with "--null--", we don't need to do anything
                for func in preprocessing:
                    if func.location is not None and func.location.resource_id == "--null--":
                        func.location.resource_id = resource_id

                for var in variables.values():
                    if var.location.resource_id == "--null--":
                        var.location.resource_id = resource_id

        self.args = {
            "resources": resources,
            "variables": variables,
            "preprocessing": preprocessing,
            "mappings": mappings,
            "semantic_model": semantic_model
        }
