from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR


@dataclass
class ConstantIR(ExprIR):
    value: ExprIR
    # not including "kind" field because it seems like legacy metadata

    def to_proto(self):
        return _pb2.ExprIR(
            constant=self.to_proto()
        )


@dataclass
class BooleanIR(ConstantIR):
    value: bool
    span: SourceSpan

    def __repr__(self):
        return "true" if self.value is True else "false"

    def to_proto(self):
        return _pb2.ExprIR(
            constant=_pb2.ConstantIR(
                bool_lit=_pb2.BooleanIR(
                    value=self.value,
                    span=self.span.to_proto(),
                )
            )
        )


@dataclass
class BytesIR(ConstantIR):
    value: bytes
    span: SourceSpan


@dataclass
class ComplexIR(ConstantIR):
    real: float
    imag: float
    span: SourceSpan | None

    def to_proto(self):
        return _pb2.ExprIR(ComplexIR(real=self.real, imag=self.imag))


@dataclass
class EllipsisIR(ConstantIR):
    value: ellipsis
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(
            constant=_pb2.ConstantIR(
                ellipsis_lit=_pb2.EllipsisIR(
                    span=self.span.to_proto(),
                )
            )
        )


@dataclass
class IntegerIR(ConstantIR):
    value: int
    span: SourceSpan

    def __repr__(self):
        return str(self.value)

    def to_proto(self):
        return _pb2.ExprIR(
            constant=_pb2.ConstantIR(
                integer_lit=_pb2.IntegerIR(
                    value=self.value,
                    span=self.span.to_proto(),
                )
            )
        )


@dataclass
class NoneIR(ConstantIR):
    value: None
    span: SourceSpan | None

    def to_proto(self):
        return _pb2.ExprIR(
            constant=_pb2.ConstantIR(
                none_lit=_pb2.NoneIR(
                    span=self.span.to_proto(),
                )
            )
        )


@dataclass
class FloatIR(ConstantIR):
    value: float
    span: SourceSpan

    def __repr__(self):
        return str(self.value)

    def to_proto(self):
        return _pb2.ExprIR(
            constant=_pb2.ConstantIR(
                float_lit=_pb2.FloatIR(
                    value=self.value,
                    span=self.span.to_proto(),
                )
            )
        )


@dataclass
class StringIR(ConstantIR):
    value: str
    span: SourceSpan

    def __repr__(self):
        return self.value

    def to_proto(self):
        return _pb2.ExprIR(
            constant=_pb2.ConstantIR(
                string_lit=_pb2.StringIR(
                    value=self.value,
                    span=self.span.to_proto(),
                )
            )
        )
