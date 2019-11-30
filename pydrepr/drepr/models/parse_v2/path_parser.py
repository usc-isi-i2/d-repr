import re
from abc import ABC, abstractmethod
from copy import copy
from typing import List, Union

from drepr.models.parse_v1.path_parser import PathParser
from drepr.utils.validator import InputError
from ..path import Path, IndexExpr, RangeExpr, WildcardExpr, Expr
from ..resource import Resource, ResourceType


class PathParserV2(PathParser):
    """
    Similar to path parser v1. However, we now allow special using indexing name per resource type

    For spreadsheet, we allow column to be letter
    """
    REG_SRANGE = re.compile(r"^(\d+)?\.\.(-?\d+)?(?::(\d+))?$")
    REG_SINDEX = re.compile(r"^(?:\$\{([^}]+)})|(\d+)|(.*)$")
    REG_SRANGE_EXPR = re.compile(
        r"^(?:(\d+)|(?:\$\{([^}]+)}))?\.\.(?:(-\d+)|(?:\$\{([^}]+)}))?(?::(\d+)|(?:\$\{([^}]+)}))?$"
    )

    REG_JPATH_BRACKET = re.compile(
        r"(?:\[(-?\d+)?\:(-?\d+)?(?:\:(-?\d+))?\])|(?:\[(-?\d+)\])|(?:\['([^']+)'\])")
    REG_JPATH_DOT = re.compile(r"\.((?:(?!\.|\[).)+)")

    def parse(self, resource: Resource, path: Union[str, list], parse_trace: str) -> Path:
        if isinstance(path, str):
            return self.parse_jsonpath(resource, path, parse_trace)

        if isinstance(path, list):
            return self.parse_custom_path(resource, path, parse_trace)

        raise InputError(f"{parse_trace}\nERROR: the path must either be a "
                         f"string (JSONPath) or a list of steps. Get {type(path)} instead")

    # noinspection PyMethodMayBeStatic
    def letter2index(self, letter: str) -> int:
        letter = list(letter.lower())
        n_chars = ord('z') - ord('a') + 1
        index = 0
        for i, c in enumerate(reversed(letter)):
            assert ord('a') <= ord(c) <= ord('z'), f'{c} is not a valid column in spreadsheet'
            index += (ord(c) - ord('a') + 1) * (n_chars**i)
        return index - 1

    def isdigit(self, s: str) -> bool:
        if s.startswith("-"):
            return s[1:].isdigit()
        return s.isdigit()

    def parse_jsonpath(self, resource: Resource, jpath: str, parse_trace: str) -> Path:
        if not jpath.startswith("$"):
            raise InputError(
                f"{parse_trace}\nERROR: invalid json path. The path must start with `$`. "
                f"Get: {jpath}")

        jpath = jpath[1:]
        steps = []
        parsing_pos = 1

        # pre-processing the spreadsheet resource to allow letter column
        if resource.type == ResourceType.Spreadsheet:
            last_step_index = max(jpath.rfind("["), jpath.rfind("."))
            if jpath[last_step_index] == "[":
                last_step = jpath[last_step_index + 1:-1]
                result = last_step.split(":")
                if len(result) == 1:
                    index = result[0]
                    if not self.isdigit(index):
                        new_last_step = self.letter2index(index)
                    else:
                        new_last_step = index
                else:
                    if len(result) == 3:
                        start, end, step = result
                    elif len(result) == 2:
                        start, end = result
                        step = 1
                    else:
                        raise InputError(f"{parse_trace}\nERROR: invalid path")

                    if (len(start) > 0 and not self.isdigit(start)) or (len(end) > 0
                                                                        and not self.isdigit(end)):
                        # they use letter system, otherwise, do nothing
                        if (len(start) > 0 and self.isdigit(start)) or (len(end) > 0
                                                                        and self.isdigit(end)):
                            raise InputError(
                                f"{parse_trace}\nERROR: Cannot mixed between number and letter index"
                            )
                        start = self.letter2index(start)
                        if len(end) > 0:
                            end = self.letter2index(end)
                        new_last_step = f"{start}:{end}:{step}"
                    else:
                        new_last_step = f"{start}:{end}:{step}"

                jpath = jpath[:last_step_index] + f"[{new_last_step}]"
            elif jpath[last_step_index] == ".":
                last_step = jpath[last_step_index + 1:]
                if not self.isdigit(last_step):
                    new_last_step = self.letter2index(last_step)
                else:
                    new_last_step = last_step
                jpath = jpath[:last_step_index] + f".{new_last_step}"

        while len(jpath) > 0:
            if jpath.startswith("["):
                m = self.REG_JPATH_BRACKET.match(jpath)
                if m is None:
                    raise InputError(
                        f"{parse_trace}\nERROR: invalid json path, error while parsing bracket at position {parsing_pos}"
                    )

                jpath = jpath[m.span()[-1]:]
                parsing_pos += m.span()[-1]  # m.span()[0] is always 0

                if m.group(5) is not None:
                    # match with string
                    steps.append(IndexExpr(m.group(5)))
                elif m.group(4) is not None:
                    # match with a single number
                    steps.append(IndexExpr(int(m.group(4))))
                else:
                    steps.append(
                        RangeExpr(int(m.group(1) or "0"),
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
                        f"{parse_trace}\nERROR: invalid json path, error while parsing step at position {parsing_pos}"
                    )

                jpath = jpath[m.span()[-1]:]
                parsing_pos += m.span()[-1]  # m.span()[0] is always 0

                # after a dot, it can either be a number or a string
                if m.group(1).isdigit():
                    steps.append(IndexExpr(int(m.group(1))))
                else:
                    steps.append(IndexExpr(m.group(1)))

        return Path(steps)

    def parse_custom_path(self, resource: Resource, path: List[str], parse_trace: str) -> Path:
        if resource.type == ResourceType.Spreadsheet:
            path = copy(path)
            last_step = path[-1]
            if isinstance(last_step, str):
                if last_step.find("..") != -1:
                    tmp = last_step.split(":")
                    start, end = tmp[0].split("..")
                    if len(tmp) == 2:
                        step = f":{tmp[1]}"
                    else:
                        step = ""

                    if (len(start) > 0 and not self.isdigit(start)) or (len(end) > 0
                                                                        and not self.isdigit(end)):
                        # they use letter system, otherwise, do nothing
                        if (len(start) > 0 and self.isdigit(start)) or (len(end) > 0
                                                                        and self.isdigit(end)):
                            raise InputError(
                                f"{parse_trace}\nERROR: Cannot mixed between number and letter index"
                            )
                        start = self.letter2index(start)
                        if len(end) > 0:
                            end = self.letter2index(end)
                        new_last_step = f"{start}..{end}{step}"
                    else:
                        new_last_step = f"{start}..{end}{step}"
                    path[-1] = new_last_step
                elif not self.isdigit(last_step):
                    path[-1] = self.letter2index(last_step)

        steps = []
        for i, step in enumerate(path):
            trace = f"Parsing step {i} ({step})"
            if isinstance(step, str):
                m = self.REG_SRANGE.match(step)
                if m is not None:
                    steps.append(
                        RangeExpr(int(m.group(1) or '0'),
                                  int(m.group(2)) if m.group(2) is not None else None,
                                  int(m.group(3) or '1')))
                    continue

                m = self.REG_SRANGE_EXPR.match(step)
                if m is not None:
                    steps.append(
                        RangeExpr((Expr(m.group(1)[2:-1]) if m.group(1).startswith("${") else int(
                            m.group(1))) if m.group(1) is not None else 0,
                                  (Expr(m.group(2)[2:-1]) if m.group(2).startswith("${") else int(
                                      m.group(2))) if m.group(2) is not None else None,
                                  (Expr(m.group(2)[2:-1]) if m.group(2).startswith("${") else int(
                                      m.group(2))) if m.group(2) is not None else 1))
                    continue

                if step.startswith("${"):
                    steps.append(IndexExpr(Expr(step[2:-1])))
                else:
                    steps.append(IndexExpr(step))
            elif isinstance(step, int):
                steps.append(IndexExpr(step))
            else:
                raise InputError(
                    f"{parse_trace}\n{trace}\nERROR: step must either be string or number. Get {type(step)} instead"
                )

        return Path(steps)
