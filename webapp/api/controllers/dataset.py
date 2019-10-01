import shutil
import uuid
from io import BytesIO
from pathlib import Path

from flask import jsonify, Blueprint, request, send_file, current_app

from api.config import EXEC_TIME_OUT
from api.misc.auth import auth, get_auth_token, get_current_user
from api.misc.exception import BadRequest
from api.misc.form import Form, Validator, Normalizer
from api.models import *
from api.models.event import *
from api.services.ontology_service import OntologyService
from drepr import DRepr, execute as exc_repr, StringOutput, models, OutputFormat
from api.services.queue import QueueProducer

dataset_bp = Blueprint('dataset_bp', __name__)
create_dataset_form = Form({"name": Validator.is_not_empty(), "description": Validator.is_string})
create_loc_form = Form({
    "slices": Validator.always_valid,
    "resource_id": Validator.is_not_empty(),
})
upsert_variable_form = Form(
    {
        "prev_id": Validator.is_not_empty(nullable=True),
        "id": Validator.is_not_empty(),
        "value_type": Validator.is_in_list([x.value for x in models.ValueType]),
        "sorted": Validator.is_in_list([x.value for x in models.Sorted]),
        "unique": Validator.is_boolean(),
        "location": Validator.is_valid_form(create_loc_form),
        "missing_values": Validator.always_valid,
    }, {
        "location": Normalizer.form(create_loc_form),
    })


@dataset_bp.route("/datasets", methods=["GET"])
@auth
def list_dataset():
    user = get_current_user()
    return jsonify({
        "datasets": [{
            "name": d.name,
            "description": d.description
        } for d in Dataset.select().where(Dataset.owner == user)]
    })


@dataset_bp.route("/datasets/<dataset_name>", methods=["HEAD"])
@auth
def has_dataset(dataset_name: str):
    user = get_current_user()
    if Dataset.select().where(Dataset.owner == user, Dataset.name == dataset_name).exists():
        return "", 200
    return "", 404


@dataset_bp.route("/datasets/<dataset_name>", methods=["GET"])
@auth
def get_dataset(dataset_name: str):
    user = get_current_user()
    dataset = Dataset.get(Dataset.owner == user, Dataset.name == dataset_name)

    ds_json = {
        "resources": {},
    }
    for k in ["preprocessing", "alignments", "variables", "semantic_model"]:
        ds_json[k] = dataset.representation[k]

    for rid, resource in dataset.representation['resources'].items():
        ds_json['resources'][rid] = {
            'dimension': dataset.representation['ext_resources'][rid]['dimension']
        }
        ds_json['resources'][rid].update(resource)

    return jsonify(ds_json)


@dataset_bp.route("/datasets/<dataset_name>", methods=["DELETE"])
@auth
def del_dataset(dataset_name: str):
    user = get_current_user()
    dataset = Dataset.get(Dataset.owner == user, Dataset.name == dataset_name)
    dataset.delete_instance()
    return jsonify({"status": "success"})


@dataset_bp.route("/datasets", methods=["POST"])
@auth
def create_dataset():
    user = get_current_user()
    submission = create_dataset_form.post_json_form()

    if Dataset.select().where(Dataset.owner == user, Dataset.name == submission['name']).exists():
        return jsonify({"status": "error", "message": "duplicated dataset"}), 409

    dataset = Dataset.create(owner=user.id,
                             name=submission['name'],
                             description=submission['description'],
                             representation=Representation.default().serialize())

    QueueProducer.get_instance().publish(
        DatasetCreateEvent(get_auth_token(), dataset.name, EXEC_TIME_OUT).serialize2str())

    return jsonify({"status": "success", "dataset": dataset.name})


@dataset_bp.route("/datasets/<dataset_name>/variables", methods=['POST'])
@auth
def upsert_variable(dataset_name: str):
    user = get_current_user()
    form = upsert_variable_form.post_json_form()

    with db.atomic() as transaction:
        try:
            dataset = Dataset.get(Dataset.owner == user, Dataset.name == dataset_name)
            repr: Representation = dataset.get_repr()

            prev_id = form.pop('prev_id')
            if prev_id is not None:
                if not repr.has_variable(prev_id):
                    return jsonify({
                        "status": "error",
                        "message": f"The variable `{prev_id}` does not exist"
                    })
                repr.replace_variable(prev_id, Variable.deserialize(form))
            else:
                repr.add_variable(Variable.deserialize(form))

            dataset.set_repr(repr)
            dataset.save()
        except Exception:
            transaction.rollback()
            raise

    QueueProducer.get_instance().publish(
        VariableUpsertEvent(get_auth_token(), dataset_name, form['id'],
                            EXEC_TIME_OUT).serialize2str())
    return jsonify({"status": "success", "variable_id": form['id']})


@dataset_bp.route("/datasets/<dataset_name>/variables/<variable_id>", methods=['DELETE'])
@auth
def del_variable(dataset_name: str, variable_id: str):
    user = get_current_user()

    with db.atomic() as txn:
        dataset = Dataset.get(Dataset.owner == user, Dataset.name == dataset_name)
        try:
            wrepr = dataset.get_repr()
            wrepr.remove_variable(variable_id)
            dataset.set_repr(wrepr)
            dataset.save()
        except Exception:
            txn.rollback()
            raise

    QueueProducer.get_instance().publish(
        VariableDeleteEvent(get_auth_token(), dataset_name, variable_id,
                            EXEC_TIME_OUT).serialize2str())
    return jsonify({"status": "success"})


