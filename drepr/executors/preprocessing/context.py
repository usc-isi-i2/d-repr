from typing import Dict

from drepr.executors.readers.ra_reader import NDArrayReader


class Context:
    def __init__(self, reader: NDArrayReader):
        self.reader = reader

    def get_value(self, index):
        return self.reader.get_value(index)