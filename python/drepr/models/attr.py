from dataclasses import dataclass
from enum import Enum
from typing import List, Union, Optional

from .path import Path


class Sorted(Enum):
    Null = "none"
    Ascending = "ascending"
    Descending = "descending"


class ValueType(Enum):
    Unspecified = "unspecified"
    Int = "int"
    Float = "float"
    String = "str"
    List_Int = "list[int]"
    List_Float = "list[float]"
    List_String = "list[str]"


@dataclass
class Attr:
    id: str
    resource_id: str
    path: Path
    missing_values: List[Optional[Union[str, int, float]]]
    unique: bool = False
    sorted: Sorted = Sorted.Null
    value_type: ValueType = ValueType.Unspecified

    @staticmethod
    def deserialize(raw: dict) -> "Attr":
        attr = Attr(**raw)
        attr.path = Path.deserialize(raw['path'])
        attr.sorted = Sorted(attr.sorted)
        attr.value_type = ValueType(attr.value_type)

        return attr
