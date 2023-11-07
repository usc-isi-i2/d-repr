from typing import List

from drepr.utils.validator import Validator, InputError

from ..resource import *


class ResourceParser:
    """
    `resources` has two possible schemas

    1. Shorthand when you have only one resource (`resource_id` is `default`): `resources: <resource_type>`
    2. When you have multiple resources:
        ```
        resources:
            <resource_id>: <resource_conf>
            # .. other resources ..
        ```

        The `<resource_conf>` can either be:
        a. `<resource_type>`, when other resource properties are all options
        b. a dictionary as follows, when some properties require to define explicitly
            ```
                <type>: <resource_type>
                # .. other attributes ..
            ```
    """
    DEFAULT_RESOURCE_ID = "default"
    RESOURCE_TYPES = {rtype.value for rtype in ResourceType}

    @classmethod
    def parse(cls, resources: Union[str, dict]) -> List[Resource]:
        if isinstance(resources, str):
            return cls._parse_schema1(resources)

        if isinstance(resources, dict):
            return cls._parse_schema2(resources)

        raise InputError(
            f"Invalid type for `resources`, expect either a string or a dictionary. Get {type(resources)} instead")

    @classmethod
    def _parse_schema1(cls, resource_type: str) -> List[Resource]:
        Validator.must_in(resource_type, cls.RESOURCE_TYPES, error_msg="Invalid resource type for `resources`")
        resource_type = ResourceType(resource_type)
        if resource_type == ResourceType.CSV:
            resource_prop = CSVProp()
        else:
            resource_prop = None

        return [Resource(cls.DEFAULT_RESOURCE_ID, resource_type, resource_prop)]

    @classmethod
    def _parse_schema2(cls, resources: dict) -> List[Resource]:
        result = []
        for resource_id, conf in resources.items():
            trace = f"Parsing resource {resource_id}."

            if isinstance(conf, str):
                resource = cls._parse_schema1(conf)[0]
                resource.id = resource_id
            elif isinstance(conf, dict):
                Validator.must_have(conf, "type", trace)
                Validator.must_in(conf['type'], cls.RESOURCE_TYPES, f"{trace}\n\tParsing resource type")
                resource_type = ResourceType(conf['type'])

                if resource_type == ResourceType.CSV:
                    if 'delimiter' in conf:
                        if len(conf['delimiter']) != 1:
                            raise InputError(f"{trace}.\nERROR: Expect one character delimiter "
                                             f"for CSV resource. Get `{conf['delimiter']}` instead")

                        resource_prop = CSVProp(conf['delimiter'])
                    else:
                        resource_prop = CSVProp()
                else:
                    resource_prop = None

                resource = Resource(resource_id, resource_type, resource_prop)
            else:
                raise InputError(f"{trace}.\nERROR: The configuration of a resource can either be string "
                                 f"or dictionary. Get {type(conf)} instead")

            result.append(resource)
        return result
