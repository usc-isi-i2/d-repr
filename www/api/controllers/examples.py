import os
from tempfile import NamedTemporaryFile

import ujson
from flask import Blueprint, send_from_directory, render_template, request, abort
from werkzeug.exceptions import NotFound

from api.config import HOME_DIR
from drepr.data_source importDataSource
from drepr.models import Representation, InvalidReprException
from drepr.query.return_format import ReturnFormat
from drepr.query.selection_interface import SelectionGraph

examples_bp = Blueprint('examples_bp', __name__)


@examples_bp.route("/demo", methods=["GET", "POST"])
def demo():
    if request.method == 'GET':
        return render_template('demo.html')

    if request.method != 'POST':
        return "Invalid request", 404

    try:
        drepr = ujson.loads(request.form.get('drepr', ''))
    except ValueError as e:
        return "Invalid json", 400

    try:
        drepr = Representation.parse(drepr)
    except InvalidReprException as e:
        return str(e), 400

    if 'resources[]' not in request.files:
        return "Missing resources", 400

    resource_files = {
        r.filename[:r.filename.find(".")]: r
        for r in request.files.getlist('resources[]')
    }

    resource_tmp_files = {rid: NamedTemporaryFile(delete=True) for rid in resource_files.keys()}

    try:
        for rid, file in resource_files.items():
            resource_tmp_files[rid].write(file.read())
            resource_tmp_files[rid].flush()

        resource_data = {rid: tmpfile.name for rid, tmpfile in resource_tmp_files.items()}
        if len(resource_data) == 1 and len(drepr.resources) == 1:
            # remap the resource id
            resource_data = {next(iter(drepr.resources.keys())): next(iter(resource_data.values()))}
        data_source = DataSource(drepr, resource_data)

        try:
            result = data_source.select(SelectionGraph.from_sm(drepr.semantic_model)).exec(
                ReturnFormat.JsonLD)
        except AssertionError as e:
            return abort(400, "AssertionError: " + str(e))
        return ujson.dumps(result, indent=4)
    finally:
        for rid, tmpfile in resource_tmp_files.items():
            tmpfile.close()


@examples_bp.route("/examples/")
@examples_bp.route("/examples/<path:path>")
def serve_examples(path=""):
    try:
        return send_from_directory(HOME_DIR / "examples", path)
    except NotFound as e:
        dpath = os.path.abspath(str(HOME_DIR / "examples" / path))
        basedir = os.path.abspath(str(HOME_DIR / "examples"))
        if not dpath.startswith(basedir) or not os.path.isdir(dpath):
            raise

        relpath = dpath.replace(basedir, "")

        files = [{'href': f'/examples{os.path.dirname(relpath)}', 'name': '..'}]

        for fname in os.listdir(dpath):
            files.append({'href': f'/examples{relpath}/{fname}', 'name': fname})

        return render_template("file_explorer.html", files=files)
