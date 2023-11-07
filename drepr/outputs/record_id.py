from typing import List, Dict, Tuple, Callable, Any, Optional, Union


class BlankRecordID(tuple):
    def __new__(cls, index: Tuple[int, ...], class_id: str):
        return super().__new__(cls, index)

    def __init__(self, index: Tuple[int, ...], class_id: str):
        self.index = index
        self.class_id = class_id


class URIRecordID(str):
    def __new__(cls, uri: str, index: Any, class_id: str):
        return super().__new__(cls, uri)

    def __init__(self, uri: str, index: Any, class_id: str):
        self.index = index
        self.class_id = class_id


class GroupRecordID(str):
    def __new__(cls, uri: str, indice: List[Any], class_id: str):
        return super().__new__(cls, uri)

    def __init__(self, uri: str, indice: List[Any], class_id: str):
        self.indice = indice
        self.class_id = class_id


class GraphRecordID(str):
    def __new__(cls, id: str, class_id: str) -> Any:
        return super().__new__(cls, id)

    def __init__(self, id: str, class_id: str):
        self.class_id = class_id

    def __copy__(self):
        return GraphRecordID(self, self.class_id)

    def __deepcopy__(self, memodict={}):
        return GraphRecordID(self, self.class_id)


ArrayRecordID = Union[BlankRecordID, URIRecordID, GroupRecordID]
RecordID = Union[GraphRecordID, ArrayRecordID]


if __name__ == '__main__':
    rid = URIRecordID("https://example.org", (1, 2, 3))
    print(rid)
    print(rid.index)