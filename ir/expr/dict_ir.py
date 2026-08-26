from dataclasses import dataclass
from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR

@dataclass
class DictIR(ExprIR):
    keys: list[ExprIR | None]
    values: list[ExprIR]
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.DictIR(
            keys=[_pb2.DictKeyIR(value=key.to_proto()) if key is not None else _pb2.DictKeyIR() for key in self.keys],
            values=[value.to_proto() for value in self.values],
            span=self.span.to_proto(),
        )
        return _pb2.ExprIR(dict=proto)
