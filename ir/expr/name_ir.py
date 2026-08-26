from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR


@dataclass(repr=False)
class NameIR(ExprIR):
    id: str
    use_scope_id: int
    span: SourceSpan

    def __repr__(self):
        return self.id

    def to_proto(self):
        proto = _pb2.IdentifierIR(id=self.id, use_scope_id=self.use_scope_id)
        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())
        return _pb2.ExprIR(identifier=proto)
