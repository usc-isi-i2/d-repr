from flask import jsonify, Blueprint

from api.misc.auth import get_current_user
from api.misc.form import Form, Validator
from api.models.access_token import AccessToken
from api.models.user import User

auth_bp = Blueprint('auth_bp', __name__)
login_form = Form(
    dict(email=Validator.is_not_empty(), password=Validator.is_not_empty()))


@auth_bp.route("/login", methods=["POST"])
def login():
    submission = login_form.post_json_form()
    user = User.auth(submission['email'], submission['password'])
    if user is None:
        return jsonify({
            "status": "fail",
            "message": "Invalid email or password"
        }), 401

    # 3 days
    auth_token = AccessToken.create_token(user, 24 * 3)
    return jsonify({"status": "success", "auth_token": auth_token.token})


@auth_bp.route("/has_authority", methods=["HEAD"])
def has_authority():
    if get_current_user() is not None:
        return "", 200
    return "", 401
