from typing import List, Dict, Tuple, Callable, Any, Optional

import numpy as np

from drepr.outputs.array_backend.array_backend import ArrayBackend
from drepr.outputs.base_output_sm import BaseOutputSM
from drepr.outputs.namespace import Namespace


def test_get_prop_as_ndarray(s01: List[BaseOutputSM], s02: List[BaseOutputSM], s03: List[BaseOutputSM]):
    array_values = [
        # ds01
        [[-0.33390343, 0.07124988, 0.72986975, -0.24433717, 0.36373665],
         [0.11689817, -0.43058764, 1.78643916, 0.1838721, -0.65895388],
         [0.53011395, -0.10577698, 0.42899604, 1.09749187, -0.68841564],
         [-1.26742952, -0.65940623, -0.91022476, -0.25552528, -0.18210168],
         [0.48675445, 1.02092747, -0.96355687, -0.38728761, -0.53733528]],

        # ds02
        [[-0.33390343, 0.07124988, 0.72986975, -0.24433717, 0.36373665],
         [0.11689817, -0.43058764, 1.78643916, 0.1838721, -0.65895388],
         [0.53011395, -0.10577698, 0.42899604, 1.09749187, -0.68841564],
         [-1.26742952, -0.65940623, -0.91022476, -0.25552528, -0.18210168],
         [0.48675445, 1.02092747, -0.96355687, -0.38728761, -0.53733528]],

        # ds03
        [[-0.33390343, 0.07124988, 0.72986975, -0.24433717, 0.36373665],
         [0.11689817, -0.43058764, 1.78643916, 0.1838721, -0.65895388],
         [0.53011395, -0.10577698, 0.42899604, 1.09749187, -0.68841564],
         [-1.26742952, -0.65940623, -0.91022476, -0.25552528, -0.18210168],
         [0.48675445, 1.02092747, -0.96355687, -0.38728761, -0.53733528]],
        [[-0.12108972, 0.30380936, 0.64028664, 0.26100528, 0.88736269],
         [-0.15377342, 1.20728663, -1.80140146, -0.74886095, 0.33814465],
         [-0.97048698, -0.51121816, -0.43715448, 0.7265014, -1.09105628],
         [-1.46563961, -0.77755875, -0.67079213, -0.06328761, 0.31931191],
         [-0.69737306, 0.79152992, -0.35785484, -1.37099527, 1.39819097]]
    ]
    graph_values = [
        np.asarray(x).reshape(-1).tolist()
        for x in array_values
    ]
    for sm in (item for lst in [s01, s02, s03] for item in lst):
        mint = sm.ns("https://mint.isi.edu/")
        rdf = sm.ns(Namespace.RDF)
        mint_geo = sm.ns("https://mint.isi.edu/geo")

        for c in sm.c(mint.Variable):
            data = c.p(rdf.value).as_ndarray([c.p(mint_geo.lat), c.p(mint_geo.long)])
            assert len(data.index_props) == 2

            records = list(c.iter_records())
            assert data.data.size == len(records)

            if isinstance(sm, ArrayBackend):
                assert np.allclose(data.data, np.asarray(array_values.pop(0)))
                assert len(data.data.shape) == 2
                # loop to validate relationships between data and index_props
                for i, j in np.ndindex(*data.data.shape):
                    idx = np.ravel_multi_index(([i], [j]), data.data.shape)[0]
                    assert records[idx].s(rdf.value) == data.data[i, j]
                    assert records[idx].s(mint_geo.lat) == data.index_props[0][i]
                    assert records[idx].s(mint_geo.long) == data.index_props[1][j]
            else:
                assert np.allclose(data.data, np.asarray(graph_values.pop(0)))
                assert len(data.data.shape) == 1
                # loop to validate relationships between data and index_props
                for i, in np.ndindex(*data.data.shape):
                    assert records[i].s(rdf.value) == data.data[i]
                    assert records[i].s(mint_geo.lat) == data.index_props[0][i]
                    assert records[i].s(mint_geo.long) == data.index_props[1][i]
    assert len(array_values) == 0 and len(graph_values) == 0
