import ast


class Dim:
    def __init__(self, name):
        self.name = name

class KnownDim(Dim):
    def __init__(self, value):
        self.value = value

class BinaryDim(Dim):
    def __init__(self, operator, left, right):
        self.operator = operator
        self.left = left
        self.right = right

    def __repr__(self):
        return f"BinaryDim({self.operator}, {self.left.value}, {self.right.value})"

class DimDecl:
    def __init__(self, name, value):
        self.name = name
        self.value = value

    def __repr__(self):
        return f"DimDecl({self.name}, {self.value})"

def binop_tostr(binop: ast.BinOp) -> str:
    if isinstance(binop, ast.Add):
        return "+"
    if isinstance(binop, ast.Sub):
        return "-"
    if isinstance(binop, ast.Mult):
        return "*"
    if isinstance(binop, ast.Div):
        return "/"

# def fold(expr):
#     if isinstance(expr, KnownDim):
#         return expr
#     if isinstance(expr, BinaryDim):
#         left = fold(expr.left)
#         right = fold(expr.right)
#         if isinstance(left, KnownDim) and isinstance(right, KnownDim):
#             op = binop_tostr(expr.operator)
#             if op == "+":
#                 return left + right
#             elif op == "-":
#                 return left - right
#             elif op == "*":
#                 return left * right
#             elif op == "/":
#                 return left / right
#         return BinaryDim(left, expr.operator, right)
#     return expr
