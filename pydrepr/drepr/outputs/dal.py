class DAL(object):
    pass


class OwlClass:
    name: str
    data_props: List[DataProperty]
    obj_props: List[ObjectProperty]

    def iter(self) -> List[ProxyRecord]:
        return []
    
    def group_by(self, prop):
        pass