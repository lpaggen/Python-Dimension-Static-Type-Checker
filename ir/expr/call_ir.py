from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR


@dataclass
class KeywordIR(ExprIR):
    arg: str | None
    value: ExprIR
    span: SourceSpan | None = None

    def to_proto(self):
        proto = _pb2.KeywordArgIR()
        if self.arg is not None:
            proto.arg = self.arg
        proto.value.CopyFrom(self.value.to_proto())
        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())
        return proto


@dataclass
class CallIR(ExprIR):
    func: ExprIR
    args: list[ExprIR]
    keywords: list[KeywordIR]
    span: SourceSpan | None = None

    def to_proto(self):
        return _pb2.ExprIR(
            call=_pb2.CallExprIR(
                func=self.func.to_proto(),
                args=[arg.to_proto() for arg in self.args],
                keywords=[keyword.to_proto() for keyword in self.keywords],
            )
        )
