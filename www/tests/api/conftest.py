from pathlib import Path
from typing import Callable

import pytest

from api.app import app
from api.models.access_token import AccessToken
from api.models.dataset import Dataset
from api.models.repr.representation import Representation, ExtResource
from api.models.resource import Resource
from api.models.user import User
from drepr import DRepr, models


@pytest.fixture
def resource_dir() -> Path:
    return Path(__file__).parent.parent / "resources"


@pytest.fixture()
def client(resource_dir):
    # setup data
    # #################################
    for cls in [Resource, Dataset, AccessToken, User]:
        cls.delete().execute()

    tester = User.create("tester", "tester123")
    tester2 = User.create("tester2", "tester123")
    tester3 = User.create("tester3", "tester123")
    tester4 = User.create("tester4", "tester123")
    admin = User.create("admin", "admin123")

    # reload dataset
    # #################################
    for user in [tester, tester2, tester3, tester4]:
        for ds_name, ds_files in ({
                's001_kaya_rice_import': ["s001_kaya_rice_import.csv"],
                's201_multiple_sources': [],
                "s101_time_row": ["s101_time_row.csv"]
        }).items():
            dataset = Dataset.create(
                owner=user.id, name=ds_name, description=ds_name, representation=Representation.default().serialize())
            ds_model = DRepr.parse_from_file(str(resource_dir / f"{ds_name}.model.yml"))

            ext_resources = {}

            for file in ds_files:
                with open(str(resource_dir / file), "rb") as f:
                    raw_bytes = f.read()

                resource_id = file[file.find("_") + 1:file.find(".")]
                r = next(r for r in ds_model.resources if r.id == resource_id)
                tmp_obj = {
                    "id": resource_id,
                    "type": r.type.value
                }
                if r.type == models.ResourceType.CSV and r.prop is not None:
                    tmp_obj['delimiter'] = r.prop.delimiter

                wres = Resource(
                    dataset=dataset,
                    value=raw_bytes,
                    resource_id=resource_id,
                    resource=tmp_obj)
                wres.save()
                ext_resources[resource_id] = ExtResource.from_resource(wres)

            dataset.set_repr(Representation.from_repr(ds_model, ext_resources))
            dataset.save()

    # #################################
    yield app.test_client()

    # clear all data
    # #################################
    for cls in [Resource, Dataset, AccessToken, User]:
        cls.delete().execute()


def get_auth(client, username, password):
    resp = client.post('/login', json={'email': username, 'password': password})
    return {
        'Authorization': resp.json['auth_token'],
        '_UserEmail_': username,
        '_UserId_': User.get(User.email == username).id
    }


@pytest.fixture
def tester_auth(client):
    return get_auth(client, 'tester', 'tester123')


@pytest.fixture
def tester2_auth(client):
    return get_auth(client, 'tester2', 'tester123')


@pytest.fixture
def tester3_auth(client):
    return get_auth(client, 'tester3', 'tester123')


@pytest.fixture
def tester4_auth(client):
    return get_auth(client, 'tester4', 'tester123')


@pytest.fixture()
def admin_auth(client):
    return get_auth(client, 'admin', 'admin123')


@pytest.fixture
def get_s001_kaya_rice_import(client) -> Callable[[str], Dataset]:
    def get(email: str):
        return Dataset.get((Dataset.owner == User.get(User.email == email))
                           & (Dataset.name == "s001_kaya_rice_import"))
    return get


@pytest.fixture
def get_s201_multiple_sources(client) -> Callable[[str], Dataset]:
    def get(email: str):
        return Dataset.get((Dataset.owner == User.get(User.email == email))
                           & (Dataset.name == "s201_multiple_sources"))
    return get


@pytest.fixture
def get_s101_time_row(client) -> Callable[[str], Dataset]:
    def get(email: str):
        return Dataset.get((Dataset.owner == User.get(User.email == email))
                           & (Dataset.name == "s101_time_row"))
    return get
