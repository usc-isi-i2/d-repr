from dataclasses import dataclass
from typing import List, Dict, Tuple, Callable, Any, Optional, TYPE_CHECKING

if TYPE_CHECKING:
    from drepr.outputs.array_based.array_class import ArrayClass
from drepr.outputs.array_based.types import RecordID


@dataclass
class ArrayRecord:
    """
    This record assumes that there is one to one mapping between an item in the pk attribute with the subject attribute.
    If this assumption does not hold, we have to perform a group by the subject attribute to gather duplicated record first.
    """
    def __init__(self, id: RecordID, array_class: 'ArrayClass'):
        self._id = id
        self._cls = array_class

    @property
    def id(self):
        """
        ID of a record is not guarantee to be the `pk_attr`
        Shorthand for "drepr:uri" predicate if the uri is there. otherwise
        """
        if self._cls.uri_attr is None:
            return self._id
        return

    def s(self, predicate: str):
        """Get value of a predicate, guarantee to return the first single value"""
        aid = self._cls.pred2attrs[predicate][0]
        return self._cls.data_attrs[aid].get_value(self._cls.pk2attr_funcs[aid](self._id))

    def m(self, predicate: str):
        """Get values of a predicate, always return a list"""
        result = []
        for aid in self._cls.pred2attrs[predicate]:
            result.append(self._cls.data_attrs[aid].get_value(self._cls.pk2attr_funcs[aid](self._id)))
        return result

    def us(self, predicate: str, val: Any):
        aid = self._cls.pred2attrs[predicate][0]
        self._cls.data_attrs[aid].set_value(self._cls.pk2attr_funcs[aid](self._id), val)

    def um(self, predicate: str, values: List[Any]):
        for i, aid in enumerate(self._cls.pred2attrs[predicate]):
            self._cls.data_attrs[aid].set_value(self._cls.pk2attr_funcs[aid](self._id), values[i])

    def to_dict(self) -> dict:
        info = {'@id': self.id}
        for k in self._cls.pred2attrs.keys():
            info[k] = self.m(k)
        return info

