from .stmt_ir import StmtIR
from common.span import SourceSpan
from .expr_ir import ExprIR
from common.operators import Operator


class UnaryOpIR(ExprIR):
    def __init__(self, op: Operator, value: ExprIR, span: SourceSpan):
        super().__init__(span=span)
        self.span=span
        self.value=value
        self.op=op
