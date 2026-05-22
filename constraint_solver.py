from z3 import Int, simplify
from dimension import KnownDim, Dim, BinaryDim, binop_tostr


from z3 import *

class ConstraintSolver:
    def __init__(self):
        self.solver = Solver()

    def to_z3(self, expr):  # fold simple constants
        print(type(expr))
        if isinstance(expr, int):
            return IntVal(expr)
        if isinstance(expr, KnownDim):
            return IntVal(expr.value)
        if isinstance(expr, Dim):
            return Int(expr.name)
        if isinstance(expr, BinaryDim):
            lhs = self.to_z3(expr.left)
            rhs = self.to_z3(expr.right)
            op = binop_tostr(expr.operator)
            if op == "+":
                return lhs + rhs
            elif op == "-":
                return lhs - rhs
            elif op == "*":
                return lhs * rhs
            elif op == "/":
                return lhs / rhs
        raise TypeError(f"Unsupported expr: {expr}")

    def solve(self, semantic_ir) -> bool:
        for node in semantic_ir:
            if isinstance(node, DimDecl) and node.value is not None:
                z3_expr = self.to_z3(node.value)  # can be BinaryDim or just int, fix
                var = Int(node.name)
                self.solver.add(var == z3_expr)
                self.solver.add(var > 0)

            if isinstance(node, TensorDecl) and isinstance(node.value, ast.Call):
                if isinstance(node.value.args, list):
                    found_cols = len(node.value.args[0].elts[0].elts)
                    found_rows = len(node.value.args[0].elts[0])


        result = self.solver.check()
        return result == sat