import re
from abc import ABC, abstractmethod
from typing import List, Union

from drepr.utils.validator import InputError
from ..path import Path, IndexExpr, RangeExpr, WildcardExpr, Expr
from ..resource import Resource


class PathParser(ABC):
    @abstractmethod
    def parse(self, resource: Resource, path: Union[str, list], parse_trace: str) -> Path:
        pass

    # noinspection PyMethodMayBeStatic
    def get_resource(self, resources: List[Resource], resource_id: str, trace: str) -> Resource:
        for res in resources:
            if res.id == resource_id:
                return res
        raise InputError(f"{trace}\nERROR: Refer to path of an nonexistent resource: {resource_id}")


class PathParserV1(PathParser):
    """
    A path can either be a JSONPath or our list path

    1. If the path is a JSONPath, then it is a string startswith `$`. We only support the following
       type of step: range, index, list of index, and wildcard. However, wildcard is only used for selecting
       all values of an object
    2. If the path is a normal string
    """
    REG_SRANGE = re.compile(r"^(\d+)?\.\.(-?\d+)?(?::(\d+))?$")
    REG_SINDEX = re.compile(r"^(?:\$\{([^}]+)})|(\d+)|(.*)$")
    REG_SRANGE_EXPR = re.compile(
        r"^(?:(\d+)|(?:\$\{([^}]+)}))?\.\.(?:(-\d+)|(?:\$\{([^}]+)}))?(?::(\d+)|(?:\$\{([^}]+)}))?$"
    )

    REG_JPATH_BRACKET = re.compile(r"(?:\[(-?\d+)?\:(?:(-?\d+)(?:\:(-?\d+))?)?\])|(?:\[(-?\d+)\])|(?:\['([^']+)'\])")
    REG_JPATH_DOT = re.compile(r"\.((?:(?!\.|\[).)+)")

    def parse(self, _resource: Resource, path: Union[str, list], parse_trace: str) -> Path:
        if isinstance(path, str):
            return self.parse_jsonpath(path, parse_trace)

        if isinstance(path, list):
            return self.parse_custom_path(path, parse_trace)

        raise InputError(f"{parse_trace}\nERROR: the path must either be a "
                         f"string (JSONPath) or a list of steps. Get {type(path)} instead")

    def parse_jsonpath(self, jpath: str, parse_trace: str) -> Path:
        if not jpath.startswith("$"):
            raise InputError(f"{parse_trace}\nERROR: invalid json path. The path must start with `$`. "
                             f"Get: {jpath}")

        jpath = jpath[1:]
        steps = []
        parsing_pos = 1

        while len(jpath) > 0:
            if jpath.startswith("["):
                m = self.REG_JPATH_BRACKET.match(jpath)
                if m is None:
                    raise InputError(
                        f"{parse_trace}\nERROR: invalid json path, error while parsing bracket at position {parsing_pos}")

                jpath = jpath[m.span()[-1]:]
                parsing_pos += m.span()[-1]  # m.span()[0] is always 0

                if m.group(5) is not None:
                    # match with string
                    steps.append(IndexExpr(m.group(5)))
                elif m.group(4) is not None:
                    # match with a single number
                    steps.append(IndexExpr(int(m.group(4))))
                else:
                    steps.append(RangeExpr(
                        int(m.group(1) or "0"),
                        int(m.group(2)) if m.group(2) is not None else None,
                        int(m.group(3) or "1")))
            elif jpath.startswith(".*~"):
                # *~ select property names
                steps.append(WildcardExpr.Names)
                jpath = jpath[3:]
                parsing_pos += 3
            elif jpath.startswith(".*"):
                steps.append(WildcardExpr.Values)
                jpath = jpath[2:]
                parsing_pos += 2
            else:
                m = self.REG_JPATH_DOT.match(jpath)
                if m is None:
                    raise InputError(
                        f"{parse_trace}\nERROR: invalid json path, error while parsing step at position {parsing_pos}")

                jpath = jpath[m.span()[-1]:]
                parsing_pos += m.span()[-1]  # m.span()[0] is always 0

                # after a dot, it can either be a number or a string
                if m.group(1).isdigit():
                    steps.append(IndexExpr(int(m.group(1))))
                else:
                    steps.append(IndexExpr(m.group(1)))

        return Path(steps)

    def parse_custom_path(self, path: List[str], parse_trace: str) -> Path:
        steps = []
        for i, step in enumerate(path):
            trace = f"Parsing step {i} ({step})"
            if isinstance(step, str):
                m = self.REG_SRANGE.match(step)
                if m is not None:
                    steps.append(RangeExpr(
                        int(m.group(1) or '0'),
                        int(m.group(2)) if m.group(2) is not None else None,
                        int(m.group(3) or '1')))
                    continue

                m = self.REG_SRANGE_EXPR.match(step)
                if m is not None:
                    steps.append(RangeExpr(
                        (Expr(m.group(1)[2:-1]) if m.group(1).startswith("${") else int(m.group(1)))
                        if m.group(1) is not None else 0,
                        (Expr(m.group(2)[2:-1]) if m.group(2).startswith("${") else int(m.group(2)))
                        if m.group(2) is not None else None,
                        (Expr(m.group(2)[2:-1]) if m.group(2).startswith("${") else int(m.group(2)))
                        if m.group(2) is not None else 1))
                    continue

                if step.startswith("${"):
                    steps.append(IndexExpr(Expr(step[2:-1])))
                else:
                    steps.append(IndexExpr(step))
            elif isinstance(step, int):
                steps.append(IndexExpr(step))
            else:
                raise InputError(
                    f"{parse_trace}\n{trace}\nERROR: step must either be string or number. Get {type(step)} instead")

        return Path(steps)
