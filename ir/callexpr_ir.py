from .expr_ir import ExprIR
from common.span import SourceSpan



class KeywordArgIR(ExprIR):
    def __init__(self, name: str, value: ExprIR, span: SourceSpan=None):
        super().__init__(span=span, value=value)
        name=name
        value=value
        span=span


class CallExprIR(ExprIR):
    def __init__(self, callee, args: list[ExprIR], kwargs: list[KeywordArgIR], span=None):
        super().__init__(span=span, value=callee)
        self.span = span
        self.callee = callee
        self.args = args
        self.kwargs=kwargs
