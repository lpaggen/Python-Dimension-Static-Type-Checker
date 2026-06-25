from .expr_ir import ExprIR
from common.span import SourceSpan
from generated import _pb2


class AttributeExprIR(ExprIR):
    def __init__(self, base: ExprIR, attr: str, span: SourceSpan):
        super().__init__(span=span, value=base)
        self.base = base
        self.attr = attr
        self.span = span

    def to_proto(self):
        attr_proto = _pb2.AttributeExprIR(
            attr=self.attr,
        )

        attr_proto.base.CopyFrom(self.base.to_proto())

        if self.span is not None:
            attr_proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.attribute.CopyFrom(attr_proto)
        return expr
