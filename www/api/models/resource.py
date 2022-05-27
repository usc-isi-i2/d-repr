#!/usr/bin/python
# -*- coding: utf-8 -*-
import csv
import io
from typing import *

import ujson
from peewee import CharField, BlobField, ForeignKeyField, AutoField
from playhouse.postgres_ext import JSONField

from api.models.base import BaseModel
from api.models.dataset import Dataset


class Resource(BaseModel):
    class Meta:
        table_name = "resources"

    db_id = AutoField(primary_key=True)
    dataset = ForeignKeyField(Dataset, backref="resources", on_delete="CASCADE")
    resource_id: str = CharField(max_length=128)
    resource: dict = JSONField(null=False)
    value: bytes = BlobField()

    def is_valid_data(self, retain: bool = False) -> bool:
        try:
            if retain:
                self.get_data()
            else:
                self.read_data()
        except NotImplementedError:
            raise
        except Exception:
            raise

        return True

    def get_data(self):
        if '_data_' not in self.__dict__:
            self.__dict__['_data_'] = self.read_data()
        return self.__dict__['_data_']

    def read_data(self) -> Any:
        if self.resource['type'] == 'csv':
            return deserialize_csv(self.value, self.resource.get('delimiter', ','))
        elif self.resource['type'] == 'json':
            return deserialize_json(self.value)
        else:
            raise NotImplementedError()


def deserialize_csv(content: bytes, delimiter: str):
    with io.TextIOWrapper(io.BytesIO(content), encoding='utf-8') as f:
        reader = csv.reader(f, delimiter=delimiter)
        return [row for row in reader]


def deserialize_json(content: bytes):
    with io.BytesIO(content) as f:
        return ujson.load(f)
