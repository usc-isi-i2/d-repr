import os
import ujson
from pathlib import Path

import numpy

import pytest
from typing import List

from drepr.data_source import DataSource
from drepr.misc.ont_ns import OntNS
from drepr.models.semantic_model import ClassID
from drepr.query.return_format import ReturnFormat
from drepr.query.selection_interface import SelectionGraph
from tests.drepr.conftest import DSInput


def test_preprocessing():
    ds_inputs: List[DSInput] = DSInput.get_inputs()["Transformation"]
    data_sources: List[DataSource] = [x.get_data_source() for x in ds_inputs]

    for ipt, ds in zip(ds_inputs, data_sources):
        ds.preprocess()
        updated_data = ds.dump2json()

        with open(
                ipt.data_source_dir / "preprocessing" /
                f"{ipt.repr_file.name[:ipt.repr_file.name.find('.')]}.json", 'r') as f:
            gold_data = ujson.load(f)
        assert updated_data == gold_data


@pytest.mark.parametrize("ds_input,query,return_format", [
    pytest.param(y, q, r, id=f"{str(y)}--{q['__name__']}--{r}") for ys in DSInput.get_inputs().values()
    for y in ys for q in y.get_queries()
    for r in [ReturnFormat.JsonLD, ReturnFormat.JsonLD_FullURI]
])
def test_query(ds_input: DSInput, query: dict, return_format: ReturnFormat):
    ds: DataSource = ds_input.get_data_source()
    sg = query2sg(ds.repr.semantic_model.ont, query['query'])

    results = ds.select(sg).exec(return_format)
    for eres in query['results']:
        pred_results = results[eres['range'][0]:eres['range'][1]]
        if return_format.use_full_uri:
            gold_results = [expand_ont(ds.repr.semantic_model.ont, o) for o in eres["value"]]
        else:
            gold_results = eres['value']

        assert len(gold_results) == len(pred_results)
        all([compare2dict(g, p) for g, p in zip(gold_results, pred_results)])


def query2sg(ont: OntNS, query: dict) -> SelectionGraph:
    sg = SelectionGraph(ont, ClassID.create(ont.full_uri(query['main_node'])))
    for e in query['edges']:
        if e[0] == '[' and e[-1] == ']':
            is_optional = True
            e = e[1:-1]
        else:
            is_optional = False

        e = e.split("--")
        if len(e) == 2:
            sg.add_edge(ClassID.create(e[0]), None, e[1], is_optional)
        else:
            sg.add_edge(ClassID.create(e[0]), ClassID.create(e[2]), e[1], is_optional)

    return sg


def expand_ont(ont: OntNS, jsonld_object: dict) -> dict:
    new_object = {}
    for key, value in jsonld_object.items():
        if key == '@type':
            value = ont.full_uri(value)
        else:
            key = ont.full_uri(key)

        if isinstance(value, dict):
            new_object[key] = expand_ont(ont, value)
        elif isinstance(value, list):
            if len(value) > 0 and isinstance(value[0], dict):
                new_object[key] = [expand_ont(ont, v) for v in value]
            else:
                new_object[key] = value
        else:
            new_object[key] = value
    return new_object


def compare2dict(gold_dict: dict, pred_dict: dict):
    assert set(gold_dict.keys()) == set(pred_dict.keys()), "Number of keys is different"
    for key in gold_dict:
        if isinstance(gold_dict[key], float):
            numpy.testing.assert_almost_equal(
                pred_dict[key], gold_dict[key], err_msg=f"Incompatible value at key: {key}")
        elif isinstance(gold_dict[key], dict):
            assert isinstance(pred_dict[key], dict), "Can only compare values of same type"
            compare2dict(gold_dict[key], pred_dict[key])
        elif isinstance(gold_dict[key], list):
            assert isinstance(pred_dict[key], list), "Can only compare values of same type"
            assert len(gold_dict[key]) == len(pred_dict[key]), f"Incompatible value at key: {key}"
            all([compare2dict(g, p) for g, p in zip(gold_dict[key], pred_dict[key])])
        else:
            assert gold_dict[key] == pred_dict[key], f"Incompatible value at key: {key}"

    return True
