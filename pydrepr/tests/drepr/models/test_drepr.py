from drepr import DRepr
from drepr.models.align import RangeAlignment, AlignedStep
from drepr.models.attr import Attr, Sorted, ValueType
from drepr.models.drepr import yaml
from drepr.models.parse_v1.path_parser import PathParser, PathParserV1
from drepr.models.preprocessing import Preprocessing, PMap, PreprocessingType
from drepr.models.resource import Resource, ResourceType, CSVProp
from drepr.models.sm import ClassNode, DataNode, SemanticModel, DataType, Edge

content = """
version: '1'
resources:
  default:
    type: csv
preprocessing:
  - type: pmap
    path: $[2:][1:]
    code: |
      return float(value)
attributes:
  area: 
    resource_id: default
    path: $[2:][0]
    missing_values: [-999]
    unique: true
    sorted: ascending
    value_type: list[int]
  gender: $[1][1:]
  period: $[0][1:]
  obs: $[2:][1:]
alignments:
  - type: dimension
    source: obs
    target: area
    aligned_dims:
      - { source: 0, target: 0 }
  - type: dimension
    value: obs:1 <-> gender:1
  - type: dimension
    value: obs:1 <-> period:1
semantic_model:
  data_nodes:
    area: qb:Observation:1--eg:refArea
    gender: qb:Observation:1--eg:gender
    period: qb:Observation:1--eg:refPeriod^^xsd:anyURI
    obs: qb:Observation:1--smdx-measure:obsValue
  relations: []
  prefixes:
    qb: http://purl.org/linked-data/cube#
    smdx-measure: http://purl.org/linked-data/sdmx/2009/measure#
    eg: http://example.org/
    """


def test_parse():
    ds_model = DRepr.parse(yaml.load(content))
    gold_resource = Resource("default", ResourceType.CSV, CSVProp(delimiter=","))
    gold_ds_model = DRepr(
        [gold_resource],
        [Preprocessing(PreprocessingType.pmap,
                       PMap("default", PathParserV1().parse(gold_resource, "$[2:][1:]", ""), "return float(value)\n", None, None))],
        [
            Attr("area", "default", PathParserV1().parse(gold_resource, "$[2:][0]", ""), [-999], True, Sorted.Ascending,
                 ValueType.List_Int),
            Attr("gender", "default", PathParserV1().parse(gold_resource, "$[1][1:]", ""), [], False, Sorted.Null, ValueType.Unspecified),
            Attr("period", "default", PathParserV1().parse(gold_resource, "$[0][1:]", ""), [], False, Sorted.Null, ValueType.Unspecified),
            Attr("obs", "default", PathParserV1().parse(gold_resource, "$[2:][1:]", ""), [], False, Sorted.Null,
                 ValueType.Unspecified),
        ],
        [
            RangeAlignment('obs', 'area', [AlignedStep(0, 0)]),
            RangeAlignment('obs', 'gender', [AlignedStep(1, 1)]),
            RangeAlignment('obs', 'period', [AlignedStep(1, 1)])
        ],
        SemanticModel(
            nodes={
                'qb:Observation:1': ClassNode('qb:Observation:1', 'qb:Observation'),
                'dnode:area': DataNode('dnode:area', 'area'),
                'dnode:gender': DataNode('dnode:gender', 'gender'),
                'dnode:period': DataNode('dnode:period', 'period', DataType('xsd:anyURI')),
                'dnode:obs': DataNode('dnode:obs', 'obs')
            },
            edges={
                0: Edge(0, 'qb:Observation:1',
                     'dnode:area',
                     'eg:refArea',
                     ),
                1: Edge(1, 'qb:Observation:1',
                     'dnode:gender',
                     'eg:gender',
                     ),
                2: Edge(2, 'qb:Observation:1',
                     'dnode:period',
                     'eg:refPeriod',
                     ),
                3: Edge(3, 'qb:Observation:1',
                     'dnode:obs',
                     'smdx-measure:obsValue',
                     )
            },
            prefixes={
                'qb': 'http://purl.org/linked-data/cube#',
                'smdx-measure': 'http://purl.org/linked-data/sdmx/2009/measure#',
                'eg': 'http://example.org/',
                'drepr': 'https://purl.org/drepr/1.0/',
                'rdf': 'http://www.w3.org/1999/02/22-rdf-syntax-ns#',
                'rdfs': 'http://www.w3.org/2000/01/rdf-schema#',
                'owl': 'http://www.w3.org/2002/07/owl#'

            }
        )
    )

    assert ds_model == gold_ds_model


