from flask import jsonify, Blueprint, request

from api.config import EXEC_TIME_OUT
from api.misc.auth import auth, get_current_user, get_auth_token
from api.misc.exception import BadRequest
from api.misc.form import Form, Validator
from api.models.base import db
from api.models.dataset import Dataset
from api.models.resource import Resource
from api.models.event import *
from api.services.resource_data import ResourceDataService
from api.services.queue import QueueProducer
from drepr import DRepr, models

resource_bp = Blueprint('resource_bp', __name__)
create_resource_form = Form({
    "resource_id": Validator.is_not_empty(),
    "resource_type": Validator.is_in_list([x.value for x in models.ResourceType]),
    "extra": Validator.always_valid
})


@resource_bp.route("/datasets/<dataset_name>/resources", methods=["POST"])
@auth
def create_resource(dataset_name: str):
    user = get_current_user()
    form = create_resource_form.post_form()

    if not ('resource_file' in request.files and request.files['resource_file'].filename != ''):
        raise BadRequest("Missing resource file")

    resource_value = request.files['resource_file'].read()

    # TODO: set isolation level to serializable
    with db.atomic() as transaction:
        dataset = Dataset.get(Dataset.owner == user, Dataset.name == dataset_name)
        resource = {
            'type': form['resource_type'],
            'id': form['resource_id'],
        }

        try:
            resource.update(ujson.loads(form['extra']))
        except ValueError:
            raise BadRequest("Invalid value for `extra` field")

        resource = Resource(
            dataset=dataset,
            resource_id=form['resource_id'],
            resource_type=form['resource_type'],
            resource=resource,
            value=resource_value)
        if not resource.is_valid_data():
            raise BadRequest("Invalid resource data")

        try:
            wrepr = dataset.get_repr()
            if wrepr.has_resource(resource.resource_id):
                raise BadRequest("Duplicate resource")
            else:
                resource.save()
                wrepr.add_resource(resource)
                dataset.set_repr(wrepr)
                dataset.save()
        except Exception:
            transaction.rollback()
            raise

    QueueProducer.get_instance().publish(
        ResourceCreateEvent(get_auth_token(), dataset_name, resource.resource_id,
                            EXEC_TIME_OUT).serialize2str())

    resp = {
        "status": "success",
        "resource": {
            "dimension": wrepr.ext_resources[resource.resource_id].dimension.serialize()
        }
    }
    resp['resource'].update(wrepr.resources[resource.resource_id].serialize())
    return jsonify(resp)


@resource_bp.route("/datasets/<dataset_name>/resources/<resource_id>", methods=["DELETE"])
@auth
def delete_resource(dataset_name: str, resource_id: str):
    user = get_current_user()

    with db.atomic() as txn:
        try:
            dataset = Dataset.get(Dataset.owner == user, Dataset.name == dataset_name)
            wrepr = dataset.get_repr()

            if not wrepr.has_resource(resource_id):
                return jsonify({"status": "fail", "message": "resource doesn't exist"}), 404

            resource_db_id = wrepr.get_resource_db_id(resource_id)
            wrepr.remove_resource(resource_id)
            Resource.delete_by_id(resource_db_id)
            dataset.set_repr(wrepr)
            dataset.save()
        except Exception:
            txn.rollback()
            raise

    QueueProducer.get_instance().publish(
        ResourceDeleteEvent(get_auth_token(), dataset_name, resource_id,
                            EXEC_TIME_OUT).serialize2str())

    return jsonify({"status": "success"})


@resource_bp.route("/datasets/<dataset_name>/resources/<resource_id>", methods=['GET'])
@auth
def get_resource_data(dataset_name: str, resource_id: str):
    try:
        slices = ujson.loads(request.args.get("slices"))
    except Exception:
        raise BadRequest("Invalid slices")

    user = get_current_user()
    dataset = Dataset.get(Dataset.owner == user, Dataset.name == dataset_name)

    wrepr = dataset.get_repr()
    resource = Resource.get(Resource.dataset == dataset,
                            Resource.db_id == wrepr.get_resource_db_id(resource_id))

    # try:
    data, nslice = ResourceDataService.get_instance().get_resource_data(resource, slices)
    # except Exception as e:
    #     raise BadRequest("May be slices are invalid") from e
    return jsonify({"data": data, "slice": nslice})
