from pydrepr import ReprBuilder


def test_build_data_cube_model():
    repr = ReprBuilder() \
        .add_resource("default", "csv", delimiter=",") \
        .add_preprocess_func("pmap", ["2..", "1.."], code="return float(value)") \
        .add_preprocess_func("pmap", [0, "1.."], code="""
if value == "":
    return context.get_left_value(index)
return "http://reference.data.gov.uk/id/gregorian-interval/" + value.split("-")[0] + "-01-01T00:00:00/P3Y"
        """.strip()) \
        .add_attribute("area", ["2..", 0]) \
        .add_attribute("gender", [1, "1.."]) \
        .add_attribute("period", [0, "1.."]) \
        .add_attribute("obs", ["2..", "1.."]) \
        .add_dim_alignment("obs", "area", [{"source": 0, "target": 0}]) \
        .add_dim_alignment("obs", "gender", [{"source": 1, "target": 1}]) \
        .add_dim_alignment("obs", "period", [{"source": 1, "target": 1}]) \
        .add_sm() \
            .add_prefix("qb", "http://purl.org/linked-data/cube#") \
            .add_prefix("smdx-measure", "http://purl.org/linked-data/sdmx/2009/measure#") \
            .add_prefix("eg", "http://example.org/") \
            .add_class("qb:Observation") \
                .add_data_node("eg:refArea", "area") \
                .add_data_node("eg:gender", "gender") \
                .add_data_node("eg:refPeriod", "period", "xsd:anyURI") \
                .add_data_node("smdx-measure:obsValue", "obs") \
            .finish() \
        .finish() \
        .build()

    assert repr.to_yml_string(simplify=False) == """
version: '1'
resources:
  default:
    type: csv
    delimiter: ','
preprocessing:
- type: pmap
  input:
    resource_id: default
    slices:
    - 2..
    - 1..
  code: |-
    return float(value)
  output:
- type: pmap
  input:
    resource_id: default
    slices:
    - 0
    - 1..
  code: |-
    if value == "":
        return context.get_left_value(index)
    return "http://reference.data.gov.uk/id/gregorian-interval/" + value.split("-")[0] + "-01-01T00:00:00/P3Y"
  output:
variables:
  area:
    location:
      resource_id: default
      slices:
      - 2..
      - 0
    unique: false
    sorted: none
    value_type: unspecified
  gender:
    location:
      resource_id: default
      slices:
      - 1
      - 1..
    unique: false
    sorted: none
    value_type: unspecified
  period:
    location:
      resource_id: default
      slices:
      - 0
      - 1..
    unique: false
    sorted: none
    value_type: unspecified
  obs:
    location:
      resource_id: default
      slices:
      - 2..
      - 1..
    unique: false
    sorted: none
    value_type: unspecified
alignments:
- type: dimension
  source: obs
  target: area
  aligned_dims:
  - source: 0
    target: 0
- type: dimension
  source: obs
  target: gender
  aligned_dims:
  - source: 1
    target: 1
- type: dimension
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
  literal_nodes: []
  relations: []
  prefixes:
    qb: http://purl.org/linked-data/cube#
    smdx-measure: http://purl.org/linked-data/sdmx/2009/measure#
    eg: http://example.org/
""".lstrip()
