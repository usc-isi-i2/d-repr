import re
from typing import List

from drepr.models.align import AlignedStep, RangeAlignment, ValueAlignment, AlignmentType, Alignment
from drepr.utils.validator import Validator, InputError


class AlignParser:
    """
    Alignments are defined as a list

    ```
    - type: <alignment_type>
      # other properties
    ```

    If <alignment_type> is `dimension`, we have two schemas:
        1. align by one dimension

        ```
        - type: dimension
          value: <source_attr_id>:<step_index> <-> <target_attr_id>:<step_index>
        ```

        2. align by multiple dimensions

        ```
        - type: dimension
          source: <attr_id>
          target: <attr_id>
          aligned_dims:
            - { source: <step_index>, target: <step_index> }
            - { source: <step_index>, target: <step_index> }
            ...
        ```
    
    If <alignment_type> is `value`, then the schema is
        ```
        - type: value
          source: <attr_id>
          target: <attr_id>
        ```
    """
    ALIGNMENT_TYPES_VALUES = {x.value if x != AlignmentType.range else "dimension" for x in AlignmentType}
    REG_DALIGN = re.compile(r"^([^:]+)(?::(\d+))? *<-> *([^:]+)(?::(\d+))?$")
    DALIGN_KEYS = {"type", "source", "target", "aligned_dims"}
    DALIGN_ALIGNED_DIMS_KEYS = {"source", "target"}
    VALIGN_KEYS = {"type", "source", "target"}

    @classmethod
    def parse(cls, alignments: List[dict]) -> List[Alignment]:
        Validator.must_be_list(alignments, "Parsing alignments")
        aligns = []

        for i, align in enumerate(alignments):
            trace = f"Parsing an alignment at index {i}"
            Validator.must_be_dict(align, trace)
            Validator.must_have(align, "type", trace)

            align_type = AlignmentType(align["type"] if align["type"] != "dimension" else "range")
            if align_type == AlignmentType.range:
                aligns.append(cls.parse_range_align(align, trace))
            elif align_type == AlignmentType.value:
                aligns.append(cls.parse_value_align(align, trace))
            else:
                raise NotImplemented(
                    f"{trace}\nERROR: not implement parser for alignment type: {align_type}")

        return aligns

    @classmethod
    def parse_range_align(cls, conf: dict, parse_trace: str) -> RangeAlignment:
        if "value" in conf:
            Validator.must_be_str(conf["value"],
                                  f"{parse_trace}\nParsing property `value` of the alignment")
            m = cls.REG_DALIGN.match(conf["value"])
            if m is None:
                raise InputError(
                    f"{parse_trace}\nERROR: invalid `value` property, its format is invalid")

            source = m.group(1).strip()
            target = m.group(3).strip()
            if m.group(2) is None or m.group(4) is None:
                if not (m.group(2) is None and m.group(4) is None):
                    raise InputError(
                        f"{parse_trace}\nERROR: invalid `value` property, its format is invalid")
                return RangeAlignment(source, target, [])

            return RangeAlignment(source, target,
                                  [AlignedStep(int(m.group(2)), int(m.group(4)))])

        Validator.must_be_subset(cls.DALIGN_KEYS, conf.keys(), "properties of alignment",
                                 parse_trace)
        aligned_dims = []
        for i, adim in enumerate(conf['aligned_dims']):
            Validator.must_be_subset(cls.DALIGN_ALIGNED_DIMS_KEYS, adim.keys(),
                                     "properties of aligned_dims", parse_trace)
            for key in cls.DALIGN_ALIGNED_DIMS_KEYS:
                Validator.must_be_int(
                    adim[key],
                    f"{parse_trace}\nParsing property `{key}` of aligned_dim at position {i}")
            aligned_dims.append(AlignedStep(adim['source'], adim['target']))
        return RangeAlignment(conf["source"], conf["target"], aligned_dims)

    @classmethod
    def parse_value_align(cls, conf: dict, parse_trace: str) -> ValueAlignment:
        Validator.must_be_subset(cls.VALIGN_KEYS, conf.keys(), "properties of alignment",
                                 parse_trace)
        for key in cls.VALIGN_KEYS:
            Validator.must_be_str(conf[key],
                                  f"{parse_trace}\nParsing property `{key}` of value alignment")

        return ValueAlignment(conf["source"], conf["target"])
