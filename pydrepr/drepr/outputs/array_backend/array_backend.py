from typing import *

from drepr.engine import complete_description
from drepr.executors.cf_convention_map.cf_convention_map import CFConventionNDArrayMap
from drepr.models import Alignment, DRepr

from drepr.outputs.array_based.array_attr import Attribute
from drepr.outputs.array_based.array_class import ArrayClass
from drepr.outputs.array_based.array_record import ArrayRecord
from drepr.outputs.array_based.indexed_sm import IndexedSM
from drepr.outputs.array_based.lst_array_class import LstArrayClass
from drepr.outputs.array_based.record_id import RecordID


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

    @staticmethod
    def from_drepr(drepr_file: str, resources: Union[str, Dict[str, str]]):
        ds_model = DRepr.parse_from_file(drepr_file)
        resource_file = next(resources.values()) if isinstance(resources, dict) else resources
        plan = complete_description(ds_model)
        result, attrs = CFConventionNDArrayMap.execute(ds_model, resource_file)
        sm = IndexedSM(plan.sm)
        return ArrayBackend(sm, attrs, plan.alignments)

    def get_class_by_id(self, node_id: str):
        return self.classes[node_id]

    def get_classes_by_uri(self, cls_name: str):
        if len(self.sm.sm_classes[cls_name]) == 1:
            return self.classes[self.sm.sm_classes[cls_name][0].node_id]
        return LstArrayClass([self.classes[c.node_id] for c in self.sm.sm_classes[cls_name]])

    def get_record_by_id(self, rid: RecordID) -> ArrayRecord:
        return self.classes[rid.class_id].get_record_by_id(rid)
