from dataclasses import dataclass, asdict
from enum import Enum
from typing import List, Optional, Union, Set


@dataclass
class Expr:
    expr: str


@dataclass
class RangeExpr:
    start: Union[int, Expr]
    end: Optional[Union[int, Expr]]
    step: Union[int, Expr]

    def is_select_all(self) -> bool:
        return self.start == 0 and self.end is None and self.step == 1


@dataclass
class IndexExpr:
    val: Union[str, int, Expr]


@dataclass
class SetIndexExpr:
    vals: Set[Union[str, int, Expr]]


class WildcardExpr(Enum):
    Values = "*"
    Names = "*~"


StepExpr = Union[RangeExpr, IndexExpr, SetIndexExpr, WildcardExpr]


@dataclass
class Path:
    steps: List[StepExpr]

    @staticmethod
    def deserialize(raw: dict) -> "Path":
        """
        Deserialize a dictionary to get back the Path object
        :param raw:
        :return:
        """
        steps = []
        for step in raw['steps']:
            if not isinstance(step, dict):
                steps.append(WildcardExpr(step))
            elif 'start' in step:
                # range index
                start = Expr(step['start']['expr']) if isinstance(step['start'], dict) else step['start']
                end = Expr(step['end']['expr']) if isinstance(step['end'], dict) else step['end']
                step1 = Expr(step['step']['expr']) if isinstance(step['step'], dict) else step['step']
                steps.append(RangeExpr(start, end, step1))
            elif 'val' in step:
                steps.append(IndexExpr(Expr(step['val']['expr']) if isinstance(step['val'], dict) else step['val']))
            elif 'vals' in step:
                steps.append(SetIndexExpr({
                    Expr(val['expr']) if isinstance(val, dict) else val
                    for val in step['vals']
                }))
        return Path(steps)

    def to_engine_format(self) -> dict:
        steps = []
        for step in self.steps:
            if isinstance(step, RangeExpr):
                if step.end is None:
                    end = None
                elif isinstance(step.end, Expr):
                    end = f"${{{step.end.expr}}}"
                else:
                    end = step.end

                steps.append({
                    "type": "range",
                    "start": f"${{{step.start.expr}}}" if isinstance(step.start, Expr) else step.start,
                    "end": end,
                    "step": f"${{{step.step.expr}}}" if isinstance(step.step, Expr) else step.step
                })
            elif isinstance(step, IndexExpr):
                steps.append({
                    "type": "index",
                    "val": {
                        "t": "str" if isinstance(step.val, str) else "idx",
                        "c": step.val
                    }
                })
            elif isinstance(step, SetIndexExpr):
                steps.append({
                    "type": "set_index",
                    "values": [
                        {
                            "t": "str" if isinstance(val, str) else "idx",
                            "c": val
                        }
                        for val in step.vals
                    ]
                })
            elif isinstance(step, WildcardExpr):
                if step == WildcardExpr.Values:
                    steps.append({
                        "type": "wildcard"
                    })
                else:
                    raise NotImplementedError("We haven't supported operator `*~` yet")
        return {
            "steps": steps
        }

    def to_lang_format(self, use_json_path: bool = False) -> Union[list, str]:
        """
        Convert this Path object into the path object in the D-REPR language

        :param use_json_path: whether we should use the JSONPath or our new notation
        :return:
        """
        if use_json_path:
            jpath = ["$"]
            for step in self.steps:
                if isinstance(step, RangeExpr):
                    if any(isinstance(v, Expr) for v in [step.start, step.end, step.step]):
                        raise NotImplementedError("Haven't supported JSONPath with expression yet")
                    jpath.append(f"[{step.start}:{step.end or ''}:{step.step}]")
                elif isinstance(step, IndexExpr):
                    if isinstance(step.val, str):
                        jpath.append(f"['{step.val}']")
                    else:
                        jpath.append(f"[{step.val}]")
                elif isinstance(step, SetIndexExpr):
                    raise NotImplementedError()
                else:
                    jpath.append(f".{step.value}")
            return "".join(jpath)

        path = []
        for step in self.steps:
            if isinstance(step, RangeExpr):
                start, end, step = [
                    "" if v is None else (
                        f"${{{v.expr}}}" if isinstance(v, Expr) else v
                    )
                    for v in [step.start, step.end, step.step]
                ]
                path.append(f"{start}..{end}:{step}")
            elif isinstance(step, IndexExpr):
                path.append(step.val)
            elif isinstance(step, SetIndexExpr):
                path.append(step.vals)
            elif isinstance(step, WildcardExpr):
                path.append(step.value)
            else:
                raise NotImplementedError()
        return path