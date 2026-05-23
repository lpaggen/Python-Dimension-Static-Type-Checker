from z3 import Int, sat, Solver, IntVal, AstRef
from dimension import KnownDim, Dim, BinaryDim, binop_tostr, DimDecl, SymDim
from tensor_decl import TensorDecl
from rules import Rules
from custom_literals import *


class ConstraintSolver:
    def __init__(self, env, report_errors=True, crash_if_error=False):
        self.solver = Solver()
        self.errors = []  # append errors as we go, inform the user and crash at the end only if provided crash=true arg?
        self.rules = Rules()
        self.env = env  # shared context

    def to_z3(self, expr: Dim) -> AstRef:  # fold simple constants
        if isinstance(expr, KnownDim):
            return IntVal(expr.value)
        elif isinstance(expr, SymDim):  # Dim should actually be the interface, SymbolicDim should inherit, else this is too fragile
            return Int(expr.name)
        elif isinstance(expr, BinaryDim):
            lhs = self.to_z3(expr.left)
            rhs = self.to_z3(expr.right)
            op = expr.operator
            if op == "+":
                return lhs + rhs
            elif op == "-":
                return lhs - rhs
            elif op == "*":
                return lhs * rhs
            elif op == "/":
                return lhs / rhs
        raise TypeError(f"Unsupported expr: {expr}")
    
    def get_var(self, name):
        return Int(name)

    def solve(self, semantic_ir) -> bool:
        for node in semantic_ir:

            if isinstance(node, DimDecl) and node.value is not None:
                z3_expr = self.to_z3(node.value)
                var = self.get_var(node.name)
                self.solver.add(var == z3_expr)
                self.solver.add(var > 0)

            # type hint shapes and runtime shapes must align via z3
            if isinstance(node, TensorDecl) and isinstance(node.value, TensorLiteralExpr):  # declared tensor with elements, ex: torch.tensor([[...]])
                shape_val = node.value.shape
                rows_val = self.to_z3(KnownDim(shape_val[0]))  # operate on the assumption that shape must contain int, this always holds true
                cols_val = self.to_z3(KnownDim(shape_val[1]))
                rows_hint = self.to_z3(node.shape[0])
                cols_hint = self.to_z3(node.shape[1])
                self.solver.add(rows_hint == rows_val)
                self.solver.add(cols_hint == cols_val)

            if isinstance(node, TensorDecl) and isinstance(node.value, MatMulExpr):
                ltype = self.env.lookup(node.value.left)
                rtype = self.env.lookup(node.value.right)
                self.solver.add(  # will dispatch this back to Rules eventually, fow now it's fine, this works
                    self.to_z3(ltype.cols) == self.to_z3(rtype.rows)
                )
                self.solver.add(
                    self.to_z3(node.shape[0]) == self.to_z3(ltype.rows)
                )
                self.solver.add(
                    self.to_z3(node.shape[1]) == self.to_z3(rtype.cols)
                )

            if isinstance(node, TensorDecl) and isinstance(node.value, RandnExpr):
                shape_val = node.value.shape
                rows_val = self.to_z3(shape_val[0])
                cols_val = self.to_z3(shape_val[1])
                rows_hint = self.to_z3(node.shape[0])
                cols_hint = self.to_z3(node.shape[1])
                self.solver.add(rows_hint == rows_val)
                self.solver.add(cols_hint == cols_val)

        return self.solver.check() == sat