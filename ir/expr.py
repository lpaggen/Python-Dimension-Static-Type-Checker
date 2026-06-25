from ir_node import IRNode
from common.span import SourceSpan


class ExprIR(IRNode):
    def __init__(self, span: SourceSpan):
        super().__init__(span=span)
