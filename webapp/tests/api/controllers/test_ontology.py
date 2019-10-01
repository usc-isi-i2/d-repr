import io

import pytest


"""These tests will failed because of doesn't change HOME_DIR"""


@pytest.mark.skip
def test_create_ontology(client, authorization, resource_dir):
    for fpath in (resource_dir / "ontologies").iterdir():
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
                    'schema': 'http://schema.org/'
                }[fpath.stem],
                'ontology_file': ont_file
            },
            headers=authorization)
        assert resp.status_code == 200


@pytest.mark.skip
def test_create_duplicated_ontology(client, authorization, resource_dir):
    for fpath in (resource_dir / "ontologies").iterdir():
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
                    'schema': 'http://schema.org/'
                }[fpath.stem],
                'ontology_file': ont_file
            },
            headers=authorization)
        assert resp.status_code == 200

    for fpath in (resource_dir / "ontologies").iterdir():
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
                    'schema': 'http://schema.org/'
                }[fpath.stem],
                'ontology_file': ont_file
            },
            headers=authorization)
        assert resp.status_code == 409
