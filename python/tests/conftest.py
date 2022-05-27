import os
from pathlib import Path

import pytest


def get_examples_dir():
    testdir = Path(os.path.abspath(__file__)).parent
    return testdir.parent / "examples"


@pytest.fixture()
def resource_dir():
    return Path(os.path.abspath(__file__)).parent / "resources"


@pytest.fixture(params=[
    item for item in get_examples_dir().iterdir()
    if item.is_dir() and not (item / ".ignore").exists()
], ids=lambda item: item.stem)
def example_dir(request) -> Path:
    return request.param
