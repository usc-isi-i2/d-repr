from typing import List, Dict, Tuple, Callable, Any, Optional


class BlankRecordID(tuple):
    def __new__(cls, index: Tuple[int, ...], class_id: str):
        return super().__new__(cls, index)

    def __init__(self, index: Tuple[int, ...], class_id: str):
        self.index = index
        self.class_id = class_id


class RecordID(str):
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


if __name__ == '__main__':
    rid = RecordID("https://example.org", (1, 2, 3))
    print(rid)
    print(rid.index)