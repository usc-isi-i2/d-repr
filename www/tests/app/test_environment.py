import os
import subprocess
import sys
from pathlib import Path
from urllib.parse import urlparse
from uuid import uuid4

from dotenv import load_dotenv
from flask import Flask, jsonify

# load environment variable
webapp_dir = Path(__file__).parent.absolute().parent.parent
load_dotenv(str(webapp_dir / ".env"))
sys.path.append(str(webapp_dir))
sys.path.append(str(webapp_dir.parent / "drepr"))

from api.cmd import create_user, delete_user

app = Flask(__name__)
environments = {}


@app.route("/setup")
def setup():
    global environments

    uid = str(uuid4()).replace("-", "")
    env = {"id": uid, "username": uid, "password": "tester123"}
    environments[env['id']] = env
    create_user(uid, "tester123")

    return jsonify(env)


@app.route("/tear_down/<env_id>", methods=["GET"])
def tear_down(env_id: str):
    global environments

    env = environments.pop(env_id)
    delete_user(env['username'])
    return "", 200


if __name__ == '__main__':
    o = urlparse(os.environ["TEST_ENV_SERVER"])
    if o.netloc.find(":") != -1:
        host = o.netloc[:o.netloc.find(":")]
    else:
        host = o.netloc

    app.run(host=host, port=o.port)
