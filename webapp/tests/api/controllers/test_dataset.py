#!/usr/bin/python
# -*- coding: utf-8 -*-
from typing import *

from api.models.dataset import Dataset


def test_required_authorization(client, tester_auth):
    assert client.get("/datasets").status_code == 401
    assert client.get("/datasets", headers=tester_auth).status_code == 200


def test_create_dataset(client, tester_auth, admin_auth):
    # create new dataset will increase number of dataset by one, and the newest is the one we just create
    n_datasets = Dataset.select().count()
    assert Dataset.select().where((Dataset.name == "test-dataset")
                                  & (Dataset.owner == tester_auth['_UserId_'])).count() == 0
    resp = client.post('/datasets', json={'name': 'test-dataset', "description": ""}, headers=tester_auth)
    assert resp.status_code == 200
    assert Dataset.select().count() == n_datasets + 1

    dataset = Dataset.get((Dataset.name == "test-dataset") & (Dataset.owner == tester_auth['_UserId_']))
    assert resp.json == {'status': 'success', 'dataset': dataset.name}

    # create duplicated dataset is not allow
    assert client.post("/datasets", json={'name': 'test-dataset', "description": ""}, headers=tester_auth).status_code == 409
    assert Dataset.select().count() == n_datasets + 1

    # but another user create is totally fine
    assert client.post("/datasets", json={'name': 'test-dataset', "description": ""}, headers=admin_auth).status_code == 200
    assert Dataset.select().count() == n_datasets + 2


def test_list_datasets(client, tester_auth):
    assert {x['name']: x for x in client.get("/datasets", headers=tester_auth).json['datasets']} == {
        dsname: {"description": dsname, "name": dsname}
        for dsname in ["s101_time_row", "s001_kaya_rice_import", "s201_multiple_sources"]
    }
    client.post('/datasets', json={'name': 'test-dataset', "description": "test-dataset"}, headers=tester_auth)
    assert {x['name']: x for x in client.get("/datasets", headers=tester_auth).json['datasets']} == {
        dsname: {"description": dsname, "name": dsname}
        for dsname in ['test-dataset', "s101_time_row", "s001_kaya_rice_import", "s201_multiple_sources"]
    }


def test_has_dataset(client, tester_auth, admin_auth, get_s001_kaya_rice_import: Callable[[str], Dataset]):
    dataset = get_s001_kaya_rice_import(tester_auth['_UserEmail_'])

    assert client.head(f"/datasets/{dataset.name}", headers=tester_auth).status_code == 200
    assert client.head(f"/datasets/kdsjflsdkj", headers=tester_auth).status_code == 404

    # other user cannot see this dataset
    assert client.head(f"/datasets/{dataset.name}", headers=admin_auth).status_code == 404


def test_get_dataset(client, tester_auth, get_s001_kaya_rice_import: Callable[[str], Dataset]):
    dataset = get_s001_kaya_rice_import(tester_auth['_UserEmail_'])

    resp = client.get(f"/datasets/{dataset.name}", headers=tester_auth)
    assert resp.status_code == 200
    results = resp.json
    assert results == {
        'resources': {
            'kaya_rice_import': {
                'id': 'kaya_rice_import',
                'type': 'csv',
                'delimiter': ',',
                'dimension': {
                    'type': 'range',
                    'range': [0, 5],
                    'values': [{
                        'type': 'range',
                        'range': [0, 13],
                        'values': [None]
                    }]
                }
            }
        },
        'preprocessing': [],
        'variables': {
            'month': {
                'id': 'month',
                'location': {
                    'resource_id': 'kaya_rice_import',
                    'slices': [{
                        'idx': 0,
                        'type': 'index'
                    }, {
                        'end': None,
                        'start': 1,
                        'step': 1,
                        'type': 'range'
                    }]
                },
                'missing_values': [],
                'sorted': 'none',
                'value_type': 'unspecified',
                'unique': False,
            },
            'year': {
                'id': 'year',
                'location': {
                    'resource_id':
                        'kaya_rice_import',
                    'slices': [{
                        'end': None,
                        'start': 1,
                        'step': 1,
                        'type': 'range'
                    }, {
                        'idx': 0,
                        'type': 'index'
                    }]
                },
                'missing_values': [],
                'sorted': 'none',
                'value_type': 'unspecified',
                'unique': False,
            },
            'import_unit': {
                'id': 'import_unit',
                'location': {
                    'resource_id':
                        'kaya_rice_import',
                    'slices': [{
                        'end': None,
                        'start': 1,
                        'step': 1,
                        'type': 'range'
                    }, {
                        'end': None,
                        'start': 1,
                        'step': 1,
                        'type': 'range'
                    }]
                },
                'missing_values': [],
                'sorted': 'none',
                'value_type': 'unspecified',
                'unique': False,
            }
        },
        'alignments': [],
        'semantic_model': {
            'prefixes': {
                'drepr': 'https://purl.org/drepr/1.0/',
                'owl': 'http://www.w3.org/2002/07/owl#',
                'rdf': 'http://www.w3.org/1999/02/22-rdf-syntax-ns#',
                'rdfs': 'http://www.w3.org/2000/01/rdf-schema#'
            },
            'relations': [],
            'data_nodes': {},
            'literal_nodes': [],
        },
    }


