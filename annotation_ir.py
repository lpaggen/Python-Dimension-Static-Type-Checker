import ast
from typing import List
from dimension_ir import DimIR
from span import SourceSpan


class AnnotationHeadIR:
    def __init__(self, root: str, attrs: list[str], scope_id: int, span: SourceSpan):
        self.root = root        # "torch"
        self.attrs = attrs      # ["Tensor"]
        self.scope_id = scope_id
        self.span = span


class AnnotationIR:
    def __init__(self, head: AnnotationHeadIR, args: List[DimIR, DimIR]):
        self.head=head
        self.args=args
