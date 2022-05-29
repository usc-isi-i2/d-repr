import os
from pathlib import Path

from drepr.engine import execute, DRepr, FileOutput, OutputFormat

if __name__ == "__main__":
    wdir = Path(os.path.abspath(__file__)).parent
    for dataset_dir in sorted(wdir.iterdir()):
        # if not dataset_dir.name.startswith("african_port"):
        #     continue

        if not dataset_dir.is_dir() or (dataset_dir / ".ignore").exists():
            continue

        # one folder may have multiple examples
        models = [
            file for file in dataset_dir.iterdir() if file.name.endswith("model.yml")
        ]
        for model in models:
            dsid = model.stem.split(".")[0]

            model_output = model.parent / f"{dsid}.model.out"
            resources = {}
            for file in dataset_dir.iterdir():
                if file.stem.startswith(dsid) and not file.stem.startswith(
                    f"{dsid}.model"
                ):
                    resources[file.stem[len(dsid) + 1 :]] = str(file)

            print(f"Execute dataset: {dataset_dir.name}/{dsid}")
            ds_model = DRepr.parse_from_file(str(model))

            try:
                execute(
                    ds_model,
                    resources,
                    FileOutput(dataset_dir / f"{dsid}.model.out", OutputFormat.TTL),
                )
            except SystemError:
                print(
                    "Error while executing the D-REPR engine for dataset",
                    dataset_dir.name,
                )
                pass
