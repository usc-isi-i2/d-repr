#!/usr/bin/python
# -*- coding: utf-8 -*-


class Equal:

    eq_ignored_props = set()

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False

        for prop, value in self.__dict__.items():
            if prop not in self.eq_ignored_props and other.__dict__[prop] != value:
                return False

        return True
