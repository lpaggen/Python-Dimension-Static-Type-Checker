from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.annotation.annotation_ir import AnnotationIR
from ir.expr.expr_ir import ExprIR
from ir.stmt.stmt_ir import StmtIR

@dataclass
class AnnAssignIR(StmtIR):
    target: ExprIR
    annotation: AnnotationIR
    value: ExprIR | None
    simple: int
    span: SourceSpan | None

    def to_proto(self):
        proto = _pb2.AnnAssignIR(
            target=self.target.to_proto(),
            annotation=self.annotation.to_proto(),
            simple=self.simple
        )

        if self.value is not None:
            proto.value.CopyFrom(self.value.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        return _pb2.StmtIR(annassign=proto)
