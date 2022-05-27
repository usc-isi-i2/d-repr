from typing import Dict, TYPE_CHECKING, Optional

import networkx as nx

from drepr.mfunc.index_mapping_func import MappingFunc, IndexMappingFunc, IdenticalMappingFunc
from drepr.models.mapping import Mapping, DimensionMapping, IdenticalMapping

if TYPE_CHECKING:
    from drepr.services.ra_reader.ra_reader import RAReader
    from drepr.models.representation import Representation


class MappingGraph:
    def __init__(self, repr: 'Representation'):
        self.repr = repr
        self.mfuncs: Dict[str, Dict[str, Mapping]] = {
            v.id: {u.id: None
                   for u in repr.get_variables()}
            for v in repr.get_variables()
        }
        mg = nx.DiGraph()

        # create initial graph
        for v in repr.get_variables():
            mg.add_node(v.id)
            self.mfuncs[v.id][v.id] = IdenticalMapping(v.id)

        for m in repr.mappings:
            self.mfuncs[m.source_var][m.target_var] = m
            self.mfuncs[m.target_var][m.source_var] = m.swap()
            mg.add_edge(m.source_var, m.target_var, mfunc=m)
            mg.add_edge(m.target_var, m.source_var, mfunc=m.swap())

        # infer more mapping functions, trying to build
        # a complete graph using DFS
        for u in repr.get_variables():
            # update the mapping function from node u to remained node in the graph
            for e in nx.dfs_edges(mg, source=u.id):
                if not mg.has_edge(u.id, e[1]) or mg.edges[u.id, e[1]]['mfunc'] is None:
                    # there is no mapping function from source to e.target
                    if self.mfuncs[u.id][e[1]] is not None:
                        raise NotImplementedError()
                    else:
                        mfunc = self.infer_mfunc(u.id, e[0], e[1])
                        if mfunc is not None:
                            self.mfuncs[u.id][e[1]] = mfunc
                            self.mfuncs[e[1]][u.id] = mfunc.swap()

            for v in repr.get_variables():
                if self.mfuncs[u.id][v.id] is not None and not mg.has_edge(u.id, v.id):
                    mg.add_edge(u.id, v.id, mfunc=self.mfuncs[u.id][v.id])
                    mg.add_edge(v.id, u.id, mfunc=self.mfuncs[v.id][u.id])

    def add_new_variable(self, var: str):
        self.mfuncs[var] = {xid: None for xid in self.mfuncs}
        self.mfuncs[var][var] = None
        for xid in self.mfuncs:
            self.mfuncs[xid][var] = None

    def set_mapping_func(self, func: Mapping, xid: str, yid: str):
        self.mfuncs[xid][yid] = func
        self.mfuncs[yid][xid] = func.swap()

    def get_mapping_func(self, xid: str, yid: str) -> Optional[Mapping]:
        """Get a mapping function between two variables x and y"""
        return self.mfuncs[xid][yid]

    def has_mapping_func(self, xid: str, yid: str) -> bool:
        """Check if there is a mapping function between two variables x and y"""
        return self.mfuncs[xid][yid] is not None

    def get_mapping_exec_func(self, ra_reader: 'RAReader', xid: str, yid: str) -> MappingFunc:
        """Get an executable mapping function that take a data value of x and return a corresponding value of y"""
        f = self.mfuncs[xid][yid]
        if isinstance(f, DimensionMapping):
            return IndexMappingFunc(
                ra_reader, f,
                ra_reader.get_grounded_location(self.repr.get_variable(xid).location),
                ra_reader.get_grounded_location(self.repr.get_variable(yid).location))
        elif isinstance(f, IdenticalMapping):
            return IdenticalMappingFunc()
        else:
            raise NotImplementedError()

    def infer_mfunc(self, xid: str, yid: str, zid: str) -> Optional[Mapping]:
        """Infer a mapping function: h: x->z given f: x->y, g: y->z"""
        x = self.repr.get_variable(xid)
        y = self.repr.get_variable(yid)
        z = self.repr.get_variable(zid)

        f = self.mfuncs[xid][yid]
        g = self.mfuncs[yid][zid]
        if isinstance(f, DimensionMapping) and isinstance(g, DimensionMapping) \
                and f.is_single_value_func(self.repr) and g.is_single_value_func(self.repr) \
                and f.is_surjective() and g.is_surjective():
            # auto-infer for case f, g single-value surjective functions
            y2x = dict(zip(f.target_dims, f.source_dims))
            z2y = dict(zip(g.target_dims, g.source_dims))

            x2z = ([], [])
            for zdim, ydim in z2y.items():
                x2z[0].append(y2x[ydim])
                x2z[1].append(zdim)
            return DimensionMapping(xid, x2z[0], zid, x2z[1])

        return None
