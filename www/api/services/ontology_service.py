import ujson
from pathlib import Path
from typing import List

import rdflib
from rdflib.namespace import RDF, RDFS, OWL
from whoosh.fields import Schema, TEXT, ID, analysis
from whoosh.index import create_in, open_dir
from whoosh.qparser import QueryParser

from api.config import HOME_DIR


class OntologyService:
    """TODO: doesn't support multi-users yet"""
    instance = None

    def __init__(self):
        self.schema = Schema(uri=ID(stored=True), label=TEXT(
            analyzer=analysis.NgramWordAnalyzer(minsize=2, maxsize=10), stored=True, spelling=True))
        self.ont_classes = {}
        self.ont_predicates = {}
        self.prefixes = {}
        self.namespaces = {}

        self.index_dir: Path = (HOME_DIR / "index_dir").absolute()
        if not self.index_dir.exists():
            self.index_dir.mkdir()
            (self.index_dir / "ont_class").mkdir()
            (self.index_dir / "ont_predicate").mkdir()
            self.ix_ont_class = create_in(
                str(self.index_dir / "ont_class"), self.schema)
            self.ix_ont_predicate = create_in(
                str(self.index_dir / "ont_predicate"), self.schema)
        else:
            self.ix_ont_class = open_dir(str(self.index_dir / "ont_class"))
            self.ix_ont_predicate = open_dir(
                str(self.index_dir / "ont_predicate"))

        if (self.index_dir / "ontologies.json").exists():
            with open(str(self.index_dir / "ontologies.json"), "r") as f:
                o = ujson.load(f)
                self.ont_classes = o['ont_classes']
                self.ont_predicates = o['ont_predicates']
                self.prefixes = o['prefixes']
                self.namespaces = o['namespaces']

    @staticmethod
    def get_instance():
        if OntologyService.instance is None:
            OntologyService.instance = OntologyService()
        return OntologyService.instance

    def has_ontology(self, namespace: str):
        return namespace in self.namespaces

    def add_ontology(self, raw_ont_str: str, namespace: str, prefix: str, format: str):
        g = rdflib.Graph()
        g.parse(data=raw_ont_str, format=format)

        cprefix = prefix + ":"
        new_classes, new_predicates = [], []

        for u in set(g.subjects(RDF.type, RDFS.Class)).union(
                set(g.subjects(RDF.type, OWL.Class))):
            u = str(u)
            if u.startswith(namespace) and u not in self.ont_classes:
                self.ont_classes[u] = {
                    "URI": u,
                    "namespace": namespace,
                    "prefix": prefix,
                    "shortURI": u.replace(namespace, cprefix)
                }
                new_classes.append(u)

        for u in set(g.subjects(RDF.type, RDF.Property)):
            u = str(u)
            if u.startswith(namespace) and u not in self.ont_predicates:
                self.ont_predicates[u] = {
                    "URI": u,
                    "namespace": namespace,
                    "prefix": prefix,
                    "shortURI": u.replace(namespace, cprefix)
                }
                new_predicates.append(u)

        if len(new_classes) + len(new_predicates) > 0:
            self.prefixes[prefix] = namespace
            self.namespaces[namespace] = prefix

            if not (self.index_dir / f"{prefix}.log").exists():
                # index the ontology
                writer = self.ix_ont_class.writer()
                for u in new_classes:
                    writer.add_document(
                        uri=u, label=self.ont_classes[u]['shortURI'])
                writer.commit()
                writer = self.ix_ont_predicate.writer()
                for u in new_predicates:
                    writer.add_document(
                        uri=u, label=self.ont_predicates[u]['shortURI'])
                writer.commit()
                open(str(self.index_dir / f"{prefix}.log"), 'a').close()
                with open(str(self.index_dir / "ontologies.json"), "w") as f:
                    ujson.dump({
                        "prefixes": self.prefixes,
                        "namespaces": self.namespaces,
                        "ont_classes": self.ont_classes,
                        "ont_predicates": self.ont_predicates
                    }, f)

        return len(new_classes), len(new_predicates)

    def search_ont_class(self, query: str) -> List[dict]:
        results = []
        with self.ix_ont_class.searcher() as searcher:
            parser = QueryParser("label", self.schema)
            for res in searcher.search(parser.parse(query)):
                results.append(self.ont_classes[res['uri']])
        return results

    def search_ont_predicate(self, query: str) -> List[dict]:
        results = []
        with self.ix_ont_predicate.searcher() as searcher:
            parser = QueryParser("label", self.schema)
            for res in searcher.search(parser.parse(query)):
                results.append(self.ont_predicates[res['uri']])
        return results
