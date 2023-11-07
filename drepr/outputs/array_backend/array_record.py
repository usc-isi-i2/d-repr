from typing import List, Tuple, Any, TYPE_CHECKING

from drepr.outputs.record_id import RecordID
from drepr.outputs.base_record import BaseRecord

if TYPE_CHECKING:
    from drepr.outputs.array_backend.array_class import ArrayClass


class ArrayRecord(BaseRecord):
    """
    This record assumes that there is one to one mapping between an item in the pk attribute with the subject attribute.
    If this assumption does not hold, we have to perform a group by the subject attribute to gather duplicated record first.
    """

    def __init__(self, index: Tuple[int, ...], array_class: 'ArrayClass'):
        self._index = index
        self._cls = array_class

    @property
    def id(self) -> RecordID:
        """Get ID of the record"""
        return self._cls.uri_attr.get_sval(self._index)

    def s(self, pred_uri: str):
        """Get value of a predicate, guarantee to return the first single value"""
        aid = self._cls.pred2attrs[pred_uri][0]
        return self._cls.attrs[aid].get_sval(self._index)

    def m(self, pred_uri: str):
        """Get values of a predicate, always return a list"""
        result = []
        for aid in self._cls.pred2attrs[pred_uri]:
            result += self._cls.attrs[aid].get_mval(self._index)
        return result

    def us(self, predicate: str, val: Any):
        aid = self._cls.pred2attrs[predicate][0]
        self._cls.attrs[aid].set_value(self._index, val)

    def um(self, predicate: str, values: List[Any]):
        values = iter(values)
        for i, aid in enumerate(self._cls.pred2attrs[predicate]):
            self._cls.attrs[aid].set_value(self._index, values)

    def to_dict(self) -> dict:
        info = {'@id': self.id}
        for k in self._cls.pred2attrs.keys():
            info[k] = self.m(k)
        return info
