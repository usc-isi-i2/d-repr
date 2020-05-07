from typing import List, Dict, Tuple, Callable, Any, Optional

from pathlib import Path

import pytest

from drepr.outputs.array_backend.array_backend import ArrayBackend
from drepr.outputs.graph_backend.graph_backend import GraphBackend


def get_backends(dataset_dir: Path):
    return [
        ArrayBackend.from_drepr(str(dataset_dir / "model.yml"), str(dataset_dir / "resource.json")),
        GraphBackend.from_drepr(str(dataset_dir / "model.yml"), str(dataset_dir / "resource.json"))
    ]


@pytest.fixture()
def s01(resource_dir):
    return get_backends(resource_dir / "s01_synthesis")


@pytest.fixture()
def s02(resource_dir):
    return get_backends(resource_dir / "s02_synthesis")


@pytest.fixture()
def s03(resource_dir):
    return get_backends(resource_dir / "s03_synthesis")


@pytest.fixture()
def s05(resource_dir):
    return get_backends(resource_dir / "s05_crop_place")