from dataclasses import dataclass
from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from ir.stmt.stmt_ir import StmtIR

@dataclass
class RaiseIR(StmtIR):
    exc: ExprIR | None
    cause: ExprIR | None
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(raise_stmt=_pb2.RaiseIR(
            exc=self.exc.to_proto() if self.exc is not None else None,
            cause=self.cause.to_proto() if self.cause is not None else None,
            span=self.span.to_proto(),
        ))
