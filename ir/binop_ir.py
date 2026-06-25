from .ir_node import IRNode
from common.span import SourceSpan
from common.operators import Operator


class BinOpIR(IRNode):
    def __init__(self, left: IRNode, right: IRNode, op: Operator, span=None):
        super().__init__(span=span)
        self.op = op
        self.left = left
        self.right = right
        self.span = span
