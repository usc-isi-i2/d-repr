from typing import Optional

from flask import request
from peewee import DoesNotExist
from functools import wraps
from api.misc.exception import UnauthorizedRequest
from api.models.access_token import AccessToken
from api.models.user import User


def get_auth_token():
    return request.headers.get("Authorization")


def get_current_user() -> Optional[User]:
    auth_token = request.headers.get("Authorization", "")
    try:
        auth_token: Optional[AccessToken] = AccessToken.get(AccessToken.token == auth_token)
    except DoesNotExist:
        return None

    if not auth_token.is_valid():
        return None

    return auth_token.owner


def auth(func):
    @wraps(func)
    def wrapped_func(*args, **kwargs):
        user = get_current_user()
        if user is None:
            raise UnauthorizedRequest()

        results = func(*args, **kwargs)
        return results

    return wrapped_func
