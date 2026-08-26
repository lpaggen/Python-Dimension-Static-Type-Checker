from dataclasses import dataclass
from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from ir.stmt.stmt_ir import StmtIR, stmt_to_proto

@dataclass
class WhileIR(StmtIR):
    test: ExprIR
    scope_id: int
    body_scope_id: int
    body: list[StmtIR]
    orelse: list[StmtIR]
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.WhileLoopIR(
            test=self.test.to_proto(), scope_id=self.scope_id, body_scope_id=self.body_scope_id,
            body=[stmt_to_proto(stmt) for stmt in self.body],
            orelse=[stmt_to_proto(stmt) for stmt in self.orelse],
            span=self.span.to_proto(),
        )
        return _pb2.StmtIR(while_loop=proto)
