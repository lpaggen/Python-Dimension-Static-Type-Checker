from .ir_node import IRNode
from common.span import SourceSpan
from generated import _pb2


class BooleanIR(IRNode):
    def __init__(self, value: bool, span: SourceSpan = None):
        super().__init__(span=span)
        self.span = span
        self.value = value

    def __repr__(self):
        return "true" if self.value is True else "false"

    def to_proto(self):
        return _pb2.ExprIR(
            bool_lit=_pb2.BooleanIR(value=self.value)
        )
