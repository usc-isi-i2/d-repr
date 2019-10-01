#!/usr/bin/python
# -*- coding: utf-8 -*-
import argparse
import io, os, shutil
import subprocess
from pathlib import Path

from api.app import app
from api.config import HOME_DIR
from api.models.access_token import AccessToken
from api.models.base import db
from api.models.dataset import Dataset
from api.models.user import User
from api.models.resource import Resource


def setup_databases():
    with db:
        db.create_tables([User, Dataset, Resource, AccessToken])


def create_user(email: str, password: str):
    if User.get_or_none(User.email == email) is None:
        User.create(email, password)
    else:
        assert User.auth(email, password) is not None, "User already exists with different password"


def delete_user(email: str):
    user = User.get(User.email == email)
    Dataset.delete().where(Dataset.owner == user).execute()
    User.delete_by_id(user.id)


def upload_ontologies(email: str, password: str):
    client = app.test_client()
    resp = client.post('/login', json={'email': email, 'password': password})
    authorization = {'Authorization': resp.json['auth_token']}

    (HOME_DIR / "ontologies").mkdir(exist_ok=True)

    for fpath in (HOME_DIR / "ontologies").iterdir():
        if fpath.name.startswith(".") or fpath.is_dir():
            continue

        with open(str(fpath), "rb") as f:
            ont_file = (io.BytesIO(f.read()), fpath.name)

        resp = client.post(
            f"/ontologies",
            data={
                'format': fpath.suffix[1:],
                'prefix': fpath.stem,
                'namespace': {
                    'owl': "http://www.w3.org/2002/07/owl#",
                    'rdf': "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
                    'rdfs': "http://www.w3.org/2000/01/rdf-schema#",
                    'schema': 'http://schema.org/',
                    'qb': 'http://purl.org/linked-data/cube#',
                    'dcat-cube': 'http://isi.edu/ontologies/dcat-cube#',
                    'sdmx-attribute': 'http://purl.org/linked-data/sdmx/2009/attribute#',
                    'sdmx-dimension': 'http://purl.org/linked-data/sdmx/2009/dimension#'
                }[fpath.stem],
                'ontology_file': ont_file
            },
            headers=authorization)
        if resp.status_code != 200:
            assert resp.status_code == 409, 'The ontology should already exist'


def provision_webapp(email: str, password: str):
    setup_databases()
    create_user(email, password)
    upload_ontologies(email, password)


def build_webapp():
    webapp_dir = Path(os.path.abspath(__file__)).parent.parent
    shutil.rmtree(str(webapp_dir / "api/flask_files/static/webapp"))

    subprocess.check_call("npm run build && cp -a build/static ../api/flask_files/static/webapp",
                          cwd=str(webapp_dir / "app"),
                          shell=True)
    js_files = []
    for file in (webapp_dir / "api/flask_files/static/webapp/js").iterdir():
        if not file.name.endswith(".js"):
            continue
        js_files.append(file.name)

    css_files = []
    for file in (webapp_dir / "api/flask_files/static/webapp/css").iterdir():
        if not file.name.endswith(".css"):
            continue
        css_files.append(file.name)

    with open(webapp_dir / "api/flask_files/templates/index.html.template", "r") as f:
        html = f.read() \
            .replace("%1.chunk.css%", [s for s in css_files if s.startswith("1.")][0]) \
            .replace("%main.chunk.css%", [s for s in css_files if s.startswith("main.")][0]) \
            .replace("%1.chunk.js%", [s for s in js_files if s.startswith("1.")][0]) \
            .replace("%main.chunk.js%", [s for s in js_files if s.startswith("main.")][0]) \
            .replace("%runtime~main.js%", [s for s in js_files if s.startswith("runtime~")][0])

    with open(webapp_dir / "api/flask_files/templates/index.html", "w") as f:
        f.write(html)


if __name__ == '__main__':
    parser = argparse.ArgumentParser('Representation API')
    parser.add_argument('command', type=str, help="Command to execute")
    parser.add_argument('-u', "--user", help="user's email")
    parser.add_argument('-p', '--password', help="user's password")

    args = parser.parse_args()

    if args.command == "provision_webapp":
        provision_webapp(args.user, args.password)
    elif args.command == "setup_db":
        setup_databases()
    elif args.command == "create_user":
        create_user(args.user, args.password)
    elif args.command == "delete_user":
        delete_user(args.user)
    elif args.command == "upload_ontologies":
        upload_ontologies(args.user, args.password)
    elif args.command == "build_webapp":
        build_webapp()
    else:
        print(f"Invalid command: {args.command}")
        exit(1)
