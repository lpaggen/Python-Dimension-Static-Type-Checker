from .ir_node import IRNode
from common.span import SourceSpan


class SubscriptIR(IRNode):
    def __init__(self, target: IRNode, subscript: IRNode, span: SourceSpan):
        super().__init__(span)
        self.target = target
        self.subscript = subscript
        self.span = span