@dataset_bp.route("/datasets/<dataset_name>/alignments", methods=["POST"])
@auth
def update_alignments(dataset_name: str):
    user = get_current_user()
    try:
        alignments = ujson.loads(request.data)
        alignments = Representation.class_properties['alignments'].deserialize(alignments)
    except Exception:
        raise BadRequest("Invalid alignments")

    with db.atomic() as txn:
        dataset = Dataset.get(Dataset.owner == user, Dataset.name == dataset_name)
        try:
            wrepr = dataset.get_repr()
            wrepr.set_alignments(alignments)
            dataset.set_repr(wrepr)
            dataset.save()
        except Exception:
            txn.rollback()
            raise

    QueueProducer.get_instance().publish(
        AlignmentUpdateEvent(get_auth_token(), dataset_name, EXEC_TIME_OUT).serialize2str())
    return jsonify({"status": "success"})


@dataset_bp.route("/datasets/<dataset_name>/semantic_model", methods=["POST"])
@auth
def update_semantic_model(dataset_name: str):
    user = get_current_user()
    try:
        sm = ujson.loads(request.data)
        sm = SemanticModel.deserialize(sm)
    except Exception:
        raise BadRequest("Invalid semantic model")

    with db.atomic() as txn:
        dataset = Dataset.get(Dataset.owner == user, Dataset.name == dataset_name)
        try:
            wrepr = dataset.get_repr()
            wrepr.set_semantic_model(sm)
            dataset.set_repr(wrepr)
            dataset.save()
        except Exception:
            txn.rollback()
            raise

    QueueProducer.get_instance().publish(
        SemanticModelUpdateEvent(get_auth_token(), dataset_name, EXEC_TIME_OUT).serialize2str())
    return jsonify({"status": "success"})


@dataset_bp.route("/datasets/<dataset_name>/repr", methods=["POST"])
@auth
def upload_repr(dataset_name: str):
    user = get_current_user()
    dataset = Dataset.get(Dataset.owner == user, Dataset.name == dataset_name)

    if not ('repr_file' in request.files and request.files['repr_file'].filename != ''):
        raise BadRequest("Missing repr file")

    wrepr: Representation = dataset.get_repr()

    # read the repr that users create
    content = request.files['repr_file'].read()
    content = content.decode()
    ds_model = DRepr.parse(models.yaml.load(content))

    missing_resources = {r.id for r in ds_model.resources}.difference(wrepr.resources.keys())
    if len(missing_resources) > 0:
        return jsonify({
            "status": "error",
            "message": f"Cannot upload Repr file because missing data of these resources: {missing_resources}"
        }), 400

    # TODO: fix me, cannonical form doesn't have clone method
    wrepr = Representation.from_repr(ds_model, wrepr.ext_resources)
    dataset.set_repr(wrepr)
    dataset.save()

    return jsonify({"status": "success"})


@dataset_bp.route("/datasets/<dataset_name>/repr", methods=["GET"])
@auth
def download_repr(dataset_name: str):
    user = get_current_user()
    dataset = Dataset.get(Dataset.owner == user, Dataset.name == dataset_name)

    ds_model: DRepr = Representation.to_repr(dataset.representation)
    try:
        ds_model.is_valid()
    except Exception as e:
        current_app.logger.exception(e)
        return jsonify({
            "status": "error",
            "message": "Cannot download repr because it is invalid"
        }), 409

    tempfile = BytesIO()
    tempfile.write(ds_model.to_lang_yml(simplify=True).encode())
    tempfile.seek(0)

    return send_file(tempfile,
                     as_attachment=True,
                     attachment_filename="model.yml",
                     mimetype="application/x-yaml")


@dataset_bp.route("/datasets/<dataset_name>/data", methods=['GET'])
@auth
def download_data(dataset_name: str):
    user = get_current_user()
    dataset = Dataset.get(Dataset.owner == user, Dataset.name == dataset_name)

    ds_model: DRepr = Representation.to_repr(dataset.representation)
    try:
        ds_model.is_valid()
    except Exception as e:
        current_app.logger.exception(e)
        return jsonify({
            "status": "error",
            "message": "Cannot download repr because it is invalid"
        }), 409

    wdir = Path("/tmp/" + str(uuid.uuid4()).replace("-", ""))
    wdir.mkdir(parents=True)

    try:
        resources = {}
        for resource in Resource.select().where(Resource.dataset == dataset):
            filename = str(wdir / f"{resource.resource_id}.dat")
            resources[resource.resource_id] = filename

            with open(filename, "wb") as f:
                f.write(resource.value)

        result = exc_repr(ds_model, resources, StringOutput(OutputFormat.TTL))
        result = result["value"]
    finally:
        shutil.rmtree(str(wdir))

    tempfile = BytesIO()
    tempfile.write(result.encode())
    tempfile.seek(0)

    return send_file(tempfile,
                     as_attachment=True,
                     attachment_filename="data.ttl",
                     mimetype="application/x-yaml")
