from collections import OrderedDict
from pathlib import Path
from typing import List, Dict, Tuple, Callable, Any, Optional

from pydrepr import Repr, Graph


def test_normalize_func():
    repr = Repr.from_yml_string("""
version: '1'
resources: csv
variables:
  country: [1.., 0]
  year: [0, 1..]
  data: [1.., 1..]
alignments:
  - type: dimension
    value: data:0 <-> country:0
  - type: dimension
    value: data:1 <-> year:1
semantic_model:
  data_nodes:
    year: eg:Record:1--eg:year
    data: eg:Record:1--eg:data
    country: eg:Record:2--eg:country
  literal_nodes:
    - eg:Record:1--eg:owner--usc
  relations:
    - eg:Record:1--eg:location--eg:Record:2
  prefixes:
    eg: https://example.org
    """)

    repr.normalize_mut()
    assert odict2dict(repr.raw) == {
        "version": "1",
        "resources": {
            "default": {"type": "csv"}
        },
        "preprocessing": [],
        "variables": {
            "country": {
                "location": {
                    "resource_id": "default",
                    "slices": ["1..", 0]
                },
                "unique": False,
                "sorted": "none",
                "value_type": "unspecified"
            },
            "year": {
                "location": {
                    "resource_id": "default",
                    "slices": [0, "1.."]
                },
                "unique": False,
                "sorted": "none",
                "value_type": "unspecified"
            },
            "data": {
                "location": {
                    "resource_id": "default",
                    "slices": ["1..", "1.."]
                },
                "unique": False,
                "sorted": "none",
                "value_type": "unspecified"
            },
        },
        "alignments": [
            {
                "type": "dimension",
                "source": "data",
                "target": "country",
                "aligned_dims": [{"source": 0, "target": 0}]
            },
            {
                "type": "dimension",
                "source": "data",
                "target": "year",
                "aligned_dims": [{"source": 1, "target": 1}]
            },
        ],
        "semantic_model": {
            "data_nodes": {
                "year": {
                    "class_id": "eg:Record:1",
                    "class_name": "eg:Record",
                    "predicate": "eg:year",
                    "data_type": None
                },
                "data": {
                    "class_id": "eg:Record:1",
                    "class_name": "eg:Record",
                    "predicate": "eg:data",
                    "data_type": None
                },
                "country": {
                    "class_id": "eg:Record:2",
                    "class_name": "eg:Record",
                    "predicate": "eg:country",
                    "data_type": None
                },
            },
            "literal_nodes": [
                {
                    "class_id": "eg:Record:1",
                    "class_name": "eg:Record",
                    "predicate": "eg:owner",
                    "data": "usc",
                    "data_type": None
                }
            ],
            "relations": [
                {
                    "source_id": "eg:Record:1",
                    "predicate": "eg:location",
                    "target_id": "eg:Record:2"
                }
            ],
            "prefixes": {
                "eg": "https://example.org"
            }
        }
    }


def test_simplify_func():
    repr = Repr.from_yml_string("""
version: '1'
resources: csv
variables:
  country: [1.., 0]
  year: [0, 1..]
  data: [1.., 1..]
alignments:
  - type: dimension
    value: data:0 <-> country:0
  - type: dimension
    value: data:1 <-> year:1
semantic_model:
  data_nodes:
    year: eg:Record:1--eg:year
    data: eg:Record:1--eg:data
    country: eg:Record:2--eg:country
  literal_nodes:
    - eg:Record:1--eg:owner--usc
  relations:
    - eg:Record:1--eg:location--eg:Record:2
  prefixes:
    eg: https://example.org
    """)
    repr.normalize_mut()
    repr.simplify_mut()

    assert odict2dict(repr.raw) == {
        "version": "1",
        "resources": "csv",
        "variables": {
            "country": ["1..", 0],
            "year": [0, "1.."],
            "data": ["1..", "1.."],
        },
        "alignments": [
            {
                "type": 'dimension',
                "value": "data:0 <-> country:0"
            },
            {
                "type": 'dimension',
                "value": "data:1 <-> year:1"
            },
        ],
        "semantic_model": {
            "data_nodes": {
                "year": "eg:Record:1--eg:year",
                "data": "eg:Record:1--eg:data",
                "country": "eg:Record:2--eg:country",
            },
            "literal_nodes": [
                "eg:Record:1--eg:owner--usc"
            ],
            "relations": [
                "eg:Record:1--eg:location--eg:Record:2"
            ],
            "prefixes": {
                "eg": "https://example.org"
            }
        }
    }


def odict2dict(odict):
    if isinstance(odict, (dict, OrderedDict)):
        return {k: odict2dict(v) for k, v in odict.items()}
    elif isinstance(odict, list):
        return [odict2dict(v) for v in odict]
    else:
        return odict
