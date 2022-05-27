class PyExec:
    def __init__(self, func):
        self.func = func

    @staticmethod
    def compile(code: str) -> "PyExec":
        lines = code.split("\n")
        indent = PyExec.detect_indent(lines)

        code = "def func(value, index, context):\n" + "\n".join([
            indent + line
            for line in lines
        ])
        session_locals = {}
        exec(code, None, session_locals)
        return PyExec(session_locals['func'])

    def exec(self, value, index, context):
        return self.func(value, index, context)

    @staticmethod
    def detect_indent(lines: str) -> str:
        indent = "\t"
        for line in lines:
            if line.startswith("\t"):
                break

            if line.startswith(" "):
                n = 0
                for c in line:
                    if c != " ":
                        indent = " " * n
                        break
                    n += 1
                break
        return indent
