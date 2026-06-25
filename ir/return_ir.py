from .stmt_ir import StmtIR
from common.span import SourceSpan
from .expr_ir import ExprIR


class ReturnIR(StmtIR):
    def __init__(self, value: ExprIR, span: SourceSpan):
        super().__init__(span=span)
        self.span=span
        self.value=value
