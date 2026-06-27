from common.span import SourceSpan
from .ir_node import IRNode
from .expr_ir import ExprIR
from generated import _pb2


class TupleIR(ExprIR):
    def __init__(self, elements: tuple[ExprIR], span: SourceSpan=None):
        super().__init__(span=span, value=None)
        self.span = span
        self.elements = elements

    def __repr__(self):
        return "TupleIR<" + str(self.elements) + ">"

    def to_proto(self):
        tuple_proto = _pb2.TupleIR()

        tuple_proto.elements.extend([
            element.to_proto() for element in self.elements
        ])

        if self.span is not None:
            tuple_proto.span.CopyFrom(self.span.to_proto())

        expr = _pb2.ExprIR()
        expr.tuple.CopyFrom(tuple_proto)
        return expr