def test_upsert_variable(client, tester_auth, get_s101_time_row: Callable[[str], Dataset]):
    dataset = get_s101_time_row(tester_auth['_UserEmail_'])
    resp = client.post(
        f"/datasets/{dataset.name}/variables",
        headers=tester_auth,
        json={
            "prev_id": None,
            "id": "metals",
            "sorted": "none",
            "unique": False,
            "location": {
                "resource_id": "time_row",
                "slices": [{
                    "type": "range",
                    "start": 1,
                    "end": None,
                    "step": 1
                }, {
                    "type": "index",
                    "idx": "2"
                }]
            },
            "missing_values": [],
            "value_type": "unspecified"
        })
    assert resp.status_code == 200, resp.json
    assert set(dataset.get_repr().variables.keys()) == {"year", "energy", "agriculture"}
    dataset.refresh()
    assert set(dataset.get_repr().variables.keys()) == {"year", "energy", "metals", "agriculture"}


def test_del_variable(client, tester2_auth, get_s101_time_row: Callable[[str], Dataset]):
    dataset = get_s101_time_row(tester2_auth['_UserEmail_'])
    resp = client.delete(f"/datasets/{dataset.name}/variables/agriculture", headers=tester2_auth)
    assert resp.status_code == 200, resp.json

    dataset.refresh()
    assert set(dataset.get_repr().variables.keys()) == {"year", "energy"}


def test_update_alignments(client, tester3_auth, get_s101_time_row: Callable[[str], Dataset]):
    dataset = get_s101_time_row(tester3_auth['_UserEmail_'])
    resp = client.post(
        f"/datasets/{dataset.name}/alignments",
        headers=tester3_auth,
        json=[{
            "type": "dimension",
            "source": "year",
            "target": "energy",
            "aligned_dims": [{"source": 0, "target": 0}],
        }])
    assert resp.status_code == 200, resp.json

    assert len(dataset.get_repr().alignments) == 0
    dataset.refresh()
    assert len(dataset.get_repr().alignments) == 1


def test_update_semantic_models(client, tester4_auth, get_s101_time_row: Callable[[str], Dataset]):
    dataset = get_s101_time_row(tester4_auth['_UserEmail_'])
    raw_sm = {
        "data_nodes": {
            "energy": {
                "node_id": "schema:Energy:0",
                "class_uri": "schema:Energy",
                "predicate": "rdf:value",
                "data_type": "xsd:decimal"
            },
            "year": {
                "node_id": "schema:Energy:0",
                "class_uri": "schema:Energy",
                "predicate": "schema:recorded_at",
                "data_type": "xsd:dateTime"
            }
        },
        "literal_nodes": [],
        "relations": [],
        "prefixes": {
            "schema": "https://schema.org/"
        }
    }
    resp = client.post(f"/datasets/{dataset.name}/semantic_model", headers=tester4_auth, json=raw_sm)
    assert resp.status_code == 200, resp.json

    assert dataset.get_repr().semantic_model.serialize() == {
        "prefixes": {
            'drepr': 'https://purl.org/drepr/1.0/',
            'owl': 'http://www.w3.org/2002/07/owl#',
            'rdf': 'http://www.w3.org/1999/02/22-rdf-syntax-ns#',
            'rdfs': 'http://www.w3.org/2000/01/rdf-schema#'
        },
        "literal_nodes": [],
        "data_nodes": {},
        "relations": []
    }
    dataset.refresh()
    assert dataset.get_repr().semantic_model.serialize() == raw_sm
