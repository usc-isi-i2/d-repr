#!/usr/bin/python
# -*- coding: utf-8 -*-
from typing import *

from peewee import IntegerField, CharField, DoesNotExist
from werkzeug.security import generate_password_hash, check_password_hash

from api.models.base import BaseModel


class User(BaseModel):
    class Meta:
        table_name = "users"

    email = CharField(unique=True, max_length=128)
    password = CharField(max_length=128)

    @staticmethod
    def create(email: str, password: str) -> 'User':
        user = User(
            email=email,
            password=generate_password_hash(password, salt_length=12))
        user.save()
        return user

    @staticmethod
    def auth(email: str, password: str) -> Optional['User']:
        try:
            user = User.get(User.email == email)
        except DoesNotExist:
            return None

        if check_password_hash(user.password, password):
            return user
        return None
