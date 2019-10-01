#!/usr/bin/python
# -*- coding: utf-8 -*-
from datetime import datetime, timedelta
from typing import *
from uuid import uuid4

from peewee import ForeignKeyField, DateTimeField, FixedCharField

from api.models.base import BaseModel
from api.models.user import User


class AccessToken(BaseModel):
    class Meta:
        table_name = "access_tokens"

    owner = ForeignKeyField(User, backref="access_tokens", on_delete="CASCADE")
    expired_on = DateTimeField()
    token = FixedCharField(max_length=32)

    def is_valid(self):
        return self.expired_on >= datetime.now()

    @staticmethod
    def create_token(owner: User, lifespan_hours: int):
        return AccessToken.create(
            owner=owner,
            token=str(uuid4()).replace("-", ""),
            expired_on=datetime.now() + timedelta(hours=lifespan_hours))
