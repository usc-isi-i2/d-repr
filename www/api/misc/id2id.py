#!/usr/bin/python
# -*- coding: utf-8 -*-

from uuid import uuid4


class ID2ID(object):

    def __init__(self):
        self.cache_key = {}

    def get_id(self, key: str):
        if key not in self.cache_key:
            uid = str(uuid4()).replace("-", "")
            self.cache_key[key] = uid
        return self.cache_key[key]
