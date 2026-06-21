from custom_literals import *
import ast
from dimension import *
from custom_types import MatrixType, ScalarType, VarType
from ast import Name, Constant


class TorchOpParser:
    def __init__(self):
        self.rules = {
            "tensor": self._parse_tensor,
            "randn": self._parse_randn,
            "matmul": self._parse_matmul,
            "add": self._parse_add
        }

    def _resolve_type(self, expr) -> MatrixType | ScalarType:
        match expr:
            case Name(id=id):
                return VarType(id)
            case Constant(value=n):
                return ScalarType(n)
            case _:
                raise TypeError("No")

    # TODO remove all instances of Dim.toDim, this is not the right place to have it
    def _parse_matmul(self, node: ast.Call):
        return MatMulExpr(self._resolve_type(node.args[0]), self._resolve_type(node.args[1]))

    def _parse_randn(self, node: ast.Call):  # assume 2 elements, see if this scales
        return RandnExpr((self._resolve_type(node.args[0]), self._resolve_type(node.args[1])))

    def _parse_add(self, node: ast.Call):
        return AddExpr(self._resolve_type(node.args[0]), self._resolve_type(node.args[1]))

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

        return TensorLiteral((rows, cols))

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
