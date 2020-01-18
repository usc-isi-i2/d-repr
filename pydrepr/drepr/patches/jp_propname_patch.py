import copy
from collections import defaultdict
from typing import List, Dict, Tuple, Callable, Any, Optional

from drepr.models import DRepr, WildcardExpr, StepExpr, RangeExpr, Expr, IndexExpr, Preprocessing, PreprocessingType, \
    RMap, RMapFunc, Path, RangeAlignment
from drepr.patches import ResourceData


def patch(repr: DRepr, _resources: Dict[str, ResourceData]) -> DRepr:
    """
    This patch will fix json path that select property names, before we officially support
    selecting property names

    How does it work: it creates a preprocessing function that convert an object to list of key-value pairs
    """
    need_patch = False
    for pfunc in repr.preprocessing:
        for step in pfunc.value.path.steps:
            if step == WildcardExpr.Names:
                raise Exception(
                    "Does not support preprocessing that is applied on properties of an object yet!"
                )

    for attr in repr.attrs:
        n_wildcard_names = sum(int(step == WildcardExpr.Names) for step in attr.path.steps)
        if n_wildcard_names > 1:
            # cannot select names inside names
            raise Exception(f"Invalid path for attribute {attr.id}")

        if n_wildcard_names > 0:
            need_patch = True

    if need_patch:
        repr = copy.deepcopy(repr)

        # first, find paths that containing the WildcardExpr.Names (*~)
        property_paths = []
        for attr in repr.attrs:
            for i, step in enumerate(attr.path.steps):
                if step == WildcardExpr.Names:
                    property_paths.append((attr.id, i, attr.path.steps[:i]))
                    break

        # now we need to make sure for every property paths, all paths that is overlap to the property paths,
        # need to be exactly equal and no property paths that is subpath of another property path
        for i in range(len(property_paths)):
            for j in range(i + 1, len(property_paths)):
                if is_path_overlap(property_paths[i][-1], property_paths[j][-1]):
                    raise Exception(
                        f"Does not handle nested selecting property names yet. Found in either {property_paths[i][0]} or {property_paths[j][0]}"
                    )

        overlapped_paths = defaultdict(lambda: [])
        for attr in repr.attrs:
            steps = attr.path.steps
            for i, (paid, _, path) in enumerate(property_paths):
                if len(steps) > len(path) and is_path_overlap(path, steps):
                    assert steps[:len(path)] == path, f"Slices {steps} does not fully contain {path}"
                    overlapped_paths[i].append(attr.id)
        # add preprocessing functions
        id2attr = {attr.id: attr for attr in repr.attrs}
        for paid, _, path in property_paths:
            # need to convert dict to key-value pairs
            repr.preprocessing.append(Preprocessing(
                PreprocessingType.rmap,
                RMap(id2attr[paid].resource_id, Path(path), RMapFunc.Dict2Items)
            ))

        # now every property path is distinct, and we have a map of property path => attributes
        # that require to re-map their positions
        for pi, aids in overlapped_paths.items():
            _, _, path = property_paths[pi]
            for aid in aids:
                # remap attribute slices
                attr = id2attr[aid]
                steps = attr.path.steps

                if steps[len(path)] == WildcardExpr.Names:
                    # this select property names
                    steps = steps[:len(path)] + [RangeExpr(0, None, 1), IndexExpr(0)] + steps[len(path) + 1:]
                else:
                    assert steps[len(
                        path
                    )] == WildcardExpr.Values, 'Does not support select both all property names and just a subset of its values'
                    steps = steps[:len(path)] + [RangeExpr(0, None, 1), IndexExpr(1)] + steps[len(path) + 1:]
                attr.path.steps = steps

                # remap alignments
                for alignment in repr.aligns:
                    if isinstance(alignment, RangeAlignment):
                        if alignment.source == aid:
                            for adim in alignment.aligned_steps:
                                if adim.source_idx > len(path):
                                    adim.source_idx += 1
                        elif alignment.target == aid:
                            for adim in alignment.aligned_steps:
                                if adim.target_idx > len(path):
                                    adim.target_idx += 1

    return repr


def is_path_overlap(path_0: List[StepExpr], path_1: List[StepExpr]):
    def is_slice_overlap(s0, s1):
        if isinstance(s0, RangeExpr):
            if any(isinstance(v, Expr) for v in [s0.start, s0.end, s0.step]):
                raise Exception("Haven't support compare expr overlapped yet")

            if isinstance(s1, RangeExpr):
                if any(isinstance(v, Expr) for v in [s1.start, s1.end, s1.step]):
                    raise Exception("Haven't support compare expr overlapped yet")

                return s1.start < (s0.end or float('inf')) and (s1.end or float('inf')) > s0.start
            elif isinstance(s1, IndexExpr):
                if isinstance(s1.val, str) or isinstance(s1.val, Expr):
                    return False
                elif s0.start <= s1.val < (s0.end or float('inf')):
                    return True
                return False
        elif isinstance(s0, IndexExpr):
            if isinstance(s1, RangeExpr):
                if any(isinstance(v, Expr) for v in [s1.start, s1.end, s1.step]):
                    raise Exception("Haven't support compare expr overlapped yet")

                if isinstance(s0.val, (str, Expr)):
                    return False
                elif s1.start <= s0.val < (s1.end or float('inf')):
                    return True
                return False
            elif isinstance(s1, IndexExpr):
                return s1.val == s1.val
        else:
            # not index & range does not have overlapping?
            return s0 == s1

    if len(path_0) > len(path_1):
        path_0, path_1 = path_1, path_0

    for i in range(len(path_0)):
        s0 = path_0[i]
        s1 = path_1[i]

        if not is_slice_overlap(s0, s1):
            return False

    return True
