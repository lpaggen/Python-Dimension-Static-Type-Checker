from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR


@dataclass
class YieldFromIR(ExprIR):
    value: ExprIR
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(
            yield_from=_pb2.YieldFromIR(
                value=self.value.to_proto(),
                span=self.span.to_proto(),
            )
        )
