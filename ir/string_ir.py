from .expr_ir import ExprIR
from common.span import SourceSpan


class StringIR(ExprIR):
    def __init__(self, value: str, span: SourceSpan=None):
        super().__init__(span=span, value=value)
        self.span = span
        self.value = value

    def __repr__(self):
        return self.value
