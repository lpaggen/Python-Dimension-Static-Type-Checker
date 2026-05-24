from custom_literals import *
import ast
from dimension import *


class TorchOpParser:
    def __init__(self):
        self.rules = {
            "tensor": self._parse_tensor,
            "randn": self._parse_randn,
            "matmul": self._parse_matmul,
            "add": self._parse_add
        }

    def _parse_matmul(self, node: ast.Call):
        args = [i.id for i in node.args]
        return MatMulExpr(args[0], args[1])  # store only identifiers, constraintSolver calls upon env to resolve shapes
    
    def _parse_randn(self, node: ast.Call):  # assume 2 elements, see if this scales
        return RandnExpr((Dim.toDim(node.args[0]), Dim.toDim(node.args[1])))
    
    def _parse_add(self, node: ast.Call):
        args = [i.id for i in node.args]
        return AddExpr(args[0], args[1])

    def _parse_tensor(self, node):  # here parse the tensor and get the shape of the RHS + rectangular, this is for Literal [[..]]
        rows_lit = node.args[0].elts
        rows = len(rows_lit)
        cols = len(node.args[0].elts[0].elts)

        for row in rows_lit:
            if not isinstance(row, ast.List):
                raise TypeError("Tensor rows must be lists")

            if len(row.elts) != cols:
                raise TypeError(
                    f"Non-rectangular tensor literal: expected {cols} cols, got {len(row.elts)}"
                )

        return TensorLiteralExpr((rows, cols))

    def parse(self, node):
        if not isinstance(node, ast.Call):
            return None

        if not isinstance(node.func, ast.Attribute):
            return None

        if not isinstance(node.func.value, ast.Name):
            return None

        if node.func.value.id != "torch":  # can add param in init to support alias eventually
            return None

        op = node.func.attr

        if op not in self.rules:
            raise TypeError(f"Unsupported torch op: {op}")

        return self.rules[op](node)
