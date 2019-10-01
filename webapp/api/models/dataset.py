#!/usr/bin/python
# -*- coding: utf-8 -*-
from peewee import ForeignKeyField, CharField, TextField
from playhouse.postgres_ext import JSONField

from api.models.base import BaseModel
from api.models.user import User
from api.models.repr.representation import Representation


class Dataset(BaseModel):
    class Meta:
        table_name = "datasets"
        indexes = ((('owner', 'name'), True), )

    owner = ForeignKeyField(User, null=False, backref="datasets")
    name = CharField(max_length=512)
    description = TextField()
    representation: dict = JSONField(null=False)

    def get_repr(self) -> Representation:
        return Representation.deserialize(self.representation)

    def set_repr(self, repr: Representation):
        self.representation = repr.serialize()

    def get_resource_db_id(self, resource_id: str) -> int:
        return Representation.unsafe_get_resource_db_id(self.representation, resource_id)

    def refresh(self):
        newer = type(self).get(self._pk_expr())
        for field_name in self._meta.fields.keys():
            val = getattr(newer, field_name)
            setattr(self, field_name, val)
        self._dirty.clear()