def test_serialize():
    ds_model = DRepr.parse(yaml.load(content))
    assert ds_model.serialize() == {
        'resources': [{'id': 'default', 'prop': {'delimiter': ','}, 'type': 'csv'}],
        'preprocessing': [{
            'type': 'pmap',
            'value': {
                'change_structure': None,
                'code': 'return float(value)\n',
                'output': None,
                'path': {
                    'steps': [
                        {'end': None, 'start': 2, 'step': 1},
                        {'end': None, 'start': 1, 'step': 1}
                    ]}
                ,
                'resource_id': 'default'}
        }],
        'attrs': [
            {
                'id': 'area',
                'missing_values': [-999],
                'path': {
                    'steps': [
                        {'end': None, 'start': 2, 'step': 1},
                        {'val': 0}
                    ]
                },
                'resource_id': 'default',
                'sorted': 'ascending',
                'unique': True,
                'value_type': 'list[int]'
            },
            {
                'id': 'gender',
                'missing_values': [],
                'path': {
                    'steps': [
                        {'val': 1},
                        {'end': None, 'start': 1, 'step': 1}
                    ]
                },
                'resource_id': 'default',
                'sorted': 'none',
                'unique': False,
                'value_type': 'unspecified'},
            {
                'id': 'period',
                'missing_values': [],
                'path': {
                    'steps': [
                        {'val': 0},
                        {'end': None, 'start': 1, 'step': 1}
                    ]
                },
                'resource_id': 'default',
                'sorted': 'none',
                'unique': False,
                'value_type': 'unspecified'},
            {
                'id': 'obs',
                'missing_values': [],
                'path': {
                    'steps': [
                        {'end': None, 'start': 2, 'step': 1},
                        {'end': None, 'start': 1, 'step': 1}
                    ]
                },
                'resource_id': 'default',
                'sorted': 'none',
                'unique': False,
                'value_type': 'unspecified'
            }
        ],
        'aligns': [
            {
                'type': 'range',
                'aligned_steps': [{'source_idx': 0, 'target_idx': 0}],
                'source': 'obs',
                'target': 'area'
            },
            {
                'type': 'range',
                'aligned_steps': [{'source_idx': 1, 'target_idx': 1}],
                'source': 'obs',
                'target': 'gender'
            },
            {
                'type': 'range',
                'aligned_steps': [{'source_idx': 1, 'target_idx': 1}],
                'source': 'obs',
                'target': 'period'
            }
        ],
        'sm': {
            'edges': {
                0: {
                    'edge_id': 0,
                    'is_required': False,
                    'is_subject': False,
                    'label': 'eg:refArea',
                    'source_id': 'qb:Observation:1',
                    'target_id': 'dnode:area'
                },
                1: {
                    'edge_id': 1,
                    'is_required': False,
                    'is_subject': False,
                    'label': 'eg:gender',
                    'source_id': 'qb:Observation:1',
                    'target_id': 'dnode:gender'
                },
                2: {
                    'edge_id': 2,
                    'is_required': False,
                    'is_subject': False,
                    'label': 'eg:refPeriod',
                    'source_id': 'qb:Observation:1',
                    'target_id': 'dnode:period'
                },
                3: {
                    'edge_id': 3,
                    'is_required': False,
                    'is_subject': False,
                    'label': 'smdx-measure:obsValue',
                    'source_id': 'qb:Observation:1',
                    'target_id': 'dnode:obs'
                }
            },
            'nodes': {
                'dnode:area': {
                    'type': 'data_node',
                    'attr_id': 'area',
                    'data_type': None,
                    'node_id': 'dnode:area'
                },
                'dnode:gender': {
                    'type': 'data_node',
                    'attr_id': 'gender',
                    'data_type': None,
                    'node_id': 'dnode:gender'
                },
                'dnode:obs': {
                    'type': 'data_node',
                    'attr_id': 'obs',
                    'data_type': None,
                    'node_id': 'dnode:obs'
                },
                'dnode:period': {
                    'type': 'data_node',
                    'attr_id': 'period',
                    'data_type': 'xsd:anyURI',
                    'node_id': 'dnode:period'
                },
                'qb:Observation:1': {
                    'type': 'class_node',
                    'label': 'qb:Observation',
                    'node_id': 'qb:Observation:1'
                }
            },
            'prefixes': {
                'drepr': 'https://purl.org/drepr/1.0/',
                'eg': 'http://example.org/',
                'owl': 'http://www.w3.org/2002/07/owl#',
                'qb': 'http://purl.org/linked-data/cube#',
                'rdf': 'http://www.w3.org/1999/02/22-rdf-syntax-ns#',
                'rdfs': 'http://www.w3.org/2000/01/rdf-schema#',
                'smdx-measure': 'http://purl.org/linked-data/sdmx/2009/measure#'
            }
        }
    }


