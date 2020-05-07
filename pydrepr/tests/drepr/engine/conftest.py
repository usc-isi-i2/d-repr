from pathlib import Path
from typing import List, Dict, Tuple, Callable, Any, Optional

import pytest

from drepr import DRepr


def get_drepr(dataset_dir: Path):
    return DRepr.parse_from_file(str(dataset_dir / "model.yml"))


@pytest.fixture()
def d_s04(resource_dir):
    o = get_drepr(resource_dir / "s04_shorten_gldas")
    o.__name__ = "d_s04"
    return o