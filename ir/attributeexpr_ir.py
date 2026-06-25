from .ir_node import IRNode
from common.span import SourceSpan


class AttributeExprIR(IRNode):
    def __init__(self, base: IRNode, attr: str, span: SourceSpan):
        super().__init__(span=span)
        self.base = base
        self.attr = attr
        self.span = span
