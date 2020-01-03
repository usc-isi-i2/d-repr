class DataProp:
    predicate: str
    attr_id: str


class ObjectProp:
    predicate: str
    target_class: Class
    attr_id: str


class Class:
    id: str
    name: str
    subj_attr: str
    data_props: List[DataProp]
    object_props: List[ObjectProp]


class ArrayBackend:
    """
    Backend for the output in arrays. In particular, each property of a class is an array. 
    A class will contains the subject property (similar to primary key column). It also has alignments between
    two subject properties of two classes and between the subject and data properties of a class, provided as
    a function alignments(source_id, target_id).

    The backend need to support a function that get a record based on some key. The key here is different to the 
    URI of a record, and it will be the index in the subject's array.
    To iter through all records of a class, we only need to loop through each index. 
    """
    def __init__(self, sm: SemanticModel, classes: Dict[str, Dict[str, NDArrayColumn]],
                 alignments: Dict[Tuple[str, str], Alignment]):
        """
        @param classes 
        @param alignments get alignment from (source_id, target_id) where source_id and target_id are IDs of attributes.
        """
        self.sm = sm
        self.classes = classes
        self.alignments = alignments

    def get_class_by_name():
        pass

class RecordArrayBackend:

    def __init__(self, class_id: str):


    def __getattr__(self, name):
