from enum import Enum
from pathlib import Path
from typing import Dict

from drepr.models import Representation, CSVResource, NetCDF4Resource, JSONResource
from drepr.query.selection_interface import SelectionGraph
from drepr.services.ra_executor.preprocessing.preprocessing import exec_preprocessing
from drepr.services.ra_executor.ra_executor import exec_ra_query
from drepr.query.return_format import ReturnFormat
from drepr.services.ra_reader import CSVRAReader, JSONRAReader, NetCDFRAReader
from drepr.services.ra_reader.multi_ra_reader import MultiRAReader
from drepr.services.ra_reader.ra_reader import RAReader


class DataSource:
    """https://en.wikipedia.org/wiki/Datasource"""

    def __init__(self, repr: Representation, resources: Dict[str, str]):
        self.resources = resources
        self.repr: Representation = repr
        self.is_preprocessed: bool = False

        readers = {}
        for rid, rtype in self.repr.resources.items():
            if isinstance(rtype, CSVResource):
                reader = CSVRAReader(Path(resources[rid]), delimiter=rtype.delimiter)
            elif isinstance(rtype, NetCDF4Resource):
                reader = NetCDFRAReader(Path(resources[rid]))
            elif isinstance(rtype, JSONResource):
                reader = JSONRAReader(Path(resources[rid]))
            else:
                raise NotImplementedError()

            readers[rid] = reader

        if len(readers) == 1:
            self.reader: RAReader = next(iter(readers.values()))
        else:
            self.reader: RAReader = MultiRAReader(readers)

    def dump2json(self):
        if isinstance(self.reader, MultiRAReader):
            return self.reader.dump2json()
        return {next(iter(self.repr.resources.keys())): self.reader.dump2json()}

    def preprocess(self):
        if not self.is_preprocessed:
            exec_preprocessing(self.repr, self.reader)

    def select(self, sg: SelectionGraph) -> 'DSQueryInterface':
        self.preprocess()
        return DSQueryInterface(self, sg)


class DSQueryInterface:
    """Query data which is described by `ont`"""

    def __init__(self, data_source: DataSource, sg: SelectionGraph):
        self.data_source = data_source
        self.sg = sg

    def exec(self, return_format: ReturnFormat=ReturnFormat.JsonLD):
        if isinstance(self.data_source.reader, RAReader):
            return exec_ra_query(self.data_source.repr, self.data_source.reader, self.sg, return_format)
        else:
            raise NotImplementedError()




