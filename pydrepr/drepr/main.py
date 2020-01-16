#!/usr/bin/python
# -*- coding: utf-8 -*-

import argparse
import time
from pathlib import Path

import ujson
from ruamel.yaml import YAML

from drepr.engine import execute, FileOutput, OutputFormat, MemoryOutput
from drepr.models import DRepr

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-r",
        "--repr",
        required=True,
        help="either a string containing representation of a dataset (JSON format) "
             "or a path to a file containing representation (support 2 formats: JSON & YML)")

    parser.add_argument("-d",
                        "--resource",
                        required=True,
                        nargs="+",
                        help="file paths of resources in this format: <resource_id>=<file_path>;")
    parser.add_argument("-v",
                        "--verbose",
                        default=0,
                        help="increase output verbosity",
                        action="count")
    parser.add_argument("-o", "--output", help="the file we will write output to")
    parser.add_argument("-f", "--format",
                        choices=[e.value for e in OutputFormat],
                        default=OutputFormat.TTL, help="format of the data (default is ttl)")
    args = parser.parse_args()

    # read repr
    try:
        raw = ujson.loads(args.repr)
        ds_model = DRepr.parse(raw)
    except ValueError:
        if not Path(args.repr).exists():
            print(f"Cannot read the representation file at: {args.repr}")
            exit(1)
        ds_model = None

    if ds_model is None:
        ds_model = DRepr.parse_from_file(args.repr)

        if args.repr.endswith(".json"):
            with open(args.repr, "r") as f:
                raw_repr = ujson.load(f)
        elif args.repr.endswith(".yml") or args.repr.endswith(".yaml"):
            yaml = YAML()
            with open(args.repr, "r") as f:
                raw_repr = yaml.load(f)
        else:
            print(
                f"Cannot determine the format of the representation file. Should ends with .json or .yml"
            )
            exit(1)

    # read the resources
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
    if args.output is not None:
        output = FileOutput(args.output, OutputFormat(args.format))
    else:
        output = MemoryOutput(OutputFormat(args.format))

    start = time.time()
    result = execute(ds_model, resources, output, args.verbose > 1)
    end = time.time()

    if args.verbose > 0:
        print(">>> [DREPR] runtime: %.5f seconds" % (end - start))

    if args.output is None:
        print(">>> Dumping RDF triples to console...\n")
        print(result['value'])
