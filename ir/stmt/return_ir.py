from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from ir.stmt.stmt_ir import StmtIR
from dataclasses import dataclass


@dataclass
class ReturnIR(StmtIR):
    value: ExprIR | None
    span: SourceSpan
    def to_proto(self):
        proto = _pb2.ReturnIR()

        if self.value is not None:
            proto.value.CopyFrom(self.value.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.return_stmt.CopyFrom(proto)
        return stmt
