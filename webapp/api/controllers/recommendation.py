from flask import Blueprint

from api.misc.auth import auth, get_current_user

recommendation_bp = Blueprint("recommendation_bp", __name__)
inmem_cache = {}


@recommendation_bp.route("/suggestions/ack", methods=["GET"])
@auth
def acknowledgement():
    # TODO: fix me!
    global inmem_cache
    user = get_current_user()
    return user.email
