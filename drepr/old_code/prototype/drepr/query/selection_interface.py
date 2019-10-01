import uuid
from collections import defaultdict
from typing import *
from typing import Optional

import networkx as nx

from drepr.misc.ont_ns import OntNS
from drepr.models import ClassID, SemanticModel
from drepr.services.ra_iterator import RAIterator
from drepr.services.ra_reader.ra_reader import RAReader


class SelectionGraph:
    """Need to specify where, optional match and non-optional match"""
    DataNodeLBL = "data_node"

    def __init__(self, ont: OntNS, main_node: ClassID):
        main_node.set_uri(ont.full_uri(main_node.uri))
        self.ont = ont
        self.main_node = main_node
        self.graph = nx.DiGraph()
        self.graph.add_node(str(main_node), label=main_node.uri, original=main_node, is_class_node=True)

    @staticmethod
    def from_sm(sm: SemanticModel) -> 'SelectionGraph':
        # TODO: fix me, inefficient
        roots = [n for n in sm.iter_class_nodes() if sm.graph.in_degree(n) == 0]
        assert len(roots) == 1, "The semantic model used to construct query must have exactly one root"
        sg = SelectionGraph(sm.ont, ClassID.create(roots[0]))
        for e in sm.graph.edges:
            if sm.graph.nodes[e[1]]['is_class_node']:
                sg.add_edge(ClassID.create(e[0]), ClassID.create(e[1]), sm.graph.edges[e]['label'])
            else:
                sg.add_edge(ClassID.create(e[0]), None, sm.graph.edges[e]['label'])

        return sg

    def add_edge(self, source: ClassID, target: Optional[ClassID], predicate: str, is_optional: bool=False):
        """expand the selection graph to select a data property or a object property"""
        source.set_uri(self.ont.full_uri(source.uri))
        predicate = self.ont.full_uri(predicate)

        if target is not None:
            target.set_uri(self.ont.full_uri(target.uri))
            self.graph.add_node(str(target), label=target.uri, original=target, is_class_node=True)
        else:
            target = str(uuid.uuid4())
            self.graph.add_node(str(target), label=self.DataNodeLBL, is_class_node=False)

        self.graph.add_node(str(source), label=source.uri, original=source, is_class_node=True)
        self.graph.add_edge(str(source), str(target), label=predicate, is_optional=is_optional)

    def add_filter(self):
        """constraint the result"""
        pass

    def optimize(self):
        """Optimize the selection graph by removing redundant edges"""
        removing_edges = []
        for u, v in self.graph.edges:
            if self.graph.nodes[v]['is_class_node'] and self.graph.out_degree(v) == 0:
                removing_edges.append((u, v))

        for u, v in removing_edges:
            self.graph.remove_edge(u, v)

    def align_with_semantic_model(self, sm: SemanticModel):
        """align the selection graph with semantic models"""
        self.optimize()

        mappings = align2graphs(sm.graph, self.graph)
        assert len(mappings) == 1, "Haven't handle when there are more than 1 mapping"
        mapping = mappings[0]
        assert get_mapping_score(sm.graph, self.graph, mapping) == 1

        self.graph = nx.relabel_nodes(self.graph, mapping)
        for n in self.graph.nodes:
            if not self.graph.nodes[n]['is_class_node']:
                self.graph.nodes[n]['label'] = n

        self.main_node = self.graph.nodes[mapping[str(self.main_node)]]['original']

    def use_short_uri(self):
        for n in self.graph.nodes:
            if self.graph.nodes[n]['is_class_node']:
                self.graph.nodes[n]['label'] = self.ont.simplify_uri(self.graph.nodes[n]['label'])
        for e in self.graph.edges:
            self.graph.edges[e]['label'] = self.ont.simplify_uri(self.graph.edges[e]['label'])

    def to_sm(self, original_sm: SemanticModel) -> SemanticModel:
        stypes = {}
        for var_id, stype in original_sm.semantic_types.items():
            if self.graph.has_node(var_id):
                stypes[var_id] = stype

        rels = []
        for rel in original_sm.semantic_relations:
            if self.graph.has_edge(str(rel.source), str(rel.target)):
                rels.append(rel)

        sm = SemanticModel(stypes, rels, original_sm.ont_prefixes)
        return sm


