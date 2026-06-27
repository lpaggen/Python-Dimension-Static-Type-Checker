from enum import Enum
import ast


from enum import IntEnum
import ast


class Operator(IntEnum):
    OP_UNKNOWN = 0

    OP_PLUS = 1
    OP_MINUS = 2
    OP_MUL = 3
    OP_MATMUL = 4
    OP_DIV = 5
    OP_FLOORDIV = 6
    OP_MOD = 7
    OP_POW = 8

    OP_LSHIFT = 9
    OP_RSHIFT = 10
    OP_BITOR = 11
    OP_BITXOR = 12
    OP_BITAND = 13

    OP_UPLUS = 14
    OP_UMINUS = 15
    OP_NOT = 16
    OP_INVERT = 17

    OP_AND = 18
    OP_OR = 19

    OP_EQ = 20
    OP_NE = 21
    OP_LT = 22
    OP_LTE = 23
    OP_GT = 24
    OP_GTE = 25
    OP_IS = 26
    OP_IS_NOT = 27
    OP_IN = 28
    OP_NOT_IN = 29

    OP_PLUS_ASSIGN = 30
    OP_MINUS_ASSIGN = 31
    OP_MUL_ASSIGN = 32
    OP_MATMUL_ASSIGN = 33
    OP_DIV_ASSIGN = 34
    OP_FLOORDIV_ASSIGN = 35
    OP_MOD_ASSIGN = 36
    OP_POW_ASSIGN = 37
    OP_LSHIFT_ASSIGN = 38
    OP_RSHIFT_ASSIGN = 39
    OP_BITOR_ASSIGN = 40
    OP_BITXOR_ASSIGN = 41
    OP_BITAND_ASSIGN = 42

    @staticmethod
    def binop_to_operator(binop: ast.operator) -> "Operator":
        mapping = {
            ast.Add: Operator.OP_PLUS,
            ast.Sub: Operator.OP_MINUS,
            ast.Mult: Operator.OP_MUL,
            ast.MatMult: Operator.OP_MATMUL,
            ast.Div: Operator.OP_DIV,
            ast.FloorDiv: Operator.OP_FLOORDIV,
            ast.Mod: Operator.OP_MOD,
            ast.Pow: Operator.OP_POW,
            ast.LShift: Operator.OP_LSHIFT,
            ast.RShift: Operator.OP_RSHIFT,
            ast.BitOr: Operator.OP_BITOR,
            ast.BitXor: Operator.OP_BITXOR,
            ast.BitAnd: Operator.OP_BITAND,
        }

        for ast_type, op in mapping.items():
            if isinstance(binop, ast_type):
                return op

        raise NotImplementedError(
            f"Unsupported binary operator: {type(binop).__name__}"
        )

    @staticmethod
    def unaryop_to_operator(unaryop: ast.unaryop) -> "Operator":
        mapping = {
            ast.UAdd: Operator.OP_UPLUS,
            ast.USub: Operator.OP_UMINUS,
            ast.Not: Operator.OP_NOT,
            ast.Invert: Operator.OP_INVERT,
        }

        for ast_type, op in mapping.items():
            if isinstance(unaryop, ast_type):
                return op

        raise NotImplementedError(
            f"Unsupported unary operator: {type(unaryop).__name__}"
        )

    @staticmethod
    def boolop_to_operator(boolop: ast.boolop) -> "Operator":
        if isinstance(boolop, ast.And):
            return Operator.OP_AND
        if isinstance(boolop, ast.Or):
            return Operator.OP_OR

        raise NotImplementedError(
            f"Unsupported boolean operator: {type(boolop).__name__}"
        )

    @staticmethod
    def cmpop_to_operator(cmpop: ast.cmpop) -> "Operator":
        mapping = {
            ast.Eq: Operator.OP_EQ,
            ast.NotEq: Operator.OP_NE,
            ast.Lt: Operator.OP_LT,
            ast.LtE: Operator.OP_LTE,
            ast.Gt: Operator.OP_GT,
            ast.GtE: Operator.OP_GTE,
            ast.Is: Operator.OP_IS,
            ast.IsNot: Operator.OP_IS_NOT,
            ast.In: Operator.OP_IN,
            ast.NotIn: Operator.OP_NOT_IN,
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
            ast.Add: Operator.OP_PLUS_ASSIGN,
            ast.Sub: Operator.OP_MINUS_ASSIGN,
            ast.Mult: Operator.OP_MUL_ASSIGN,
            ast.MatMult: Operator.OP_MATMUL_ASSIGN,
            ast.Div: Operator.OP_DIV_ASSIGN,
            ast.FloorDiv: Operator.OP_FLOORDIV_ASSIGN,
            ast.Mod: Operator.OP_MOD_ASSIGN,
            ast.Pow: Operator.OP_POW_ASSIGN,
            ast.LShift: Operator.OP_LSHIFT_ASSIGN,
            ast.RShift: Operator.OP_RSHIFT_ASSIGN,
            ast.BitOr: Operator.OP_BITOR_ASSIGN,
            ast.BitXor: Operator.OP_BITXOR_ASSIGN,
            ast.BitAnd: Operator.OP_BITAND_ASSIGN,
        }

        for ast_type, operator in mapping.items():
            if isinstance(op, ast_type):
                return operator

        raise NotImplementedError(
            f"Unsupported augmented assignment operator: {type(op).__name__}"
        )
