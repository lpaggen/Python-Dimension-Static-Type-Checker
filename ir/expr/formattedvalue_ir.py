from dataclasses import dataclass
from common.span import SourceSpan
from generated import _pb2
from ir.expr.conversion_ir import Conversion
from ir.expr.expr_ir import ExprIR
from ir.expr.joinedstr_ir import JoinedStrIR

@dataclass
class FormattedValueIR(ExprIR):
    value: ExprIR
    conversion: Conversion
    format_spec: JoinedStrIR | None
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(formatted_value=_pb2.FormattedValueIR(
            value=self.value.to_proto(), conversion=int(self.conversion),
            format_spec=self.format_spec.to_proto().joined_str if self.format_spec is not None else None,
            span=self.span.to_proto(),
        ))
