import copy

import pytest

from drepr.models import Representation, InvalidReprException
from drepr.models import DimensionMapping


def test_list_int():
    m = {
        "source_var": "prices",
        "source_dims": [None],
        "target_var": "commodity",
        "target_dims": [None]
    }

    with pytest.raises(InvalidReprException):
        DimensionMapping.unsafe_deserialize(m)

    with pytest.raises(InvalidReprException):
        m1 = copy.deepcopy(m)
        m1['type'] = 'dimension_mapping'
        Representation.class_properties['mappings'].unsafe_deserialize([m1])
