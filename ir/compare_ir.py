from .stmt_ir import StmtIR
from common.span import SourceSpan
from .expr_ir import ExprIR
from common.operators import Operator


class CompareIR(ExprIR):
    def __init__(self, left: ExprIR, ops: list[str], comparators: list[ExprIR], span: SourceSpan):
        super().__init__(span=span)
        self.left=left
        self.ops=ops
        self.comparators=comparators
        self.span=span
