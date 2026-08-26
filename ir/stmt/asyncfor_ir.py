from dataclasses import dataclass
from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from ir.stmt.stmt_ir import StmtIR, stmt_to_proto

@dataclass
class AsyncForIR(StmtIR):
    target: ExprIR
    iter: ExprIR
    body: list[StmtIR]
    orelse: list[StmtIR]
    type_comment: str | None
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.AsyncForIR(
            target=self.target.to_proto(), iter=self.iter.to_proto(),
            body=[stmt_to_proto(stmt) for stmt in self.body],
            orelse=[stmt_to_proto(stmt) for stmt in self.orelse],
            span=self.span.to_proto(),
        )
        if self.type_comment is not None:
            proto.type_comment = self.type_comment
        return _pb2.StmtIR(async_for=proto)
