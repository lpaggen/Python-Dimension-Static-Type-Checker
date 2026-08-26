from dataclasses import dataclass
from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR

@dataclass
class JoinedStrIR(ExprIR):
    values: list[ExprIR]
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(joined_str=_pb2.JoinedStrIR(
            values=[value.to_proto() for value in self.values], span=self.span.to_proto()
        ))
