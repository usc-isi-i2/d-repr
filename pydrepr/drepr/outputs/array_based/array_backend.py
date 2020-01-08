from typing import *

from drepr.models import Alignment

from drepr.outputs.array_based.array_attr import Attribute
from drepr.outputs.array_based.array_class import ArrayClass
from drepr.outputs.array_based.indexed_sm import IndexedSM


class ArrayBackend:
    """
    Array-based output. In particular, each property of a class is an array.
    A class will contains the subject property (similar to primary key column). It also has alignments between
    two subject properties of two classes and between the subject and data properties of a class, provided as
    a function alignments(source_id, target_id).

    The backend need to support a function that get a record based on some key. The key here is different to the 
    URI of a record, and it will be the index in the subject's array.
    To iter through all records of a class, we only need to loop through each index. 
    """
    def __init__(self, sm: IndexedSM, attrs: Dict[str, Attribute],
                 alignments: Dict[Tuple[str, str], List[Alignment]]):
        """
        @param sm
        @param attrs
        @param alignments get alignment from (source_id, target_id) where source_id and target_id are IDs of attributes.
        """
        self.sm = sm
        self.attrs = attrs
        self.alignments = alignments
        self.classes = {}
        for lst in sm.sm_classes.values():
            for c in lst:
                self.classes[c.node_id] = ArrayClass(self, c)

    def get_class_by_id(self, node_id: str):
        return self.classes[node_id]

    def iter_classes_by_name(self, cls_name: str):
        for c in self.sm.sm_classes[cls_name]:
            yield self.classes[c.node_id]
