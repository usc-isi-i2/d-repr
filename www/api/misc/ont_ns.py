from itertools import chain
from typing import *


class Namespace(str):

    def __new__(cls, *args, **kwargs):
        return str.__new__(cls, *args, **kwargs)

    def __getattr__(self, item: str) -> str:
        return self + item

    def __getitem__(self, item: str) -> str:
        return self + item


class OntNS:
    prefixes = {
        "rdfs": "http://www.w3.org/2000/01/rdf-schema#",
        "rdf": "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
        "owl": "http://www.w3.org/2002/07/owl#"
    }

    def __init__(self, prefixes: Dict[str, str]):
        self.prefixes: Dict[str, Namespace] = {
            x: Namespace(y)
            for x, y in chain(prefixes.items(), OntNS.prefixes.items())
        }

    def full_uri(self, simplified_uri: str) -> str:
        if simplified_uri.startswith('http://') or simplified_uri.startswith('https://'):
            return simplified_uri

        if simplified_uri.find(":") != -1:
            prefix, lbl = simplified_uri.split(":", maxsplit=1)
            return self.prefixes[prefix][lbl]
        return simplified_uri

    def simplify_uri(self, uri: str) -> str:
        for ns, prefix in self.prefixes.items():
            if uri.startswith(prefix):
                return f"{ns}:{uri.replace(prefix, '')}"
        raise Exception(f"Cannot simplify the uri: {uri}")

    def __getattr__(self, item: str) -> Namespace:
        return self.prefixes[item]


class KarmaOnt:

    ns = "https://isi.edu/karma#"
    uri = ns + "uri"