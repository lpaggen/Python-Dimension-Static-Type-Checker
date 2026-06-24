from enum import Enum
import ast


class Operator(Enum):
    PLUS = "+"
    MINUS = "-"
    MUL = "*"
    MOD = "%"
    DIV = "/"
    POW = "**"

    @staticmethod
    def binop_tostr(binop: ast.operator) -> str:
        if isinstance(binop, ast.Add):
            return Operator.PLUS
        if isinstance(binop, ast.Sub):
            return Operator.MINUS
        if isinstance(binop, ast.Mult):
            return Operator.MUL
        if isinstance(binop, ast.Div):
            return Operator.DIV
