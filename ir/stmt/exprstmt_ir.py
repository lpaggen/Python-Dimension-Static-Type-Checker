from common.span import SourceSpan
from ir.expr.expr_ir import ExprIR
from .stmt_ir import StmtIR
from generated import _pb2
from dataclasses import dataclass


@dataclass
class ExprStmtIR(StmtIR):
    value: ExprIR
    span: SourceSpan
    def to_proto(self):
        proto = _pb2.ExprStmtIR()

        if self.value is not None:
            proto.value.CopyFrom(self.value.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.expr_stmt.CopyFrom(proto)
        return stmt
