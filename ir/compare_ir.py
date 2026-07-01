from .stmt_ir import StmtIR
from common.span import SourceSpan
from .expr_ir import ExprIR
from common.operators import Operator
from generated import _pb2



class CompareIR(ExprIR):
    def __init__(
        self, left: ExprIR, ops: list[Operator], comparators: list[ExprIR], span: SourceSpan
    ):
        super().__init__(span=span, value=None)
        self.left = left
        self.ops = ops
        self.comparators = comparators
        self.span = span

    def to_proto(self):
        compare = _pb2.CompareIR()

        compare.left.CopyFrom(self.left.to_proto())

        compare.ops.extend([
            op.value if isinstance(op, Operator) else op
            for op in self.ops
        ])

        compare.comparators.extend([
            comparator.to_proto()
            for comparator in self.comparators
        ])

        if self.span is not None:
            compare.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.compare.CopyFrom(compare)
        return expr
