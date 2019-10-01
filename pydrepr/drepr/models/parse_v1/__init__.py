from drepr.utils.validator import *
from .align_parser import AlignParser
from .attr_parser import AttrParser
from .preprocessing_parser import PreprocessingParser
from .resource_parser import ResourceParser
from .sm_parser import SMParser
from ..sm import SemanticModel


class ReprV1Parser:
    """
    The DREPR language version 1 has the following schema:

    ```
    version: '1'
    resources: <resources>
    [preprocessing]: <preprocessing> (default is empty list)
    attributes: <attributes>
    [alignments]: <alignments> (default is empty list)
    semantic_model: <semantic_model>
    ```
    """

    TOP_KEYWORDS = {"version", "resources", "preprocessing", "attributes", "alignments", "semantic_model"}
    DEFAULT_RESOURCE_ID = "default"

    @classmethod
    def parse(cls, raw: dict):
        from ..drepr import DRepr

        Validator.must_be_subset(cls.TOP_KEYWORDS, raw.keys(),
                                 setname="Keys of D-REPR configuration",
                                 error_msg="Parsing D-REPR configuration")

        for prop in ['version', 'resources', 'attributes']:
            Validator.must_have(raw, prop, error_msg="Parsing D-REPR configuration")

        Validator.must_equal(raw['version'], '1', "Parsing D-REPR configuration version")
        resources = ResourceParser.parse(raw['resources'])

        if len(resources) == 1:
            default_resource_id = resources[0].id
        else:
            default_resource_id = ResourceParser.DEFAULT_RESOURCE_ID

        preprocessing = PreprocessingParser.parse(default_resource_id, raw.get('preprocessing', []))
        attrs = AttrParser.parse(default_resource_id, raw['attributes'])
        aligns = AlignParser.parse(raw.get('alignments', []))

        if 'semantic_model' in raw:
            sm = SMParser.parse(raw['semantic_model'])
            sm.prefixes.update(SemanticModel.get_default_prefixes())
        else:
            sm = None

        return DRepr(resources, preprocessing, attrs, aligns, sm)
