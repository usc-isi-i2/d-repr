from pathlib import Path
from typing import List, Dict, Tuple, Callable, Any, Optional

import pytest

from drepr import DRepr


def get_drepr(dataset_dir: Path):
    return DRepr.parse_from_file(str(dataset_dir / "model.yml"))


@pytest.fixture()
def d_s01(resource_dir):
    return get_drepr(resource_dir / "s01_synthesis")
