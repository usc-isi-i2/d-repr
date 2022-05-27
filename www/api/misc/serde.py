#!/usr/bin/python
# -*- coding: utf-8 -*-
import csv
import pickle

import ujson, yaml
from collections import OrderedDict
from pathlib import Path
from typing import Union, Optional


def deserialize_yml(fpath: Union[Path, str]) -> dict:
    # load yaml with OrderedDict to preserve order
    # http://stackoverflow.com/questions/5121931/in-python-how-can-you-load-yaml-mappings-as-ordereddicts
    def load_yaml_file(file_stream):
        # noinspection PyPep8Naming
        def ordered_load(stream, Loader=yaml.Loader, object_pairs_hook=OrderedDict):
            class OrderedLoader(Loader):
                pass

            # noinspection PyArgumentList
            def construct_mapping(loader, node):
                loader.flatten_mapping(node)
                return object_pairs_hook(loader.construct_pairs(node))

            # noinspection PyUnresolvedReferences
            OrderedLoader.add_constructor(yaml.resolver.BaseResolver.DEFAULT_MAPPING_TAG, construct_mapping)
            return yaml.load(stream, OrderedLoader)

        # noinspection PyTypeChecker
        return ordered_load(file_stream, yaml.SafeLoader)

    with open(fpath, 'r') as f:
        return load_yaml_file(f)


def deserialize_json(fpath: Union[Path, str]) -> dict:
    with open(str(fpath), "r") as f:
        return ujson.load(f)