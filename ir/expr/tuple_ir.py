from dataclasses import dataclass
from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR

@dataclass
class TupleIR(ExprIR):
    elts: tuple[ExprIR, ...]
    span: SourceSpan | None = None

    def to_proto(self):
        proto = _pb2.TupleIR(elts=[elt.to_proto() for elt in self.elts])
        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())
        return _pb2.ExprIR(tuple=proto)
