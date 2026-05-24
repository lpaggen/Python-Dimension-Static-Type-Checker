from __future__ import annotations
import ast


class Dim:
    def __init__(self):
        pass

    @staticmethod
    def toDim(node) -> Dim:
        if isinstance(node, ast.Name):
            return SymDim(node.id)
        if isinstance(node, ast.Constant):
            return KnownDim(node.value)
        if isinstance(node, ast.BinOp):
            return BinaryDim(
                Dim.binop_tostr(node.op),
                Dim.toDim(node.left),
                Dim.toDim(node.right)
            )
        raise TypeError(f"Unsupported dimension node: {node}")
    
    @staticmethod
    def binop_tostr(binop: ast.operator) -> str:
        if isinstance(binop, ast.Add):
            return "+"
        if isinstance(binop, ast.Sub):
            return "-"
        if isinstance(binop, ast.Mult):
            return "*"
        if isinstance(binop, ast.Div):
            return "/"

class UnknownDim(Dim):
    def __init__(self):
        pass

class SymDim(Dim):
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
