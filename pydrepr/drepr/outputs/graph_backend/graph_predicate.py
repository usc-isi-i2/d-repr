from typing import List, Dict, Tuple, Callable, Any, Optional, TYPE_CHECKING
import numpy as np
from drepr.models import SemanticModel, DRepr, Alignment, RangeAlignment, Attr, Edge, ClassNode
from drepr.outputs.array_backend.array_attr import NoData
from drepr.outputs.base_output_predicate import BaseOutputPredicate
from drepr.outputs.base_output_class import BaseOutputClass
from drepr.outputs.graph_backend.lst_graph_class import LstGraphClass

if TYPE_CHECKING:
    from drepr.outputs import GraphBackend
    from drepr.outputs.graph_backend.graph_class import GraphClass
from drepr.outputs.prop_data_ndarray import PropDataNDArray, IndexPropRange


class GraphPredicate(BaseOutputPredicate):

    def __init__(self, backend: 'GraphBackend', cls: 'GraphClass', edges: List[Edge]) -> None:
        super().__init__()
        self.sm = backend.sm
        self.cls = cls
        self.attrs = backend.drepr.attrs
        self.uri = edges[0].label
        self.edges = edges
        self.targets: Optional[LstGraphClass] = None

    def _init(self, backend: 'GraphBackend'):
        if len(self.edges) > 0:
            self.targets = LstGraphClass([
                backend.cid(edge.target_id)
                for edge in self.edges
                if isinstance(backend.sm.nodes[edge.target_id], ClassNode)
            ])
            assert len(self.targets) == len(self.edges)

    def ndarray_size(self) -> int:
        return len(self.cls)

    def as_ndarray(self, indexed_predicates: List[BaseOutputPredicate]) -> PropDataNDArray:
        if len(self.edges) > 1 or any(len(p.edges) > 1 for p in indexed_predicates):
            raise Exception("Cannot convert values of this predicate to an ndarray indexed by other predicates "
                            "because values for one entry may be greater than one")

        attr_id = self.sm.nodes[self.edges[0].target_id].attr_id
        nodata = None
        for attr in self.attrs:
            if attr.id == attr_id:
                if len(attr.missing_values) > 0:
                    # the engine already converted all missing values to None
                    nodata = NoData(None)
                break

        # cannot handle the case where uri of one of the index predicate is duplicated, since we cannot retrieve correct
        # value using graph backend, if we want to support it, we need to modify the backend to keep the null value as None
        for p in indexed_predicates:
            assert len(p.edges) == 1

        n_records = len(self.cls)
        if n_records == 0:
            data = np.asarray([])
            index_props_range = [IndexPropRange(0, 0) for _ in indexed_predicates]
            index_props = [np.asarray([]) for p in indexed_predicates]
        else:
            record = next(self.cls.iter_records())
            data = np.fromiter((r.s(self.uri) for r in self.cls.iter_records()), type(record.s(self.uri)), count=n_records)
            index_props_range = [IndexPropRange(0, 0) for _ in indexed_predicates]
            index_props = [
                np.fromiter((r.s(p.uri) for r in self.cls.iter_records()), type(record.s(p.uri)), count=n_records)
                for p in indexed_predicates
            ]
        return PropDataNDArray(data, nodata, index_props_range, index_props)

    def o(self) -> Optional[LstGraphClass]:
        return self.targets

