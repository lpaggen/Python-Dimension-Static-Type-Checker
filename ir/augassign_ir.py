from .identified_ir_node import IdentifiedIRNode, IRNode
from common.span import SourceSpan
from common.operators import Operator


class AugAssignIR(IRNode):
    def __int__(
        self, target: str, op: Operator, value=IRNode, span: SourceSpan = None
    ):
        super().__init__(span=span)
        self.span=span
        self.target=target
        self.op=op
        self.value=value
