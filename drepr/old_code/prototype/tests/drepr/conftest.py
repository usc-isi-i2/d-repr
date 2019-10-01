#!/usr/bin/python
# -*- coding: utf-8 -*-
import subprocess
from pathlib import Path
from typing import List, Dict
from uuid import uuid4

import networkx as nx
import pytest

from drepr.data_source import DataSource
from drepr.misc.serde import deserialize_yml
from drepr.models import Representation, Mapping
from drepr.models.parser_v1 import DimensionMappingReadableDeSerV1


class DSInput:
    def __init__(self, name: str, data_source_dir: Path, data_files: Dict[str, str],
                 repr_file: Path):
        self.name = name
        self.data_source_dir = data_source_dir
        self.repr_file = repr_file
        self.data_files = data_files

    @staticmethod
    def get_inputs(desc_dname=None) -> Dict[str, List['DSInput']]:
        resource_dir: Path = Path(__file__).parent.parent / "resources" / "data_sources"
        if desc_dname is None:
            desc_dname = "correct_descriptions"

        datasets: Dict[str, List[DSInput]] = dict()
        for dataset_dir in resource_dir.iterdir():
            if not dataset_dir.is_dir():
                continue

            data_files = {}
            for file in dataset_dir.iterdir():
                if file.is_file() and any(
                        file.name.endswith(x) for x in [".gz", ".nc", ".json", ".csv", ".tsv"]):
                    data_files[file.stem] = str(file)

            if len(data_files) == 0:
                raise Exception(f"Dataset {dataset_dir.name} doesn't have any data file")

            examples = []
            if (dataset_dir / desc_dname).exists():
                for file in (dataset_dir / desc_dname).iterdir():
                    if file.name.endswith(".s01.model.yml"):
                        examples.append(
                            DSInput(dataset_dir.name, dataset_dir, data_files,
                                    file))

            datasets[dataset_dir.name] = examples
        return datasets

    @staticmethod
    def get_invalid_inputs() -> Dict[str, List['DSInput']]:
        return DSInput.get_inputs("invalid_descriptions")

    def __repr__(self):
        return f'("{self.name}" > "{self.repr_file.name}")'

    def get_raw_repr(self):
        if self.repr_file.name.endswith(".yml"):
            return deserialize_yml(self.repr_file)
        raise NotImplementedError()

    def get_repr(self):
        return Representation.parse(self.get_raw_repr())

    def get_data_source(self):
        repr = self.get_repr()
        if len(self.data_files) == 1:
            # set resource id correctly (it was file name)
            guess_rid = next(iter(self.data_files.keys()))
            if guess_rid not in repr.resources:
                correct_rid = next(iter(repr.resources.keys()))
                self.data_files[correct_rid] = self.data_files.pop(guess_rid)

        return DataSource(repr, self.data_files)

    def get_queries(self) -> List[dict]:
        """Return a list of test queries and their results"""
        if not (self.data_source_dir / "queries").exists():
            return []

        results = []
        if sum(1 for f in self.repr_file.parent.iterdir() if f.name.endswith(".s01.model.yml")) > 1:
            # there are more than one correct descriptions, queries need to be organized based on folder
            for file in (self.data_source_dir / "queries" /
                         self.repr_file.name[:self.repr_file.name.find(".")]).iterdir():
                if file.name.endswith(".yml"):
                    results.append(deserialize_yml(file))
                    results[-1]['__name__'] = file.name
        else:
            for file in (self.data_source_dir / "queries").iterdir():
                if file.name.endswith(".yml"):
                    results.append(deserialize_yml(file))
                    results[-1]['__name__'] = file.name
        return results

    def get_supplementary_mapping_funcs(self) -> List[Mapping]:
        """Return a list of correct mapping functions that users can provide (which we auto infer)"""
        if not (self.data_source_dir / "mapping_funcs.yml").exists():
            return []

        return [
            DimensionMappingReadableDeSerV1.unsafe_deserialize(m)
            for m in deserialize_yml(self.data_source_dir / "mapping_funcs.yml")
        ]


@pytest.fixture(
    scope="session",
    params=[y for ys in DSInput.get_inputs().values() for y in ys],
    ids=DSInput.__repr__)
def ds_input(request):
    return request.param


@pytest.fixture(scope="session")
def fao_stat_crop_yields():
    return DSInput.get_inputs()["FAOSTAT_crop_yields"][0]


@pytest.mark.skip
def test_plot_ds_input(ds_input: DSInput):
    """Plot models and queries in a data source"""
    from tests.drepr.data_source.test_data_source import query2sg

    repr = ds_input.get_repr()
    sm = repr.get_semantic_model()
    tmp_file = f"/tmp/{str(uuid4())}.dot"
    nx.nx_agraph.write_dot(sm.graph, tmp_file)

    outdir = ds_input.data_source_dir / "viz" / "drepr"
    outdir.mkdir(exist_ok=True, parents=True)
    subprocess.check_call(["dot", "-Tpdf", tmp_file, f"-o{outdir / ds_input.repr_file.stem}.pdf"])

    outdir = ds_input.data_source_dir / "viz" / "queries"
    outdir.mkdir(exist_ok=True, parents=True)
    # TODO: do a proper query visualization
    for query in ds_input.get_queries():
        sg = query2sg(repr.ont, query['query'])
        nx.nx_agraph.write_dot(sg.graph, tmp_file)
        subprocess.check_call(["dot", "-Tpdf", tmp_file, f"-o{outdir / query['__name__']}.pdf"])
