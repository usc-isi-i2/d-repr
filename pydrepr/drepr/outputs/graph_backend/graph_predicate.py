from typing import List, Dict, Tuple, Callable, Any, Optional, TYPE_CHECKING
import numpy as np
from drepr.models import SemanticModel, DRepr, Alignment, RangeAlignment, Attr, Edge
from drepr.outputs.array_backend.array_attr import NoData
from drepr.outputs.base_output_predicate import BaseOutputPredicate
from drepr.outputs.base_output_class import BaseOutputClass

if TYPE_CHECKING:
    from drepr.outputs.graph_backend.graph_class import GraphClass
from drepr.outputs.prop_data_ndarray import PropDataNDArray, IndexPropRange


class GraphPredicate(BaseOutputPredicate):

    def __init__(self, drepr: DRepr, cls: 'GraphClass', edge: Edge) -> None:
        super().__init__()
        self.sm = drepr.sm
        self.cls = cls
        self.attrs = drepr.attrs
        self.id = edge.edge_id
        self.uri = edge.label
        self.edge = edge

    def as_ndarray(self, indexed_predicates: List[BaseOutputPredicate]) -> PropDataNDArray:
        attr_id = self.sm.nodes[self.edge.target_id].attr_id
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
            assert len(self.cls.p(p.uri)) == 1

        n_records = len(self.cls)
        if n_records == 0:
            data = np.asarray([])
            index_props_range = [IndexPropRange(0, 0) for _ in indexed_predicates]
            index_props = [np.asarray([]) for p in indexed_predicates]
        else:
            record = next(self.cls.iter_records())
            data = np.fromiter((r.s(self.uri) for r in self.cls.iter_records()), type(record.s(self.uri)), count=n_records)
            index_props_range = [IndexPropRange(0, data.shape[0]) for _ in indexed_predicates]
            index_props = [
                np.fromiter((r.s(p.uri) for r in self.cls.iter_records()), type(record.s(p.uri)), count=n_records)
                for p in indexed_predicates
            ]
        return PropDataNDArray(data, nodata, index_props_range, index_props)

    def o(self) -> Optional[BaseOutputClass]:
        return self.cls.backend.cid(self.sm.edges[self.id].target_id)

