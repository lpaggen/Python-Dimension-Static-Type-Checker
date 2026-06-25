from .ir_node import IRNode
from common.operators import Operator
from .expr_ir import ExprIR
from common.span import SourceSpan


class IdentifiedIRNode(IRNode):
    """
    Parent class to IRNode objects which need unique identifiers
    """

    def __init__(self, id, span=None):
        super().__init__(span)
        self.span = span
        self.id = id
