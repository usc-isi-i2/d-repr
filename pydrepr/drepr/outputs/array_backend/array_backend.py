from typing import *

from drepr.engine import complete_description
from drepr.executors.cf_convention_map.cf_convention_map import CFConventionNDArrayMap
from drepr.models import Alignment, DRepr, SemanticModel, defaultdict
from drepr.outputs.array_backend.array_attr import Attribute
from drepr.outputs.array_backend.array_class import ArrayClass
from drepr.outputs.array_backend.indexed_sm import IndexedSM
from drepr.outputs.array_backend.lst_array_class import LstArrayClass
from drepr.outputs.record_id import ArrayRecordID
from drepr.outputs.base_output_sm import BaseOutputSM, identity
from drepr.outputs.base_record import BaseRecord
from drepr.outputs.base_lst_output_class import BaseLstOutputClass
from drepr.outputs.base_output_class import BaseOutputClass


class ArrayBackend(BaseOutputSM):
    """
    Array-based output. In particular, each property of a class is an array.
    A class will contains the subject property (similar to primary key column). It also has alignments between
    two subject properties of two classes and between the subject and data properties of a class, provided as
    a function alignments(source_id, target_id).

    The backend need to support a function that get a record based on some key. The key here is different to the 
    URI of a record, and it will be the index in the subject's array.
    To iter through all records of a class, we only need to loop through each index. 
    """

    def __init__(self, sm: SemanticModel, attrs: Dict[str, Attribute],
                 alignments: Dict[Tuple[str, str], List[Alignment]], inject_class_id: Callable[[str], str]):
        """
        @param sm
        @param attrs
        @param alignments get alignment from (source_id, target_id) where source_id and target_id are IDs of attributes.
        """
        # semantic model
        self.sm = sm
        # a mapping from attribute id to the attribute
        self.attrs = attrs
        # a mapping from a pair of two attributes to their alignments
        self.alignments = alignments
        # a mapping from node id to class node
        self.classes: Dict[str, ArrayClass] = {}
        self.uri2classes: Dict[str, List[ArrayClass]] = defaultdict(list)

        for c in sm.iter_class_nodes():
            self.classes[c.node_id] = ArrayClass(self, inject_class_id(c.node_id))

        for c in self.classes.values():
            c._init_schema()

        for c in self.classes.values():
            c._init_data()
            self.uri2classes[c.uri].append(c)

    @classmethod
    def from_drepr(cls, ds_model: Union[DRepr, str], resources: Union[str, Dict[str, str]], inject_class_id: Callable[[str], str]=None) -> BaseOutputSM:
        if type(ds_model) is str:
            ds_model = DRepr.parse_from_file(ds_model)

        resource_file = next(iter(resources.values())) if isinstance(resources, dict) else resources
        plan = complete_description(ds_model)
        result, attrs = CFConventionNDArrayMap.execute(ds_model, resource_file)
        return cls(plan.sm, attrs, plan.alignments, inject_class_id or identity)

    def iter_classes(self) -> Iterable[BaseOutputClass]:
        return iter(self.classes.values())

    def get_record_by_id(self, rid: ArrayRecordID) -> BaseRecord:
        return self.classes[rid.class_id].get_record_by_id(rid)

    def c(self, class_uri: str) -> BaseLstOutputClass:
        return LstArrayClass(self.uri2classes[class_uri])

    def cid(self, class_id: str) -> ArrayClass:
        return self.classes[class_id]

    def get_sm(self) -> SemanticModel:
        return self.sm