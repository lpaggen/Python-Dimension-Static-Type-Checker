from common.span import SourceSpan
from .expr_ir import ExprIR
from generated import _pb2


class ListIR(ExprIR):
    def __init__(self, elements: list[ExprIR], span: SourceSpan=None):
        super().__init__(span=span)
        self.span = span
        self.elements = elements

    def __repr__(self):
        return "ListIR<" + str(self.contents) + ">"

    def to_proto(self):
        proto = _pb2.ListIR()

        proto.elements.extend([
            i.to_proto() for i in self.elements
        ])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.list.CopyFrom(proto)
        return expr
