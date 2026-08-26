from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR


@dataclass
class AwaitIR(ExprIR):
    value: ExprIR
    span: SourceSpan | None

    def to_proto(self):
        return _pb2.ExprIR(
            await_expr=_pb2.AwaitIR(
                value=self.value.to_proto(),
                span=self.span.to_proto(),
            )
        )
