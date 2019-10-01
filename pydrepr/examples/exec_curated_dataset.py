from pathlib import Path
from typing import List, Dict, Tuple, Callable, Any, Optional
from pydrepr import Repr, Graph, execute

if __name__ == '__main__':
    wdir = Path("/Users/rook/Google Drive/MINT Dataset Downloads/Curated_Datasets")
    for fpath in sorted(wdir.iterdir()):
        if not fpath.is_file() or not any(fpath.name.endswith(ext) for ext in [".yml", ".yaml"]):
            continue

        if not fpath.name.startswith("s0002"):
            continue

        dataset_dir = wdir / fpath.stem
        assert dataset_dir.exists(), f"{dataset_dir} does not exist"

        # reload the resources
        rfiles = {
            rfile.stem: str(rfile)
            for rfile in dataset_dir.iterdir() if rfile.name.startswith("file_")
        }
        if len(rfiles) == 1:
            rfiles['default'] = rfiles.pop('file_1')

        print("Execute dataset:", fpath.name)

        repr = Repr.from_file(str(fpath))
        repr.validate()
        result = execute(repr.normalize_mut(), rfiles, "ttl")

        with open(dataset_dir / "output.ttl", "w") as f:
            f.write(result)
