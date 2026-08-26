from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR


@dataclass
class YieldIR(ExprIR):
    value: ExprIR | None
    span: SourceSpan | None

    def to_proto(self):
        return _pb2.ExprIR(
            yield_expr=_pb2.YieldIR(
                value=self.value.to_proto() if self.value is not None else None,
                span=self.span.to_proto(),
            )
        )
