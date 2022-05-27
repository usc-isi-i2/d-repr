from pathlib import Path

from api.cmd import create_user
from api.models.dataset import Dataset
from api.models.user import User
from api.models.repr.representation import Representation
from api.models.resource import Resource
from drepr.models import Variable, ResourceDeSer, VariableType, VariableSorted, VariableValue
from drepr.models.parser_v1 import LocationReadableDeSerV1


def setup():
    try:
        create_user("tester", "tester123")
    except AssertionError:
        pass

    user = User.get(User.email == "tester")
    Dataset.delete().where(Dataset.owner == user).execute()

    dataset = Dataset.create(owner=user, name="draft", representation=Representation.default().serialize())
    wrepr = dataset.get_repr()

    data_dir = Path(__file__).parent / "resources"
    for i, fname in enumerate(["s101_time_row.csv", "s201_multiple_sources.csv"]):
        with open(data_dir / fname, "rb") as f:
            resource = Resource.create(
                dataset=dataset,
                resource_id=fname.split(".")[0],
                resource=ResourceDeSer.unsafe_deserialize({
                    'id': fname.split(".")[0],
                    'type': fname.split(".")[1],
                }).serialize(),
                value=f.read())
            assert resource.is_valid_data()

        resource.save()
        wrepr.add_resource(resource)

    loc = LocationReadableDeSerV1.unsafe_deserialize({
        "slices": ["1..", "0"],
        "resource_id": "s101_time_row"
    }).to_loc()
    variable = Variable("year", VariableValue.Literal, loc, VariableSorted.Ascending, True, set([]),
                        VariableType.Unspecified)
    wrepr.upsert_variable(variable)

    loc = LocationReadableDeSerV1.unsafe_deserialize(({
        "slices": ["0", "1.."],
        "resource_id": "s101_time_row"
    })).to_loc()
    variable = Variable("commodity", VariableValue.Literal, loc, VariableSorted.Null, True, set([]),
                        VariableType.Unspecified)
    wrepr.upsert_variable(variable)

    loc = LocationReadableDeSerV1.unsafe_deserialize(({
        "slices": ["1..", "1.."],
        "resource_id": "s101_time_row"
    })).to_loc()
    variable = Variable("prices", VariableValue.Literal, loc, VariableSorted.Null, False, set([]),
                        VariableType.Unspecified)
    wrepr.upsert_variable(variable)

    dataset.set_repr(wrepr)
    dataset.save()


if __name__ == '__main__':
    setup()
