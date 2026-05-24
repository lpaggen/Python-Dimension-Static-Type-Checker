from z3 import Int, sat, Solver, IntVal, AstRef, unsat, Bool
from dimension import KnownDim, Dim, BinaryDim, DimDecl, SymDim, UnknownDim, ScalarDim
from tensor_decl import TensorDecl
from rules import Rules
from custom_literals import *
from custom_types import *


class ConstraintSolver:
    def __init__(self, env):
        self.solver = Solver()
        self.errors = []
        self.rules = Rules()
        self.env = env
        self._claim_counter = 0
        self._labels = {}  # maps Bool name -> human readable message

    def _add(self, constraint, label: str):
        name = Bool(f"c_{self._claim_counter}")
        self._claim_counter += 1
        self._labels[str(name)] = label
        self.solver.assert_and_track(constraint, name)
        return name

    def _to_z3(self, expr: Dim) -> AstRef:
        match expr:
            case UnknownDim():
                raise TypeError("Cannot convert UnknownDim to Z3 — dimension was never resolved")
            case ScalarDim(value=v):
                return IntVal(v)
            case KnownDim(name=n, value=v):
                return IntVal(v)
            case SymDim(name=n):
                return Int(n)
            case BinaryDim(left=l, right=r, operator=op):
                lhs, rhs = self.to_z3(l), self.to_z3(r)
                match op:
                    case "+": return lhs + rhs
                    case "-": return lhs - rhs
                    case "*": return lhs * rhs
                    case "/": return lhs / rhs
                    case _: raise TypeError(f"Unknown operator: {op}")
            case _:
                raise TypeError(f"Unsupported dim expr: {expr}")

    def _get_var(self, name):
        return Int(name)

    def _resolve_type(self, expr) -> MatrixType | Scalar:
        match expr:
            case ScalarDim(value=v):
                return Scalar(v)
            case SymDim(name=n):
                return self.env.lookup(n)
            case _:
                return self.env.lookup(str(expr))

    def _process(self, semantic_ir):
        for node in semantic_ir:
            match node:
                case DimDecl():
                    self._handle_dim_decl(node)
                case TensorDecl(value=TensorLiteralExpr()):
                    self._handle_tensor_literal(node)
                case TensorDecl(value=MatMulExpr()):
                    self._handle_matmul(node)
                case TensorDecl(value=AddExpr()):
                    self._handle_add(node)
                case TensorDecl(value=RandnExpr()):
                    self._handle_randn(node)

    def _handle_dim_decl(self, node: DimDecl):
        var = self._get_var(node.name)
        if node.value is not None:
            self._add(var == self._to_z3(node.value), f"dim_decl_{node.name}")
        self._add(var > 0, f"dim_positive_{node.name}")

    def _handle_tensor_literal(self, node: TensorDecl):
        found_rows = IntVal(node.value.shape[0])
        found_cols = IntVal(node.value.shape[1])
        if isinstance(node.shape[0], UnknownDim):
            node.shape = (KnownDim(node.value.shape[0]), KnownDim(node.value.shape[1]))
        else:
            self._add(
                self._to_z3(node.shape[0]) == found_rows,
                f"tensor_literal_{node.name}_hint_rows"
            )
            self._add(
                self._to_z3(node.shape[1]) == found_cols,
                f"tensor_literal_{node.name}_hint_cols"
            )

    def _handle_matmul(self, node: TensorDecl):
        ltype = self._resolve_type(node.value.left)
        rtype = self._resolve_type(node.value.right)
        out_type, constraints = self.rules.apply("matmul", ltype, rtype)
        for c in constraints:
            self._add(self._to_z3(c.left) == self._to_z3(c.right), f"matmul_{node.name}")
        if isinstance(node.shape[0], UnknownDim):
            node.shape = (out_type.rows, out_type.cols)
        else:
            self._add(
                self.to_z3(node.shape[0]) == self.to_z3(out_type.rows),
                f"matmul_{node.name}_hint_rows"
            )
            self._add(
                self.to_z3(node.shape[1]) == self.to_z3(out_type.cols),
                f"matmul_{node.name}_hint_cols"
            )

    def _handle_add(self, node: TensorDecl):
        ltype = self._resolve_type(node.value.left)
        rtype = self._resolve_type(node.value.right)
        out_type, constraints = self.rules.apply("add", ltype, rtype)
        for c in constraints:
            self._add(self._to_z3(c.left) == self._to_z3(c.right), f"add_{node.name}")

    def _handle_randn(self, node: TensorDecl):
        self._add(self._to_z3(node.shape[0]) == self._to_z3(node.value.shape[0]), f"randn_rows_{node.name}")
        self._add(self._to_z3(node.shape[1]) == self._to_z3(node.value.shape[1]), f"randn_cols_{node.name}")

    def solve(self, semantic_ir) -> bool:
        self._process(semantic_ir)
        if self.solver.check() == sat:
            print("OK: all dimension constraints satisfied")
            return True
        else:
            print("ERRORS: dimension constraints violated\n")
            for tracked in self.solver.unsat_core():
                label = self._labels.get(str(tracked), str(tracked))
                print(f"{label}")
            return False
