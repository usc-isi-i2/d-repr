from enum import Enum
from typing import List, Dict, Tuple, Callable, Any, Optional


class ReturnFormat(Enum):
    Turtle = ("turtle", False)

    JsonLD = ("jsonld", False)
    JsonLD_FullURI = ("jsonld", True)

    TripleGraph = ("triple_graph", False)
    TripleGraph_FullURI = ("triple_graph", True)

    def __init__(self, format: str, use_full_uri: bool):
        self.format = format
        self.use_full_uri = use_full_uri
