from common.span import SourceSpan
from .ir_node import IRNode


class NoneIR(IRNode):
    def __init__(self, span: SourceSpan=None):
        super().__init__(span=span)
        self.span = span
