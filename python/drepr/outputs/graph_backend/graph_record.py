import copy
from typing import List, Dict, Tuple, Callable, Any, Optional

from drepr.outputs.base_record import BaseRecord


class GraphRecord(BaseRecord):

    def __init__(self, data: dict) -> None:
        super().__init__()
        self.data = data

    @property
    def id(self):
        return self.data['@id']

    def s(self, predicate_uri: str) -> Any:
        try:
            return self.data[predicate_uri][0]
        except KeyError:
            return None

    def m(self, predicate_uri: str) -> Any:
        return self.data[predicate_uri]

    def us(self, predicate_uri: str, val: Any):
        if len(self.data[predicate_uri]) == 0:
            self.data[predicate_uri].append(val)
        else:
            self.data[predicate_uri][0] = val

    def um(self, predicate_uri: str, val: Any):
        self.data[predicate_uri] = val

    def to_dict(self) -> dict:
        return copy.deepcopy(self.data)
