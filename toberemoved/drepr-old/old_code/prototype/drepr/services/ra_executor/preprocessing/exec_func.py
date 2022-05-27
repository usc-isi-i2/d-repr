from datetime import datetime
from typing import List, Dict, Tuple, Callable, Any, Optional

from drepr.services.ra_reader.ra_reader import RAReader


def strftime(dt: datetime, fmt: str):
    return dt.strftime(fmt)


def compile_func(ra_reader: RAReader, func: str):
    assert '__return__' not in func
    fname = f"func_trans_tt8tty1"
    new_code = [f"def {fname}(value, index):"]

    # figure out that they use tab or space
    indent_char = "\t"
    lines = func.split("\n")
    for line in lines:
        if line.startswith("\t"):
            indent_char = "\t"
            break
        elif line.startswith(" "):
            indent_char = " " * (len(line) - len(line.lstrip()))
            break

    for line in lines:
        new_code.append(indent_char + line)

    new_code.append(f"__return__ = {fname}(value, index)")
    new_code = "\n".join(new_code)

    # TODO: fix me! there may be a function have suffix: get_previous_value()
    new_code = new_code.replace('get_previous_value()', '__prev_value__')

    sess_locals = {
        'value': None,
        'index': None,
    }
    sess_globals = {
        'get_value': ra_reader.get_value,
        'strptime': datetime.strptime,
        'strftime': strftime,
        '__prev_value__': None,
    }

    return compile(new_code, '<string>', 'exec'), sess_globals, sess_locals


def exec_func(value, index, func_args):
    func_args[2]['value'] = value
    func_args[2]['index'] = index
    exec(func_args[0], func_args[1], func_args[2])
    return func_args[2]['__return__']
