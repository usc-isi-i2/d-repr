#!/usr/bin/python
# -*- coding: utf-8 -*-
import argparse
import time
from pathlib import Path

import ujson

from drepr.data_source import DataSource
from drepr.misc.serde import deserialize_yml
from drepr.models import Representation
from drepr.query.return_format import ReturnFormat
from drepr.query.selection_interface import SelectionGraph

parser = argparse.ArgumentParser()
parser.add_argument(
    "-r",
    "--repr",
    required=True,
    help="either a string containing representation of a dataset (JSON format) "
    "or a path to a file containing representation (support 2 formats: JSON & YML)")

parser.add_argument(
    "-d",
    "--resource",
    nargs="+",
    required=True,
    help="file paths of resources in this format: <resource_id>=<file_path>")
parser.add_argument(
    "-v", "--verbose", default=False, help="increase output verbosity", action="store_true")
parser.add_argument("-o", "--output", help="the file we will write output to")
args = parser.parse_args()

# read repr
try:
    raw_repr = ujson.loads(args.repr)
except ValueError:
    if not Path(args.repr).exists():
        print(f"Cannot read the representation file at: {args.repr}")
        exit(1)

    if args.repr.endswith(".json"):
        with open(args.repr, "r") as f:
            raw_repr = ujson.load(f)
    elif args.repr.endswith(".yml") or args.repr.endswith(".yaml"):
        raw_repr = deserialize_yml(args.repr)
    else:
        print(
            f"Cannot determine the format of the representation file. Should ends with .json or .yml"
        )
        exit(1)

if args.verbose:
    print("Parse representation..")

repr = Representation.parse(raw_repr)

# read the resources
if args.verbose:
    print("Read resources..")
resources = {}
for r in args.resource:
    if r.find("=") == -1:
        print(f"Invalid resource format. Expect `=`")
        parser.print_help()
        exit(1)
    resource_id, resource_path = r.split("=")
    if not Path(resource_path).exists():
        print(f"Resource file: {resource_path} does not exist")
        exit(1)
    resources[resource_id] = resource_path

# map data to RDF
ds = DataSource(repr, resources)
sg = SelectionGraph.from_sm(repr.semantic_model)

start = time.time()
triples = ds.select(sg).exec(ReturnFormat.Turtle)
end = time.time()

if args.output is not None:
    with open(args.output, "w") as f:
        for triple in triples:
            f.write(triple + "\n")
    print(">>> [DREPR] runtime: %.5f seconds" % (end - start))
else:
    if args.verbose:
        print(">>> Dumping RDF triples to console...\n")

    for triple in triples:
        print(triple)
