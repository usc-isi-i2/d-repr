#!/usr/bin/python
# -*- coding: utf-8 -*-

from peewee import PostgresqlDatabase, Model
from api.config import DB_NAME, DB_USER, DB_PWD, DB_HOST

db = PostgresqlDatabase(DB_NAME, user=DB_USER, password=DB_PWD, host=DB_HOST)


class BaseModel(Model):
    class Meta:
        database = db
