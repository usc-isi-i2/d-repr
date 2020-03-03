import enum
from abc import ABC, abstractmethod
from typing import Dict, Union, List, Iterable, Any, Optional

from drepr import DRepr
from drepr.models import SemanticModel
from drepr.outputs.record_id import RecordID
from drepr.outputs.base_lst_output_class import BaseLstOutputClass
from drepr.outputs.base_output_class import BaseOutputClass
from drepr.outputs.base_output_predicate import BaseOutputPredicate
from drepr.outputs.base_record import BaseRecord
from drepr.outputs.namespace import Namespace, PrefixedNamespace


class BaseOutputSM(ABC):

    @classmethod
    @abstractmethod
    def from_drepr(cls, ds_model: Union[DRepr, str], resources: Union[str, Dict[str, str]]) -> "BaseOutputSM":
        pass

    @abstractmethod
    def iter_classes(self) -> Iterable[BaseOutputClass]:
        pass

    @abstractmethod
    def get_record_by_id(self, rid: RecordID) -> BaseRecord:
        pass

    @abstractmethod
    def c(self, class_uri: str) -> BaseLstOutputClass:
        """Get list of classes based on their URIs"""
        pass

    @abstractmethod
    def cid(self, class_id: str) -> BaseOutputClass:
        """Get a class based on its id in the semantic model"""
        pass

    @abstractmethod
    def get_sm(self) -> SemanticModel:
        pass

    def ns(self, uri: str) -> Namespace:
        sm = self.get_sm()
        if not hasattr(sm, '_inverted_prefixes'):
            sm._inverted_prefixes = {v: k for k, v in sm.prefixes.items()}

        if uri in sm._inverted_prefixes:
            return PrefixedNamespace(uri, sm._inverted_prefixes[uri])
        return Namespace(uri)


class FCondition:
    def __init__(self, predicate_uri: str, op: str, val: Optional[Any]=None):
        self.predicate_uri = predicate_uri
        self.op = FConditionOp(op)
        self.val = val


class FConditionOp(enum.Enum):
    eq = "=="
    gt = ">"
    gte = ">="
    lt = "<"
    lte = "<="
    is_in = "is_in"
