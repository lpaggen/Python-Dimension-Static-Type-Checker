from common.span import SourceSpan
from .ir_node import IRNode
from typing import List


class WhileLoopIR(IRNode):
    def __init__(self, test: IRNode, scope_id: int, body_scope_id: int, body: List[IRNode], orelse: List[IRNode], span: SourceSpan=None):
        super().__init__(span)
        self.test = test
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.body = body
        self.orelse = orelse
        self.span = span