class MockRAReader(RAReader):

    def __init__(self, choices):
        self.choices = choices

    def _get_shape(self) -> List[int]:
        raise NotImplementedError()

    def get_value(self, idx: List[int], start_idx: int = 0) -> Any:
        return [self.choices[i][x] for i, x in enumerate(idx)]


def align2graphs(g1: nx.DiGraph, g2: nx.DiGraph):
    """Get mapping between node in g2 -> g1"""
    data_node_lbl = '@data_node'

    lbl2ids = defaultdict(lambda: [])
    for n in g1.nodes:
        if g1.nodes[n]['is_class_node']:
            lbl = g1.nodes[n]['label']
        else:
            u, v, edata = next(iter(g1.in_edges(n, data=True)))
            lbl = f"{g1.nodes[u]['label']}--{edata['label']}"
        lbl2ids[lbl].append(n)

    gold_triples = set()
    for s, o in g1.edges:
        lbl = g1.edges[(s, o)]['label']
        gold_triples.add((s, lbl, o))

    pred_lbl2ids = defaultdict(lambda: [])
    for n in g2.nodes:
        if g2.nodes[n]['is_class_node']:
            lbl = g2.nodes[n]['label']
        else:
            u, v, edata = next(iter(g2.in_edges(n, data=True)))
            lbl = f"{g2.nodes[u]['label']}--{edata['label']}"
        pred_lbl2ids[lbl].append(n)

    # partition into which we know the mappings
    mapping = {}
    unsolved_mappings = []
    for lbl, nodes in pred_lbl2ids.items():
        if len(nodes) == 1 and len(lbl2ids[lbl]) == 1:
            mapping[nodes[0]] = lbl2ids[lbl][0]
        elif len(nodes) > 0 and len(lbl2ids[lbl]) > 0:
            unsolved_mappings.append(lbl)
        else:
            assert False, "Invalid selection graph"

    if len(unsolved_mappings) == 0:
        return [mapping]

    # generate possible mappings choices
    choices = []
    for lbl in unsolved_mappings:
        nodes = lbl2ids[lbl]
        pred_nodes = pred_lbl2ids[lbl]
        # don't care if two node map to one node because we assure that the accuracy is 1
        for y in pred_nodes:
            choices.append([(y, x) for x in nodes])

    iterator = RAIterator(MockRAReader(choices), [len(x) for x in choices], [0] * len(choices))
    # select best one
    best_hypo_mappings = []
    best_score = 0
    while True:
        res = iterator.next()
        if res is None:
            break
        hypo_mapping = dict(res[1])

        pred_triples = set()
        for sp, op in g2.edges:
            lbl = g2.edges[(sp, op)]['label']
            if sp in mapping:
                s = mapping[sp]
            else:
                s = hypo_mapping[sp]

            if op in mapping:
                o = mapping[op]
            else:
                o = hypo_mapping[op]
            pred_triples.add((s, lbl, o))

        acc = len(pred_triples.intersection(gold_triples))
        if acc > best_score:
            best_hypo_mappings = [hypo_mapping]
        elif acc == best_score:
            best_hypo_mappings.append(hypo_mapping)

    return [hypo_mapping.update(mapping) for hypo_mapping in best_hypo_mappings]


def get_mapping_score(g1: nx.DiGraph, g2: nx.DiGraph, mapping: Dict[str, str]):
    gold_triples = set()
    for s, o in g1.edges:
        lbl = g1.edges[(s, o)]['label']
        gold_triples.add((s, lbl, o))

    pred_triples = set()
    for s, o in g2.edges:
        lbl = g2.edges[(s, o)]['label']
        pred_triples.add((mapping[s], lbl, mapping[o]))

    return len(pred_triples.intersection(gold_triples)) / len(pred_triples)