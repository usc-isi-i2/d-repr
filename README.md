<h1 align="center">D-REPR</h1>

<div align="center">

![PyPI](https://img.shields.io/pypi/v/drepr)
![Python](https://img.shields.io/badge/python-v3.6+-blue.svg)
[![Build Status](https://travis-ci.org/usc-isi-i2/d-repr.svg?branch=master)](https://travis-ci.org/usc-isi-i2/d-repr)
[![GitHub Issues](https://img.shields.io/github/issues/usc-isi-i2/d-repr.svg)](https://github.com/usc-isi-i2/d-repr/issues)
![Contributions welcome](https://img.shields.io/badge/contributions-welcome-orange.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

</div>

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Example](#example)
- [Contributing](#contributing)
<!-- - [Support](#support) -->

## Features

- Supporting reading datasets in heterogeneous formats (JSON, CSV, Spreadsheets, etc) and layouts (relational tables, matrix tables, etc) to the RDF format

## Installation

From PyPi: `pip install drepr`

If you want to install from source or have trouble during installation, please look in the Wiki [Installation](https://github.com/usc-isi-i2/d-repr/wiki/Installation)

## How D-REPR works

There are four steps in D-REPR to model a dataset:

1. Define resources: a resource can be a physical file in CSV, JSON format. A dataset may have multiple resources such as one main CSV file and a data-definition dictionary in a JSON file.
2. Define attributes: each attribute denotes values that belong to a group. For example, in a relational table, each column is an attribute.
3. Define alignments between attributes: a method to get a value of an attribute from a value of a corresponding attribute. The common methods are accessing by index and by value. For example, in a relational table of products, given a product id, we can retrieve the corresponding product name in the same row (by index). This step essentially defines the layout of the dataset.
4. Define a semantic model: given each attribute a type and relationships between attributes.

## Docs and Examples

Please see the paper [D-REPR: A Language for Describing and Mapping Diversely-Structured Data Sources to RDF](/docs/paper.pdf) and the [slides](/docs/slides.pdf).

The example datasets can be found in the [example folder](/examples).

## Testing

Testing rust package: `cargo test --no-default-features --features pyo3/auto-initialize`

## Contributing

Please read the Wiki [Contributing](https://github.com/usc-isi-i2/d-repr/wiki/Contributing) for details on our code of conduct, how to setup the development environment and the process for submitting pull requests to us.
