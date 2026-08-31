from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.annotation.annotation_ir import AnnotationIR
from ir.expr.expr_ir import ExprIR
from ir.stmt.stmt_ir import StmtIR

@dataclass
class AssignIR(StmtIR):
    target: list[ExprIR]
    value: ExprIR | None
    type_comment: str
    span: SourceSpan | None

    def to_proto(self):
        proto = _pb2.AssignIR(
            target=[target.to_proto() for target in self.target],
            type_comment=self.type_comment
        )

        if self.value is not None:
            proto.value.CopyFrom(self.value.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        return _pb2.StmtIR(assign=proto)
