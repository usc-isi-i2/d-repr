import os

import rdflib
from flask import jsonify, Blueprint, request, abort

from api.misc.auth import auth
from api.misc.exception import BadRequest
from api.misc.form import Form, Validator
from api.services.ontology_service import OntologyService

ont_bp = Blueprint('ont_bp', __name__)
create_ontology_form = Form({
    "namespace": Validator.is_not_empty(),
    "prefix": Validator.is_not_empty(),
})


@ont_bp.route("/ontologies", methods=["GET"])
@auth
def get_ontologies():
    return jsonify({"ontologies": OntologyService.get_instance().prefixes})


@ont_bp.route("/ontologies/search", methods=['GET'])
def search_ontologies():
    ont = OntologyService.get_instance()

    resource_type = request.args.get('a', None)
    query = request.args.get('q')
    if type(query) is not str:
        return abort(400)

    if resource_type == 'class':
        results = ont.search_ont_class(query)
    elif resource_type == 'property':
        results = ont.search_ont_predicate(query)
    else:
        return abort(400)

    return jsonify({"results": [x['shortURI'] for x in results]})


@ont_bp.route("/ontologies/<ns>", methods=["DELETE"])
@auth
def delete_ontology(ns: str):
    return jsonify({"status": "error", "message": "Not implemented"}), 501


@ont_bp.route("/ontologies", methods=["POST"])
@auth
def create_ontology():
    form = create_ontology_form.post_form()
    ont_service = OntologyService.get_instance()

    if not ('ontology_file' in request.files
            and request.files['ontology_file'].filename != ''):
        raise BadRequest("Missing ontology file")

    ext = os.path.splitext(request.files['ontology_file'].filename)[1][1:]
    if ext == "ttl":
        format = "ttl"
    else:
        raise BadRequest(f"Does not support ontology with extension {ext} yet. Possible options: .ttl")

    if form['namespace'] in ont_service.namespaces:
        return jsonify({"status": "fail", "message": "The ontology already exists"}), 409

    if form['prefix'] in ont_service.prefixes:
        return jsonify({"status": "fail", "message": "Prefix is already exists"}), 409

    raw_ont_str = request.files['ontology_file'].read().decode()

    try:
        n_classes, n_predicates = ont_service.add_ontology(raw_ont_str, form['namespace'],
                                 form['prefix'], format)
    except rdflib.plugins.parsers.notation3.BadSyntax:
        return jsonify({"status": "error", "message": "Invalid notion3 syntax"}), 400

    return jsonify({"status": "success", "n_classes": n_classes, "n_predicates": n_predicates})
