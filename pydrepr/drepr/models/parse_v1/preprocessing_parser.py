from typing import List

from drepr.utils.validator import Validator
from .path_parser import PathParser
from ..preprocessing import Preprocessing, PreprocessingType, PMap, PFilter, RMap


class PreprocessingParser:
    """
    Preprocessing are defined as a list

    ```
    - type: <preprocessing_type>
      # other properties
    ```

    1. If <preprocessing_type> is `pmap`, its other properties are:

        ```
        - type: pmap
          [resource_id]: <resource_id>
          path: <path>
          [output]: <resource_id> (default is None)
          [change_structure]: null|true|false (default is null)
          code: str
        ```

    2. If <preprocessing_type> is `pfilter`, its properties are:

        ```
        - type: pfilter
          [resource_id]: <resource_id>
          path: <path>
          [output]: <resource_id> (default is None)
          code: str
        ```

    3. If <preprocessing_type> is `rmap`, its properties are:

        ```
        - type: rmap
          resource_id: <resource_id>
          path: <path>
          func_id: <func_id>
          [output]: <resource_id> (default is None)
        ```
    """
    PREPRO_TYPES = {x.value for x in PreprocessingType}

    @classmethod
    def parse(cls, default_resource_id: str, conf: list) -> List[Preprocessing]:
        Validator.must_be_list(conf, "Parsing preprocessing")
        result = []

        for i, prepro in enumerate(conf):
            trace0 = f"Parsing preprocessing at position {i}"
            Validator.must_be_dict(prepro, trace0)
            Validator.must_have(prepro, "type", trace0)
            Validator.must_in(prepro['type'], cls.PREPRO_TYPES, f"{trace0}\nParsing property `type`")

            prepro_type = PreprocessingType(prepro['type'])
            if prepro_type == PreprocessingType.pmap:
                value = cls.parse_pmap(default_resource_id, prepro, trace0)
            elif prepro_type == PreprocessingType.pfilter:
                value = cls.parse_pfilter(default_resource_id, prepro, trace0)
            elif prepro_type == PreprocessingType.rmap:
                value = cls.parse_rmap(default_resource_id, prepro, trace0)
            else:
                raise NotImplemented(f"Not implement the parser for preprocessing function with type {prepro_type}")

            result.append(Preprocessing(prepro_type, value))
        return result

    @classmethod
    def parse_pmap(cls, default_resource_id: str, prepro: dict, trace0: str) -> PMap:
        if "resource_id" in prepro:
            Validator.must_be_str(prepro["resource_id"], f"{trace0}\nParsing property `resource_id`")
            resource_id = prepro['resource_id']
        else:
            resource_id = default_resource_id

        trace1 = f"{trace0}\nParsing property `path`"
        Validator.must_have(prepro, "path", trace1)
        path = PathParser.parse(prepro['path'], trace1)

        trace1 = f"{trace0}\nParsing property `code`"
        Validator.must_have(prepro, "code", trace1)
        Validator.must_be_str(prepro["code"], trace1)
        code = prepro["code"]

        if "output" in prepro and prepro['output'] is not None:
            trace1 = f"{trace0}\nParsing property `output`"
            Validator.must_be_str(prepro["output"], trace1)
            output = prepro["output"]
        else:
            output = None

        if "change_structure" in prepro and prepro["change_structure"] is not None:
            trace1 = f"{trace0}\nParsing property `change_structure`"
            Validator.must_be_bool(prepro["change_structure"], trace1)
            change_structure = prepro["change_structure"]
        else:
            change_structure = None

        return PMap(resource_id, path, code, output, change_structure)

    @classmethod
    def parse_pfilter(cls, default_resource_id: str, prepro: dict, trace0: str) -> PFilter:
        if "resource_id" in prepro:
            Validator.must_be_str(prepro["resource_id"], f"{trace0}\nParsing property `resource_id`")
            resource_id = prepro['resource_id']
        else:
            resource_id = default_resource_id

        trace1 = f"{trace0}\nParsing property `path`"
        Validator.must_have(prepro, "path", trace1)
        path = PathParser.parse(prepro['path'], trace1)

        trace1 = f"{trace0}\nParsing property `code`"
        Validator.must_have(prepro, "code", trace1)
        Validator.must_be_str(prepro["code"], trace1)
        code = prepro["code"]

        if "output" in prepro:
            trace1 = f"{trace0}\nParsing property `output`"
            Validator.must_be_str(prepro["output"], trace1)
            output = prepro["output"]
        else:
            output = None

        return PFilter(resource_id, path, code, output)

    @classmethod
    def parse_rmap(cls, default_resource_id: str, prepro: dict, trace0: str) -> RMap:
        if "resource_id" in prepro:
            Validator.must_be_str(prepro["resource_id"], f"{trace0}\nParsing property `resource_id`")
            resource_id = prepro['resource_id']
        else:
            resource_id = default_resource_id

        trace1 = f"{trace0}\nParsing property `path`"
        Validator.must_have(prepro, "path", trace1)
        path = PathParser.parse(prepro['path'], trace1)

        trace1 = f"{trace0}\nParsing property `func_id`"
        Validator.must_have(prepro, "func_id", trace1)
        Validator.must_be_str(prepro["func_id"], trace1)
        func_id = prepro["func_id"]

        if "output" in prepro:
            trace1 = f"{trace0}\nParsing property `output`"
            Validator.must_be_str(prepro["output"], trace1)
            output = prepro["output"]
        else:
            output = None

        return RMap(resource_id, path, func_id, output)
