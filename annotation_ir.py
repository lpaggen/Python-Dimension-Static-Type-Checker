import ast
from typing import List
from dimension import Dim
from span import SourceSpan


class AnnotationHeadIR:
    def __init__(self, root: str, attrs: list[str], scope_id: int, span: SourceSpan):
        self.root = root        # "torch"
        self.attrs = attrs      # ["Tensor"]
        self.scope_id = scope_id
        self.span = span


class AnnotationIR:
    def __init__(self, head: AnnotationHeadIR, args: List[Dim, Dim]):
        self.head=head
        self.args=args
