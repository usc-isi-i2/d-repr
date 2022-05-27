#!/usr/bin/python
# -*- coding: utf-8 -*-
import io
import ujson
from pathlib import Path
from typing import Callable

from api.models.dataset import Dataset
from api.models.resource import Resource


def test_create_resource(client, tester_auth, get_s201_multiple_sources: Callable[[str], Dataset],
                         resource_dir: Path):
    dataset = get_s201_multiple_sources(tester_auth['_UserEmail_'])
    resource_file = [file for file in resource_dir.iterdir()
                     if file.name.startswith(dataset.name)][0]
    resource_id = dataset.name[dataset.name.find("_") + 1:]
    assert resource_file.name.endswith(".csv")

    with open(str(resource_file), "rb") as f:
        resource_file = (io.BytesIO(f.read()), resource_file.name)

    resp = client.post(
        f'/datasets/{dataset.name}/resources',
        data={
            'resource_id': resource_id,
            'resource_type': 'csv',
            'resource_file': resource_file,
            'extra': ujson.dumps({
                'delimiter': ','
            })
        },
        headers=tester_auth,
        content_type='multipart/form-data')

    assert resp.status_code == 200
    assert len(Dataset.get_by_id(dataset.id).get_repr().resources) == 1 + len(
        dataset.get_repr().resources), "New resource"

    dataset.refresh()
    assert Resource.get(
        Resource.dataset == dataset,
        Resource.resource_id == resource_id).db_id == dataset.get_resource_db_id(resource_id)


def test_delete_resource(client, tester_auth, get_s001_kaya_rice_import: Callable[[str], Dataset]):
    dataset = get_s001_kaya_rice_import(tester_auth['_UserEmail_'])
    resource_id = dataset.name[dataset.name.find("_") + 1:]

    # refresh data to load resource
    assert resource_id in client.get(
        f"/datasets/{dataset.name}", headers=tester_auth).json['resources']
    assert client.delete(
        f"/datasets/{dataset.name}/resources/{resource_id}", headers=tester_auth).status_code == 200
    assert client.get(f"/datasets/{dataset.name}", headers=tester_auth).json['resources'] == {}
    assert client.delete(
        f"/datasets/{dataset.name}/resources/{resource_id}", headers=tester_auth).status_code == 404


def test_get_resource_data(client, tester_auth,
                           get_s001_kaya_rice_import: Callable[[str], Dataset]):
    dataset = get_s001_kaya_rice_import(tester_auth['_UserEmail_'])
    resource_id = dataset.name[dataset.name.find("_") + 1:]

    slice = {
        'type': 'range',
        'range': [0, 2],
        'values': [{
            'type': 'range',
            'range': [0, 3],
            'values': [None]
        }]
    }
    resp = client.get(
        f"/datasets/{dataset.name}/resources/{resource_id}",
        query_string={'slices': ujson.dumps(slice)},
        headers=tester_auth)
    assert resp.status_code == 200
    assert resp.json['data'] == [['Year', 'Jan ', 'Feb'], ['2014', '256', '543']]
    assert resp.json['slice'] == slice
