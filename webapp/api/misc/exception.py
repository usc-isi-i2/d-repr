#!/usr/bin/python
# -*- coding: utf-8 -*-

from flask.json import jsonify
from peewee import DoesNotExist


class BadRequest(Exception):
    @staticmethod
    def flask_handler(error: 'BadRequest'):
        return jsonify({"status": "error", "message": str(error)}), 400


class UnauthorizedRequest(Exception):
    @staticmethod
    def flask_handler(error: 'UnauthorizedRequest'):
        return jsonify({"status": "error", "message": "Unauthorized request"}), 401


def not_implemented_handler(error: NotImplementedError):
    return jsonify({
        "status": "error",
        "message": "A part of the function/endpoint you are requesting hasn't implemented yet"
    }), 500


def exception_handler(app):
    def handler(error: Exception):
        app.logger.exception(error)
        return jsonify({"status": "error", "message": "Internal server error"}), 500
    return handler


def handle_does_not_exist_peewee(error: DoesNotExist):
    return jsonify({"status": "error", "message": "Access to a nonexistent resource"}), 404
