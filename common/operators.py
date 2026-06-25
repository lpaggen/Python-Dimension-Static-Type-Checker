from enum import Enum
import ast


class Operator(Enum):
    PLUS = "+"
    MINUS = "-"
    MUL = "*"
    MATMUL = "@"
    DIV = "/"
    FLOORDIV = "//"
    MOD = "%"
    POW = "**"

    LSHIFT = "<<"
    RSHIFT = ">>"
    BITOR = "|"
    BITXOR = "^"
    BITAND = "&"

    UPLUS = "+u"
    UMINUS = "-u"
    NOT = "not"
    INVERT = "~"

    AND = "and"
    OR = "or"

    EQ = "=="
    NE = "!="
    LT = "<"
    LTE = "<="
    GT = ">"
    GTE = ">="
    IS = "is"
    IS_NOT = "is not"
    IN = "in"
    NOT_IN = "not in"

    PLUS_ASSIGN = "+="
    MINUS_ASSIGN = "-="
    MUL_ASSIGN = "*="
    MATMUL_ASSIGN = "@="
    DIV_ASSIGN = "/="
    FLOORDIV_ASSIGN = "//="
    MOD_ASSIGN = "%="
    POW_ASSIGN = "**="
    LSHIFT_ASSIGN = "<<="
    RSHIFT_ASSIGN = ">>="
    BITOR_ASSIGN = "|="
    BITXOR_ASSIGN = "^="
    BITAND_ASSIGN = "&="

    @staticmethod
    def binop_tostr(binop: ast.operator) -> "Operator":
        mapping = {
            ast.Add: Operator.PLUS,
            ast.Sub: Operator.MINUS,
            ast.Mult: Operator.MUL,
            ast.MatMult: Operator.MATMUL,
            ast.Div: Operator.DIV,
            ast.FloorDiv: Operator.FLOORDIV,
            ast.Mod: Operator.MOD,
            ast.Pow: Operator.POW,
            ast.LShift: Operator.LSHIFT,
            ast.RShift: Operator.RSHIFT,
            ast.BitOr: Operator.BITOR,
            ast.BitXor: Operator.BITXOR,
            ast.BitAnd: Operator.BITAND,
        }

        for ast_type, op in mapping.items():
            if isinstance(binop, ast_type):
                return op

        raise NotImplementedError(
            f"Unsupported binary operator: {type(binop).__name__}"
        )

    @staticmethod
    def unaryop_tostr(unaryop: ast.unaryop) -> "Operator":
        mapping = {
            ast.UAdd: Operator.UPLUS,
            ast.USub: Operator.UMINUS,
            ast.Not: Operator.NOT,
            ast.Invert: Operator.INVERT,
        }

        for ast_type, op in mapping.items():
            if isinstance(unaryop, ast_type):
                return op

        raise NotImplementedError(
            f"Unsupported unary operator: {type(unaryop).__name__}"
        )

    @staticmethod
    def boolop_tostr(boolop: ast.boolop) -> "Operator":
        if isinstance(boolop, ast.And):
            return Operator.AND
        if isinstance(boolop, ast.Or):
            return Operator.OR

        raise NotImplementedError(
            f"Unsupported boolean operator: {type(boolop).__name__}"
        )

    @staticmethod
    def cmpop_tostr(cmpop: ast.cmpop) -> "Operator":
        mapping = {
            ast.Eq: Operator.EQ,
            ast.NotEq: Operator.NE,
            ast.Lt: Operator.LT,
            ast.LtE: Operator.LTE,
            ast.Gt: Operator.GT,
            ast.GtE: Operator.GTE,
            ast.Is: Operator.IS,
            ast.IsNot: Operator.IS_NOT,
            ast.In: Operator.IN,
            ast.NotIn: Operator.NOT_IN,
        }

        for ast_type, op in mapping.items():
            if isinstance(cmpop, ast_type):
                return op

        raise NotImplementedError(
            f"Unsupported comparison operator: {type(cmpop).__name__}"
        )

    @staticmethod
    def augop_to_operator(op: ast.operator) -> "Operator":
        mapping = {
            ast.Add: Operator.PLUS_ASSIGN,
            ast.Sub: Operator.MINUS_ASSIGN,
            ast.Mult: Operator.MUL_ASSIGN,
            ast.MatMult: Operator.MATMUL_ASSIGN,
            ast.Div: Operator.DIV_ASSIGN,
            ast.FloorDiv: Operator.FLOORDIV_ASSIGN,
            ast.Mod: Operator.MOD_ASSIGN,
            ast.Pow: Operator.POW_ASSIGN,
            ast.LShift: Operator.LSHIFT_ASSIGN,
            ast.RShift: Operator.RSHIFT_ASSIGN,
            ast.BitOr: Operator.BITOR_ASSIGN,
            ast.BitXor: Operator.BITXOR_ASSIGN,
            ast.BitAnd: Operator.BITAND_ASSIGN,
        }

        for ast_type, operator in mapping.items():
            if isinstance(op, ast_type):
                return operator

        raise NotImplementedError(
            f"Unsupported augmented assignment operator: {type(op).__name__}"
        )
