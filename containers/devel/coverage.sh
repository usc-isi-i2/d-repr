#!/usr/bin/env bash

set -e

cargo tarpaulin --out Xml
pycobertura show --format html cobertura.xml --output cobertura.html