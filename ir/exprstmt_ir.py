from common.span import SourceSpan
from .expr_ir import ExprIR
from .stmt_ir import StmtIR


class ExprStmtIR(StmtIR):
    def __init__(self, value: ExprIR, span: SourceSpan):
        super().__init__(span=span)
        self.span=span
        self.value=value
