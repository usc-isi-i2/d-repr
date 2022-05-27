from copy import copy
from typing import List, Dict, Tuple, Callable, Any, Optional

from drepr.services.index_iterator import IndexIterator, ReversedIndexIterator, DynamicIndexIterator, \
    ReversedDynamicIndexIterator
from drepr.services.ra_reader.tensor_ra_reader import TensorRAReader


def test_index_iterator():
    assert [copy(x) for x in IndexIterator([1, 2], [0, 0], [1, 1])] == [[0, 0], [0, 1]]
    assert [copy(x) for x in IndexIterator([2, 0], [0, 0], [1, 0])] == [[0, 0], [1, 0]]
    assert [copy(x) for x in IndexIterator(['name', 3], ['name', 0], [0, 1])] == [['name', 0],
                                                                                  ['name', 1],
                                                                                  ['name', 2]]


def test_reversed_index_iterator():
    for upperbound, lowerbound, steps in [([1, 2], [0, 0], [1, 1]), ([2, 0], [0, 0], [1, 0]),
                                          ([3, 4, 5, 3], [3, 4, 0, 0], [0, 0, 1, 1])]:
        assert [copy(x) for x in ReversedIndexIterator(upperbound, lowerbound, steps)] == list(
            reversed([copy(x) for x in IndexIterator(upperbound, lowerbound, steps)]))


def test_dyn_index_iterator():
    data = [{
        'a': [2, 3, 5, 10, 11, 23],
        'b': [{
            'b1': [3, 4, 11, 12],
        }, {
            'b1': [15, 2, 12]
        }]
    }, {
        'a': [5, 3, 2],
        'b': [{
            'b1': [0]
        }]
    }]

    ra_reader = TensorRAReader(data, (len(data), ))
    args = [[[2, 'a', None], [0, 'a', 0], [1, 0, 1]],
            [[2, 'b', None, 'b1', None], [0, 'b', 0, 'b1', 0], [1, 0, 1, 0, 1]]]

    dyn_iter = DynamicIndexIterator(ra_reader, *args[0])
    assert [copy(x) for x in dyn_iter] == [[0, 'a', x] for x in range(6)] + [[1, 'a', x]
                                                                             for x in range(3)]

    dyn_iter = DynamicIndexIterator(ra_reader, *args[1])
    assert [copy(x) for x in dyn_iter] == \
           [[0, 'b', 0, 'b1', x] for x in range(4)] + \
           [[0, 'b', 1, 'b1', x] for x in range(3)] + \
           [[1, 'b', 0, 'b1', x] for x in range(1)]

    for i in range(len(args)):
        assert [copy(x) for x in DynamicIndexIterator(ra_reader, *args[i])] == list(
            reversed([copy(x) for x in ReversedDynamicIndexIterator(ra_reader, *args[i])]))


def test_dyn_index_iterator_extra():
    data = [None, None, ['nMx - age-specific death rate between ages x and x+n',
     ' &lt;1 year',
     ['0.068', '0.058'],
     ['0.07', '0.06'],
     ['0.072', '0.061'],
     ['0.074', '0.063'],
     ['0.076', '0.065'],
     ['0.078', '0.067'],
     ['0.081', '0.07'],
     ['0.084', '0.072'],
     ['0.088', '0.076'],
     ['0.093', '0.08'],
     ['0.099', '0.085'],
     ['0.104', '0.09'],
     ['0.109', '0.094'],
     ['0.113', '0.098'],
     ['0.117', '0.101'],
     ['0.121', '0.105'],
     ['0.127', '0.11']]]

    ra_reader = TensorRAReader(data, None)
    dyn_iter = DynamicIndexIterator(ra_reader, [2, None, None], [2, 2, 0], [0, 1, 1])
    results = [copy(x) for x in dyn_iter]
    assert results == [[2, x, y] for x in range(2, len(data[2])) for y in range(2)]
