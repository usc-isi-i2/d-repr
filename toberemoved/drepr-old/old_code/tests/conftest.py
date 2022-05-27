import os
from pathlib import Path

import pytest


def get_examples_dir():
    testdir = Path(os.path.dirname(os.path.abspath(__file__)))
    return testdir.parent / "examples"


@pytest.fixture(params=[
    item for item in get_examples_dir().iterdir()
    if item.is_dir() and not (item / ".ignore").exists()
], ids=lambda item: item.stem)
def example_dir(request) -> Path:
    return request.param
