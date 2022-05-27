import os
from pathlib import Path
import glob
from typing import List, Dict, Tuple, Callable, Any, Optional

import ujson

from drepr import DRepr

if __name__ == '__main__':
    # generate json models
    test_dir = os.path.dirname(os.path.abspath(__file__))
    for model_file in glob.glob(os.path.join(test_dir, "**/resources/*/model.yml"), recursive=True):
        dataset_dir = Path(model_file).parent
        if not dataset_dir.is_dir() or not (dataset_dir / "model.yml").exists():
            continue

        with open(str(dataset_dir / "model.json"), "w") as f:
            result = DRepr.parse_from_file(str(dataset_dir / "model.yml")).to_engine_format()
            f.write(ujson.dumps(result.model, indent=4, escape_forward_slashes=False))
        with open(str(dataset_dir / "model.meta"), "w") as f:
            f.write(ujson.dumps({
                "attributes": result.attribute_idmap,
                "resources": result.resource_idmap
            }, indent=4))
