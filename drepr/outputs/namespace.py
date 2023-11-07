from typing import List, Dict, Tuple, Callable, Any, Optional


class Namespace:
    RDF = "http://www.w3.org/1999/02/22-rdf-syntax-ns#"

    def __init__(self, base: str, seen_uris: Optional[Dict[str, str]] = None):
        self._base = base
        self.seen_uris = seen_uris or {}

    def __getattr__(self, item):
        if item not in self.seen_uris:
            uri = self._base + item
            self.seen_uris[item] = uri
        return self.seen_uris[item]

    def __getitem__(self, item):
        if item not in self.seen_uris:
            uri = self._base + item
            self.seen_uris[item] = uri
        return self.seen_uris[item]


class PrefixedNamespace(Namespace):

    def __init__(self, base: str, prefix: str, seen_uris: Optional[Dict[str, str]] = None):
        super().__init__(base, seen_uris)
        self._prefix = prefix + ":"

    def __getattr__(self, item):
        if item not in self.seen_uris:
            uri = self._prefix + item
            self.seen_uris[item] = uri
        return self.seen_uris[item]

    def __getitem__(self, item):
        if item not in self.seen_uris:
            uri = self._prefix + item
            self.seen_uris[item] = uri
        return self.seen_uris[item]
