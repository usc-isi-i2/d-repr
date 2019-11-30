from drepr.models.parse_v2.path_parser import PathParserV2
from drepr.utils.validator import *

from ..parse_v1.align_parser import AlignParser
from ..parse_v1.attr_parser import AttrParser
from ..parse_v1.preprocessing_parser import PreprocessingParser
from ..parse_v1.resource_parser import ResourceParser

from .sm_parser import SMParser
from ..sm import SemanticModel


class ReprV2Parser:
    """
    The D-REPR language version 2 has similar to the schema of the first version.

    Difference with previous features:
    1. For spreadsheet columns, they can the letter instead of number
    2. Semantic model configuration is changed to focus on classes
    """

    TOP_KEYWORDS = {
        "version", "resources", "preprocessing", "attributes", "alignments", "semantic_model"
    }
    DEFAULT_RESOURCE_ID = "default"

    @classmethod
    def parse(cls, raw: dict):
        from ..drepr import DRepr

        Validator.must_be_subset(cls.TOP_KEYWORDS,
                                 raw.keys(),
                                 setname="Keys of D-REPR configuration",
                                 error_msg="Parsing D-REPR configuration")

        for prop in ['version', 'resources', 'attributes']:
            Validator.must_have(raw, prop, error_msg="Parsing D-REPR configuration")

        Validator.must_equal(raw['version'], '2', "Parsing D-REPR configuration version")
        resources = ResourceParser.parse(raw['resources'])

        if len(resources) == 1:
            default_resource_id = resources[0].id
        else:
            default_resource_id = ResourceParser.DEFAULT_RESOURCE_ID

        path_parser = PathParserV2()
        preprocessing = PreprocessingParser(path_parser).parse(default_resource_id, resources,
                                                               raw.get('preprocessing', []))
        attrs = AttrParser(path_parser).parse(default_resource_id, resources, raw['attributes'])
        aligns = AlignParser.parse(raw.get('alignments', []))

        if 'semantic_model' in raw:
            sm = SMParser.parse(raw['semantic_model'])
            sm.prefixes.update(SemanticModel.get_default_prefixes())
        else:
            sm = None

        return DRepr(resources, preprocessing, attrs, aligns, sm)
