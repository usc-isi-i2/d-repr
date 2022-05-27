import time
import ujson
from itertools import chain


class Event:
    type = "event"

    def __init__(self, access_token: str, dataset_id: str, timeout: int, timestamp: float = None):
        self.access_token = access_token
        self.dataset_id = dataset_id
        self.timeout = timeout
        self.timestamp = timestamp or time.time()

    def serialize2str(self):
        return ujson.dumps(dict(chain([("type", self.type)], self.__dict__.items())))

    @staticmethod
    def deserialize4str(msg: bytes):
        obj = ujson.loads(msg)
        if obj['type'] == 'dataset_create':
            return DatasetCreateEvent(obj['access_token'], obj['dataset_id'], obj['timeout'],
                                      obj['timestamp'])
        elif obj['type'] == 'resource_create':
            return ResourceCreateEvent(obj['access_token'], obj['dataset_id'], obj['resource_id'],
                                       obj['timeout'], obj['timestamp'])
        elif obj['type'] == 'resource_delete':
            return ResourceDeleteEvent(obj['access_token'], obj['dataset_id'], obj['resource_id'],
                                       obj['timeout'], obj['timestamp'])
        elif obj['type'] == 'variable_upsert':
            return VariableUpsertEvent(obj['access_token'], obj['dataset_id'], obj['variable_id'],
                                       obj['timeout'], obj['timestamp'])
        elif obj['type'] == 'variable_upsert':
            return VariableDeleteEvent(obj['access_token'], obj['dataset_id'], obj['variable_id'],
                                       obj['timeout'], obj['timestamp'])
        elif obj['type'] == 'alignment_update':
            return AlignmentUpdateEvent(obj['access_token'], obj['dataset_id'], obj['timeout'],
                                      obj['timestamp'])
        elif obj['type'] == 'semantic_model_update':
            return SemanticModelUpdateEvent(obj['access_token'], obj['dataset_id'], obj['timeout'],
                                      obj['timestamp'])

        raise Exception("Cannot deserialize " + msg)


class DatasetCreateEvent(Event):
    type = "dataset_create"


class ResourceCreateEvent(Event):
    type = "resource_create"

    def __init__(self,
                 access_token: str,
                 dataset_id: str,
                 resource_id: str,
                 timeout: int,
                 timestamp: float = None):
        super().__init__(access_token, dataset_id, timeout, timestamp)
        self.resource_id = resource_id


class ResourceDeleteEvent(Event):
    type = "resource_delete"

    def __init__(self,
                 access_token: str,
                 dataset_id: str,
                 resource_id: str,
                 timeout: int,
                 timestamp: float = None):
        super().__init__(access_token, dataset_id, timeout, timestamp)
        self.resource_id = resource_id


class VariableUpsertEvent(Event):

    type = "variable_upsert"

    def __init__(self,
                 access_token: str,
                 dataset_id: str,
                 variable_id: str,
                 timeout: int,
                 timestamp: float = None):
        super().__init__(access_token, dataset_id, timeout, timestamp)
        self.variable_id = variable_id


class VariableDeleteEvent(Event):
    type = "variable_delete"

    def __init__(self,
                 access_token: str,
                 dataset_id: str,
                 variable_id: str,
                 timeout: int,
                 timestamp: float = None):
        super().__init__(access_token, dataset_id, timeout, timestamp)
        self.variable_id = variable_id


class AlignmentUpdateEvent(Event):
    type = "alignment_update"


class SemanticModelUpdateEvent(Event):
    type = "semantic_model_update"
