import os
from pathlib import Path
from typing import List, Dict, Tuple, Callable, Any, Optional

import ujson

from drepr import DRepr

if __name__ == '__main__':
    # generate json models
    for dataset_dir in Path(os.path.abspath(__file__)).parent.iterdir():
        if not dataset_dir.is_dir() or not (dataset_dir / "model.yml").exists():
            continue

        with open(dataset_dir / "model.json", "w") as f:
            result = DRepr.parse_from_file(str(dataset_dir / "model.yml")).to_engine_format()
            f.write(ujson.dumps(result.model, indent=4, escape_forward_slashes=False))
        with open(dataset_dir / "model.meta", "w") as f:
            f.write(ujson.dumps({
                "attributes": result.attribute_idmap,
                "resources": result.resource_idmap
            }, indent=4))
