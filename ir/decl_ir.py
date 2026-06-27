from .ir_node import IRNode
from common.span import SourceSpan


class DeclIR(IRNode):
    def __init__(self, span: SourceSpan):
        super().__init__(span=span)
        self.span = span
