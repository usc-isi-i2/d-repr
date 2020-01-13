from typing import List, Dict, Tuple, Callable, Any, Optional

from pathlib import Path

import pytest

from drepr.outputs.array_based.array_backend import ArrayBackend


def get_array_backend(dataset_dir: Path):
    return ArrayBackend.from_drepr(str(dataset_dir / "model.yml"), str(dataset_dir / "resource.json"))


@pytest.fixture()
def ds01(resource_dir):
    return get_array_backend(resource_dir / "synthesis_s1")


@pytest.fixture()
def ds02(resource_dir):
    return get_array_backend(resource_dir / "synthesis_s2")


@pytest.fixture()
def ds03(resource_dir):
    return get_array_backend(resource_dir / "synthesis_s3")
