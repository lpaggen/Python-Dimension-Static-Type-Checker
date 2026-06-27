import ast
from typing import List
from common.span import SourceSpan
from ir.expr_ir import ExprIR


class AnnotationHeadIR:
    def __init__(self, root: str, attrs: list[str], scope_id: int, span: SourceSpan):
        self.root = root  # "torch"
        self.attrs = attrs  # ["Tensor"]
        self.scope_id = scope_id
        self.span = span


class AnnotationIR:
    def __init__(self, head: AnnotationHeadIR, args: List[ExprIR, ExprIR]):
        self.head = head
        self.args = args
