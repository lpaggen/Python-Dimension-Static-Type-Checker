from __future__ import annotations
import ast
from common.operators import Operator
from .ir_node import IRNode


class DimIR(IRNode):
    def __init__(self, span):
        super.__init__(span=span)
        self.span = span

    @staticmethod
    def toDim(node) -> DimIR:
        if isinstance(node, ast.Name):
            return SymDim(node.id)
        if isinstance(node, ast.Constant):
            return ScalarDim(node.value)
        if isinstance(node, ast.BinOp):
            return BinaryDim(
                DimIR.binop_tostr(node.op),
                DimIR.toDim(node.left),
                DimIR.toDim(node.right),
            )
        raise TypeError(f"Unsupported dimension node: {node}")


class UnknownDim(DimIR):
    def __init__(self):
        pass


class SymDim(DimIR):
    def __init__(self, name):
        self.name = name

    def __str__(self):
        return self.name


class ScalarDim(DimIR):
    def __init__(self, value):
        self.value = value


class BinaryDim(DimIR):
    def __init__(self, operator: Operator, left, right):
        self.operator = operator
        self.left = left
        self.right = right

    def __repr__(self):
        return f"BinaryDim({self.operator}, {self.left.value}, {self.right.value})"
