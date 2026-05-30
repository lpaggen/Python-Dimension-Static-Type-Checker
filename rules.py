from custom_types import MatrixType, Type, ScalarType
from dataclasses import dataclass
from dimension import Dim


@dataclass
class DimConstraint:
    left: Dim
    right: Dim
    label: str

class Rules:
    def __init__(self):
        self.rules = {
            "matmul": MatmulRule,
            "add": AddRule
        }

    def apply(self, name, *args):
        rule = self.rules[name]
        return rule.apply(*args)

class MatmulRule:
    @staticmethod
    def apply(ltype: MatrixType, rtype: MatrixType):
        if isinstance(ltype, MatrixType) and isinstance(rtype, MatrixType):
            constraints = [
                DimConstraint(ltype.cols, rtype.rows, "inner_dims_match")
            ]
            out = MatrixType(rows=ltype.rows, cols=rtype.cols)
            return out, constraints
        raise TypeError("Add requires both arguments to be matrices: "
                f"{type(ltype).__name__} and {type(rtype).__name__}")

class AddRule:
    @staticmethod
    def apply(ltype, rtype):
        if isinstance(ltype, MatrixType) and isinstance(rtype, MatrixType):
            constraints = [
                DimConstraint(ltype.rows, rtype.rows, "rows_match"),
                DimConstraint(ltype.cols, rtype.cols, "cols_match")
            ]
            out = MatrixType(rows=ltype.rows, cols=ltype.cols)
            return out, constraints
        elif isinstance(ltype, ScalarType) or isinstance(rtype, ScalarType):  # supports scalar or tensor
            constraints = []  # none to add
            out = MatrixType(rows=ltype.rows, cols=ltype.cols) if isinstance(ltype, MatrixType) else MatrixType(rows=rtype.rows, cols=rtype.cols)
            return out, constraints
        raise TypeError("Add requires both arguments to be matrices or one to be a scalar, got: "
                f"{type(ltype).__name__} and {type(rtype).__name__}")
