import numpy as np
from dataclasses import dataclass
from typing import List, Dict, Iterable, Any, Tuple, TYPE_CHECKING

from drepr.models import Alignment, defaultdict, RangeAlignment
from drepr.outputs.array_based.array_attr import Attribute
if TYPE_CHECKING:
    from drepr.outputs.array_based.array_backend import ArrayBackend
from drepr.outputs.array_based.array_record import ArrayRecord
from drepr.outputs.array_based.index_map_func import PK2AttrFunc, IndexMapFunc, IdentityFunc
from drepr.outputs.array_based.indexed_sm import IndexedSM, SMClass
from drepr.outputs.array_based.types import AttrID, RecordID


class ArrayClass:
    def __init__(self, backend: 'ArrayBackend', cls: SMClass):
        self.id = cls.node_id
        self.label = cls.label
        self.pk_attr = backend.attrs[cls.pk_attr]
        self.uri_attr = None
        # contains both values of data property and object property.
        self.data_attr_ids = []
        for lst in cls.predicates.values():
            for p in lst:
                if p.label != "drepr:uri":
                    self.data_attr_ids.append((p.label, p.data_id))
                else:
                    self.uri_attr = backend.attrs[p.data_id]

        self.data_attrs: List[Attribute] = [
            backend.attrs[aid]
            for _1, aid in self.data_attr_ids
        ]
        self.pk2attr_funcs: List[IndexMapFunc] = [
            self._get_imfunc(backend.alignments[self.pk_attr.id, aid])
            if self.pk_attr.id != aid else IdentityFunc()
            for _1, aid in self.data_attr_ids
        ]
        if self.uri_attr is not None:
            if self.pk_attr.id == self.uri_attr.id:
                self.pk2uri_func = IdentityFunc()
            else:
                self.pk2uri_func = self._get_imfunc(backend.alignments[self.pk_attr.id, self.uri_attr.id])

        self.pred2attrs: Dict[str, List[int]] = defaultdict(lambda: [])
        for i, x in enumerate(self.data_attr_ids):
            self.pred2attrs[x[0]].append(i)

    def iter_records(self):
        """
        Iterate through all records of this class
        """
        shape = self.pk_attr.shape
        if len(shape) == 0:
            yield ArrayRecord((0,), self)
        elif len(shape) == 1:
            for i in range(shape[0]):
                yield ArrayRecord((i,), self)
        elif len(shape) == 2:
            for i in range(shape[0]):
                for j in range(shape[1]):
                    yield ArrayRecord((i, j), self)
        elif len(shape) == 3:
            for i in range(shape[0]):
                for j in range(shape[1]):
                    for k in range(shape[2]):
                        yield ArrayRecord((i, j, k), self)
        elif len(shape) == 4:
            for i in range(shape[0]):
                for j in range(shape[1]):
                    for k in range(shape[2]):
                        for p in range(shape[3]):
                            yield ArrayRecord((i, j, k, p), self)
        else:
            rid = [0] * len(shape)
            for r in self._recur_iter_record(shape, 0, rid):
                yield r

    def get_record_by_id(self, rid: RecordID):
        """
        Get a record of the class by id
        """
        return ArrayRecord(rid, self)

    def get_predicates_as_ndarray(self, pred_id: int, index_preds: List[int]) -> np.ndarray:
        """
        Get edge data (identified by `edge_id` as a high-dimensional array). The original data may already be in high-dimension
        array or may be not, but the returned value must be a high-dimensional array.

        The supplied `index_edges` are list of edges that will occupied first dimensions. If an edge in index_edges are
        high-dimensional array as well, then its value will be flatten. Each

        There must be an alignment between the edge_id and other index edges (the alignment represent the join). The alignment
        must be dimension alignment for now (then we don't need to do a join but only swapping and arranging dimension). In case the
        alignment are chained, then we have to join, and create new table?
        """
        pass

    def filter(self, condition) -> 'ArrayClass':
        pass

    def group_by(self, predicate: str) -> Iterable[Tuple[Any, 'ArrayClass']]:
        # perform group by the predicate id
        # if self.pred2attrs[predicate]
        # self.pred2attrs[predicate]
        pass

    def _get_imfunc(self, alignments: List[Alignment]) -> PK2AttrFunc:
        # constraint of the array-backend class, we have it until we figure out how to
        # handle chained join or value join efficiency. This constraint is supposed to
        # be always satisfied since it has been checked before writing data to array-backend
        # format
        assert len(alignments) == 1 and isinstance(alignments[0], RangeAlignment)
        # TODO: fix me! bug if there is missing values
        target2source = [s.source_idx - 1 for s in sorted(alignments[0].aligned_steps, key=lambda s: s.target_idx)]
        return PK2AttrFunc(target2source)

    def _recur_iter_record(self, shp: Tuple[int, ...], dim: int, rid: Tuple[int, ...]):
        if dim == len(shp) - 1:
            for i in range(shp[dim]):
                rid[dim] = i
                yield ArrayRecord(rid, self)
        else:
            for i in range(shp[dim]):
                rid[dim] = i
                for r in self._recur_iter_record(shp, dim+1, rid):
                    yield r
        
