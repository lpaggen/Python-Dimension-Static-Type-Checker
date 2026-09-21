from .expr_ir import ExprIR
from common.span import SourceSpan
from generated import _pb2


class KeywordArgIR(ExprIR):
    def __init__(self, name: str | None, value: ExprIR, span: SourceSpan):
        super().__init__(span=span, value=value)  # TODO check if value should be IdentifierIR or not
        self.name = name
        self.value = value
        self.span = span

    def to_proto(self):
        proto = _pb2.KeywordArgIR(
            name=self.name if self.name is not None else "",
        )
        proto.value.CopyFrom(self.value.to_proto())
        proto.span.CopyFrom(self.span.to_proto())
        return proto


class CallExprIR(ExprIR):
    def __init__(
        self, callee, args: list[ExprIR], kwargs: list[KeywordArgIR], span: SourceSpan
    ):
        super().__init__(span=span, value=callee)
        self.span = span
        self.callee = callee
        self.args = args
        self.kwargs = kwargs

    def to_proto(self):
        return _pb2.ExprIR(
            call=_pb2.CallExprIR(
                callee=self.callee.to_proto(),
                args=[arg.to_proto() for arg in self.args],
                kwargs=[kw.to_proto() for kw in self.kwargs],
            )
        )
