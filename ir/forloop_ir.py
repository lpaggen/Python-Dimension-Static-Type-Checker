from common.span import SourceSpan
from .identified_ir_node import IdentifiedIRNode
from .ir_node import IRNode
from typing import List


class ForLoopIR(IRNode):
    def __init__(self, target: IRNode, iter: int, scope_id: int, body_scope_id: int, body: List[IRNode], orelse: List[IRNode], span=None):
        super().__init__(span)
        self.target = target
        self.iter = iter
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.body = body
        self.orelse = orelse
        self.span = span
