from .ir_node import IRNode
from common.span import SourceSpan
from common.operators import Operator
from .bool_ir import BooleanIR


class BoolOpIR(IRNode):
    def __init__(
        self, left: BooleanIR, right: BooleanIR, op: Operator, span: SourceSpan = None
    ):
        super().__init__(span=span)
        self.span = span
        self.left = left
        self.right = right
        self.op = op
