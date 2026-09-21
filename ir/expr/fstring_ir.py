from enum import IntEnum
from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from dataclasses import dataclass


class Conversion(IntEnum):
    NONE = -1
    STR = ord("s")  # 115
    REPR = ord("r")  # 114
    ASCII = ord("a")  # 97


@dataclass
class FormattedValueIR(ExprIR):
    value: ExprIR
    conversion: Conversion
    format_spec: JoinedStrIR | None
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(
            formatted_value=_pb2.FormattedValueIR(
                value=self.value.to_proto(),
                conversion=int(self.conversion),
                format_spec=(
                    self.format_spec.to_proto().joined_str
                    if self.format_spec is not None
                    else None
                ),
                span=self.span.to_proto(),
            )
        )


@dataclass
class JoinedStrIR(ExprIR):
    values: list[ExprIR]  # should hold Constant or FormattedValue, but both are ExprIR 
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(
            joined_str=_pb2.JoinedStrIR(
                values=[value.to_proto() for value in self.values],
                span=self.span.to_proto(),
            )
        )


@dataclass
class TemplateStrIR(ExprIR):
    value: ExprIR
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(
            template_str=_pb2.TemplateStrIR(
                values=[value.to_proto() for value in self.values],
                span=self.span.to_proto(),
            )
        )


@dataclass
class InterpolationIR(ExprIR):
    value: ExprIR
    source: str
    conversion: Conversion
    format_spec: ExprIR | None
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(
            interpolation=_pb2.InterpolationIR(
                value=self.value.to_proto(),
                source=self.source,
                conversion=int(self.conversion),
                format_spec=(
                    self.format_spec.to_proto()
                    if self.format_spec is not None
                    else None
                ),
                span=self.span.to_proto(),
            )
        )
