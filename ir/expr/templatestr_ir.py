from dataclasses import dataclass
from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR

@dataclass
class TemplateStrIR(ExprIR):
    values: list[ExprIR]
    span: SourceSpan | None

    def to_proto(self):
        return _pb2.ExprIR(template_str=_pb2.TemplateStrIR(
            values=[value.to_proto() for value in self.values],
            span=self.span.to_proto() if self.span is not None else None,
        ))
