from dataclasses import dataclass
from enum import Enum
from typing import Optional, Union

from .path import Path


@dataclass
class PMap:
    resource_id: str
    path: Path
    code: str
    output: Optional[str] = None
    change_structure: Optional[bool] = None


@dataclass
class PFilter:
    resource_id: str
    path: Path
    code: str
    output: Optional[str] = None


@dataclass
class PSplit:
    resource_id: str
    path: Path
    code: str
    output: Optional[str] = None


class RMapFunc(Enum):
    Dict2Items = "dict2items"


@dataclass
class RMap:
    resource_id: str
    path: Path
    func_id: RMapFunc
    output: Optional[str] = None


class PreprocessingType(Enum):
    pmap = "pmap"
    pfilter = "pfilter"
    psplit = "psplit"
    rmap = "rmap"


@dataclass
class Preprocessing:
    type: PreprocessingType
    value: Union[PMap, PFilter, RMap]

    @staticmethod
    def deserialize(raw: dict):
        type = PreprocessingType(raw['type'])
        raw['value']['path'] = Path.deserialize(raw['value']['path'])
        if type == PreprocessingType.pmap:
            value = PMap(**raw['value'])
        elif type == PreprocessingType.pfilter:
            value = PFilter(**raw['value'])
        elif type == PreprocessingType.psplit:
            value = PSplit(**raw['value'])
        elif type == PreprocessingType.rmap:
            value = RMap(**raw['value'])
        else:
            raise NotImplementedError()

        return Preprocessing(type, value)

