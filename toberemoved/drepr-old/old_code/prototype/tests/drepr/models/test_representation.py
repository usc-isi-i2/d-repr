#!/usr/bin/python
# -*- coding: utf-8 -*-

import pytest

from drepr.models import InvalidReprException, Representation
from tests.drepr.conftest import DSInput


@pytest.mark.parametrize("ds_invalid_input",
                         [y for ys in DSInput.get_invalid_inputs().values() for y in ys], ids=DSInput.__repr__)
def test_parse_invalid_repr(ds_invalid_input: DSInput):
    with pytest.raises(InvalidReprException):
        Representation.parse(ds_invalid_input.get_raw_repr())


def test_parse(ds_input: DSInput):
    # deserialize without any exception
    repr = Representation.parse(ds_input.get_raw_repr())

