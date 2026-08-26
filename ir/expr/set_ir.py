from dataclasses import dataclass
from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR

@dataclass
class SetIR(ExprIR):
    elts: list[ExprIR]
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.SetIR(elts=[elt.to_proto() for elt in self.elts])
        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())
        return _pb2.ExprIR(set=proto)
