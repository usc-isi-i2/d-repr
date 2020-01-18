import copy
from typing import Dict, Tuple

import ujson
from netCDF4 import Dataset

from drepr.models import ResourceType, DRepr, LiteralNode, Preprocessing, PreprocessingType, PMap, DataNode, Attr, Path, \
    IndexExpr, Sorted, Edge, Resource, ClassNode, RangeAlignment
from drepr.patches import ResourceData, ResourceDataFile, ResourceDataString


def patch(repr: DRepr, resources: Dict[str, ResourceData]) -> Tuple[DRepr, Dict[str, ResourceData]]:
    """
    This patch turns classes that have only static properties to have one attribute in order to keep the
    algorithm implemented in Rust simple and fast
    """
    patch_classes = []
    for u in repr.sm.iter_class_nodes():
        if all(isinstance(repr.sm.nodes[e.target_id], LiteralNode) for e in repr.sm.iter_outgoing_edges(u.node_id)):
            patch_classes.append(u)

    if len(patch_classes) > 0:
        repr = copy.deepcopy(repr)
        resources = {k: resources[k] for k in resources}

        resource_id = '__static_patch_resource__'
        assert resource_id not in resources
        resource_data = {}

        existing_attrs = {a.id for a in repr.attrs}
        for u in patch_classes:
            replace_edge = None
            for e in repr.sm.iter_outgoing_edges(u.node_id):
                if e.label == 'drepr:uri':
                    replace_edge = e
                    break
                replace_edge = e
            v = repr.sm.nodes[replace_edge.target_id]

            new_attr_id = f'a87k219da_{u.node_id}_{replace_edge.edge_id}_{v.node_id}'
            assert new_attr_id not in existing_attrs
            resource_data[new_attr_id] = v.value

            # remove literal node
            repr.sm.remove_node(replace_edge.target_id)
            # repr.sm.edges.pop(replace_edge.edge_id)

            # add new attribute and link to it
            repr.attrs.append(Attr(new_attr_id, resource_id, Path([IndexExpr(new_attr_id)]), [], unique=True, sorted=Sorted.Ascending))
            v = DataNode(f'dnode:{new_attr_id}', new_attr_id)
            repr.sm.nodes[v.node_id] = v
            repr.sm.edges[replace_edge.edge_id] = Edge(replace_edge.edge_id, replace_edge.source_id, v.node_id, replace_edge.label, is_subject=True)

            # add alignment with incoming class, add all attributes since we don't know which attribute will be the subject
            for e in repr.sm.iter_incoming_edges(u.node_id):
                if isinstance(repr.sm.nodes[e.source_id], ClassNode):
                    for ie in repr.sm.iter_outgoing_edges(e.source_id):
                        if isinstance(repr.sm.nodes[ie.target_id], DataNode):
                            repr.aligns.append(RangeAlignment(repr.sm.nodes[ie.target_id].attr_id, new_attr_id, []))
            for e in repr.sm.iter_outgoing_edges(u.node_id):
                if isinstance(repr.sm.nodes[e.target_id], ClassNode):
                    for ie in repr.sm.iter_outgoing_edges(e.target_id):
                        if isinstance(repr.sm.nodes[ie.target_id], DataNode):
                            repr.aligns.append(RangeAlignment(new_attr_id, repr.sm.nodes[ie.target_id].attr_id, []))

        repr.resources.append(Resource(resource_id, ResourceType.JSON))
        resources[resource_id] = ResourceDataString(ujson.dumps(resource_data))

    return repr, resources


