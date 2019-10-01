from collections import defaultdict
from typing import List, Dict, Tuple, Callable, Any, Optional

from drepr.models import DRepr


def patch(repr: DRepr, resources: Dict[str, str]) -> DRepr:
    """
    This patch will fix json path that select property names, before we officially support
    selecting property names

    How does it work: it creates a preprocessing function that convert an object to list of key-value pairs
    """
    need_patch = False
    for pfunc in repr.get_preprocess():
        for slice in pfunc['input']['slices']:
            if slice == '*~':
                raise Exception(
                    "Does not support preprocessing that is applied on properties of an object yet!"
                )

    for attr_name, attr in repr.get_attributes().items():
        if sum(int(slice == '*~') for slice in attr['location']['slices']) > 1:
            raise Exception(f"Invalid path for attribute {attr_name}")

        for slice in attr['location']['slices']:
            if slice == '*~':
                need_patch = True
                break

    if need_patch:
        repr = repr.clone()

        property_paths = []
        for attr_name, attr in repr.get_attributes().items():
            slices = attr['location']['slices']
            for i, slice in enumerate(slices):
                if slice == '*~':
                    property_paths.append((attr_name, i, slices[:i]))
                    break

        # now we need to make sure for every property paths, all paths that is overlap to the property paths,
        # need to be exactly equal and no property paths that is subpath of another property path
        for i in range(len(property_paths)):
            for j in range(i + 1, len(property_paths)):
                if is_path_overlap(property_paths[i][-1], property_paths[j][-1]):
                    raise Exception(
                        f"Does not handle nested selecting property names yet. Found in either {property_paths[i][0]} or {property_paths[i][1]}"
                    )

        overlapped_paths = defaultdict(lambda: [])
        for aid, attr in repr.get_attributes().items():
            slices = attr['location']['slices']
            for i, (paid, _, path) in enumerate(property_paths):
                if len(slices) > len(path) and is_path_overlap(path, slices):
                    assert slices[:len(path
                                       )] == path, f"Slices {slices} does not fully contain {path}"
                    overlapped_paths[i].append(aid)

        # add preprocessing functions
        attrs = repr.get_attributes()
        for paid, _, path in property_paths:
            # need to convert dict to key-value pairs
            repr.get_preprocess().append({
                'type': 'rmap-dict2items',
                'input': {
                    "resource_id": attrs[paid]['location']['resource_id'],
                    "slices": path,
                }
            })

        # now every property path is distinct, and we have a map of property path => attributes
        # that require to re-map their positions
        for pi, aids in overlapped_paths.items():
            _, _, path = property_paths[pi]
            for aid in aids:
                # remap attribute slices
                attr = attrs[aid]
                slices = attr['location']['slices']

                if slices[len(path)] == '*~':
                    # this select property names
                    slices = slices[:len(path)] + ['..', 0] + slices[len(path) + 1:]
                else:
                    assert slices[len(
                        path
                    )] == '..', 'Does not support select both all property names and just a subset of its values'
                    slices = slices[:len(path)] + ['..', 1] + slices[len(path) + 1:]
                attr['location']['slices'] = slices

                # remap alignments
                for alignment in repr.get_alignments():
                    if alignment['type'] == 'dimension':
                        if alignment['source'] == aid:
                            for adim in alignment['aligned_dims']:
                                if adim['source'] > len(path):
                                    adim['source'] += 1
                        elif alignment['target'] == aid:
                            for adim in alignment['aligned_dims']:
                                if adim['target'] > len(path):
                                    adim['target'] += 1

    return repr


def is_path_overlap(path_0: List[str], path_1: List[str]):
    def is_slice_overlap(s0, s1):
        if s0['type'] == 'range':
            if any(str(s0[key]).startswith("${") for key in ('start', 'end', 'step')):
                raise Exception("Haven't support compare expr overlapped yet")

            if s1['type'] == 'range':
                if any(str(s1[key]).startswith("${") for key in ('start', 'end', 'step')):
                    raise Exception("Haven't support compare expr overlapped yet")

                if s1['start'] < (s0['end'] or float('inf')) and (s1['end']
                                                                  or float('inf')) > s0['start']:
                    return True
                return False
            else:
                if isinstance(s1['idx'], str):
                    return False
                elif s0['start'] <= s1['idx'] < (s0['end'] or float('inf')):
                    return True
                return False
        else:
            if s1['type'] == 'range':
                if any(str(s1[key]).startswith("${") for key in ('start', 'end', 'step')):
                    raise Exception("Haven't support compare expr overlapped yet")

                if isinstance(s0['idx'], str):
                    return False
                elif s1['start'] <= s0['idx'] < (s1['end'] or float('inf')):
                    return True
                return False
            else:
                return s1['idx'] == s0['idx']

    slices_0 = [Repr.parse_slice(slice) for slice in path_0]
    slices_1 = [Repr.parse_slice(slice) for slice in path_1]

    if len(slices_0) > len(slices_1):
        slices_0, slices_1 = slices_1, slices_0

    for i in range(len(slices_0)):
        s0 = slices_0[i]
        s1 = slices_1[i]

        if not is_slice_overlap(s0, s1):
            return False

    return True
