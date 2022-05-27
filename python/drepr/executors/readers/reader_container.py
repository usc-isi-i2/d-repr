from typing import List, Dict, Tuple, Callable, Any, Optional

from drepr.executors.readers.ra_reader import NDArrayReader


class ReaderContainer:
    """
    This contains the list of ndarray reader that can be injected by the users so that they can extend the
    coverage of the system
    """
    instance = None

    def __init__(self):
        self.readers = {}

    @staticmethod
    def get_instance():
        if ReaderContainer.instance is None:
            ReaderContainer.instance = ReaderContainer()
        return ReaderContainer.instance

    def set(self, reader_id: str, reader: NDArrayReader):
        self.readers[reader_id] = reader

    def get(self, reader_id: str):
        return self.readers[reader_id]

    def delete(self, reader_id: str):
        if reader_id in self.readers:
            del self.readers[reader_id]