def test_deserialize():
    ds_model = DRepr.parse(yaml.load(content))
    assert ds_model == DRepr.deserialize(ds_model.serialize())


def test_write_to_yml():
    ds_model = DRepr.parse(yaml.load(content))
    assert ds_model.to_lang_yml(True) == """version: '1'
resources:
  default:
    type: csv
    delimiter: ','
preprocessing:
- type: pmap
  resource_id: default
  path:
  - 2..:1
  - 1..:1
  code: |
    return float(value)
  output:
  change_structure:
attributes:
  area:
    resource_id: default
    path:
    - 2..:1
    - 0
    unique: true
    sorted: ascending
    value_type: list[int]
    missing_values: [-999]
  gender:
    resource_id: default
    path:
    - 1
    - 1..:1
    unique: false
    sorted: none
    value_type: unspecified
    missing_values: []
  period:
    resource_id: default
    path:
    - 0
    - 1..:1
    unique: false
    sorted: none
    value_type: unspecified
    missing_values: []
  obs:
    resource_id: default
    path:
    - 2..:1
    - 1..:1
    unique: false
    sorted: none
    value_type: unspecified
    missing_values: []
alignments:
- type: range
  source: obs
  target: area
  aligned_dims:
  - source: 0
    target: 0
- type: range
  source: obs
  target: gender
  aligned_dims:
  - source: 1
    target: 1
- type: range
  source: obs
  target: period
  aligned_dims:
  - source: 1
    target: 1
semantic_model:
  data_nodes:
    area: qb:Observation:1--eg:refArea
    gender: qb:Observation:1--eg:gender
    period: qb:Observation:1--eg:refPeriod^^xsd:anyURI
    obs: qb:Observation:1--smdx-measure:obsValue
  relations: []
  literal_nodes: []
  subjects: {}
  prefixes:
    qb: http://purl.org/linked-data/cube#
    smdx-measure: http://purl.org/linked-data/sdmx/2009/measure#
    eg: http://example.org/
    drepr: https://purl.org/drepr/1.0/
    rdf: http://www.w3.org/1999/02/22-rdf-syntax-ns#
    rdfs: http://www.w3.org/2000/01/rdf-schema#
    owl: http://www.w3.org/2002/07/owl#
"""
    ds_model2 = DRepr.parse(yaml.load(ds_model.to_lang_yml(True)))
    assert ds_model2 == ds_model