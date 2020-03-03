from pathlib import Path
from typing import Union

from rdflib import Graph, BNode, URIRef
from pydrepr import Repr, execute, Graph as DGraph


def test_examples(example_dir: Path):
    models = [file for file in example_dir.iterdir() if file.name.endswith("model.yml")]
    for model in models:
        dsid = model.stem.split(".")[0]
        model_output = model.parent / f"{dsid}.model.out"
        resources = {}

        for file in example_dir.iterdir():
            if file.stem.startswith(dsid) and not file.stem.startswith(f"{dsid}.model"):
                resources[file.stem[len(dsid) + 1:]] = str(file)

        assert len(DGraph.from_drepr(Repr.from_file(str(model)), resources).nodes) > 0
        pred_result = execute(Repr.from_file(str(model)).normalize_mut(), resources, "ttl")
        with open(model_output, "r") as f:
            gold_result = f.read()

        # uncomment the code below to sync the new result
        # with open(model_output, "w") as f:
        #     f.write(pred_result)

        assert_equal_two_graphs(gold_result, pred_result)


def assert_equal_two_graphs(gold: str, pred: str):
    def collect_object(g: Graph):
        records = {}
        for s in g.subjects():
            record = {}
            for p, o in g.predicate_objects(s):
                if p in record:
                    if isinstance(record[p], list):
                        record[p].append(o)
                    else:
                        record[p] = [record[p], o]
                record[p] = o
            records[s] = record
        return records

    gold_graph = Graph()
    gold_graph.parse(data=gold, format="ttl")

    pred_graph = Graph()
    pred_graph.parse(data=pred, format="ttl")

    gold_nodes = collect_object(gold_graph)
    pred_nodes = collect_object(pred_graph)

    # check non-blank nodes first
    subjs = list(pred_nodes.keys())
    o2o_blank_nodes = {}

    for subj in subjs:
        if isinstance(subj, URIRef):
            assert subj in gold_nodes
            pred_node = pred_nodes.pop(subj)
            gold_node = gold_nodes.pop(subj)

            for k, v in pred_node.items():
                if isinstance(v, BNode):
                    assert isinstance(gold_node[k], BNode)
                    o2o_blank_nodes[v] = gold_node[k]
                else:
                    assert v == gold_node[k]

    subjs = list(gold_nodes.keys())
    for subj in subjs:
        if isinstance(subj, URIRef):
            assert subj in pred_nodes
            assert pred_nodes.pop(subj) == gold_nodes.pop(subj)

    # check blank nodes here
    subjs = list(pred_nodes.keys())
    gold_subjs = list(gold_nodes.keys())

    for subj in subjs:
        pnode = pred_nodes.pop(subj)
        for i in range(len(gold_subjs)):
            if gold_nodes[gold_subjs[i]] == pnode:
                if subj in o2o_blank_nodes:
                    assert o2o_blank_nodes.pop(subj) == gold_subjs[i], 'they must be matched'
                gold_nodes.pop(gold_subjs[i])
                gold_subjs.pop(i)
                break
        else:
            raise Exception(f"Cannot find blank node {pnode} in the gold graph")

    assert len(o2o_blank_nodes) == 0
    assert len(gold_nodes) == 0 and len(pred_nodes) == 0, "Expect that we tested all nodes"