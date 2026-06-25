from .expr_ir import ExprIR
from common.span import SourceSpan


class AttributeExprIR(ExprIR):
    def __init__(self, base: str, attr: str, span: SourceSpan):
        super().__init__(span=span, value=base)
        self.base = base
        self.attr = attr
        self.span = span
