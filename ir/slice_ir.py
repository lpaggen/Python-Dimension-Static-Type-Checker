from .ir_node import IRNode
from common.span import SourceSpan


class SliceIR(IRNode):
    def __init__(self, lower: IRNode, upper: IRNode, step: IRNode, span: SourceSpan):
        super().__init__(span=span)
        self.lower = lower
        self.upper = upper
        self.step = span
        self.span=span
