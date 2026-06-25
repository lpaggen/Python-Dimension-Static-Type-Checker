from expression_ir import IdentifiedIRNode, IRNode
from span import SourceSpan
from operators import Operator


class AugAssignIR(IRNode):
    def __int__(
        self, target: str, op: Operator, value=IRNode, span: SourceSpan = None
    ):
        super().__init__(span=span)
        self.span=span
        self.target=target
        self.op=op
        self.value=value
