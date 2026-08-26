from dataclasses import dataclass
from common.span import SourceSpan
from generated import _pb2
from ir.expr.conversion_ir import Conversion
from ir.expr.expr_ir import ExprIR

@dataclass
class InterpolationIR(ExprIR):
    value: ExprIR
    str: str | None
    conversion: Conversion
    format_spec: ExprIR | None
    span: SourceSpan | None

    def to_proto(self):
        return _pb2.ExprIR(interpolation=_pb2.InterpolationIR(
            value=self.value.to_proto(), str=self.str, conversion=int(self.conversion),
            format_spec=self.format_spec.to_proto() if self.format_spec is not None else None,
            span=self.span.to_proto() if self.span is not None else None,
        ))
