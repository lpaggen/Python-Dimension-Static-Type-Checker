from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from ir.ir_node import IRNode
from ir.stmt.stmt_ir import StmtIR, stmt_to_proto


class WhileLoopIR(StmtIR):
    def __init__(
        self,
        test: ExprIR,
        scope_id: int,
        body_scope_id: int,
        body: list[StmtIR],
        orelse: list[StmtIR],
        span: SourceSpan,
    ):
        super().__init__(span)
        self.test = test
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.body = body
        self.orelse = orelse
        self.span = span

    def to_proto(self):
        proto = _pb2.WhileLoopIR(
            scope_id=self.scope_id,
            body_scope_id=self.body_scope_id,
        )

        proto.test.CopyFrom(self.test.to_proto())
        proto.body.extend([stmt_to_proto(stmt) for stmt in self.body])
        proto.orelse.extend([stmt_to_proto(stmt) for stmt in self.orelse])

        proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.while_loop.CopyFrom(proto)
        return stmt


@dataclass
class AsyncForIR(StmtIR):
    target: ExprIR
    iter: ExprIR
    body: list[StmtIR]
    orelse: list[StmtIR]
    type_comment: str | None
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            async_for=_pb2.AsyncForIR(
                target=self.target.to_proto(),
                iter=self.iter.to_proto(),
                body=[stmt.to_proto() for stmt in self.body],
                orelse=[stmt.to_proto() for stmt in self.orelse],
                type_comment=self.type_comment,
                span=self.span.to_proto(),
            )
        )


class ForLoopIR(StmtIR):
    def __init__(
        self,
        target: IRNode,
        iter: int,
        scope_id: int,
        body_scope_id: int,
        body: list[IRNode],
        orelse: list[IRNode],
        span: SourceSpan,
    ):
        super().__init__(span)
        self.target = target
        self.iter = iter
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.body = body
        self.orelse = orelse
        self.span = span

    def to_proto(self):
        proto = _pb2.ForLoopIR(
            scope_id=self.scope_id,
            body_scope_id=self.body_scope_id,
        )

        proto.target.CopyFrom(self.target.to_proto())
        proto.iter.CopyFrom(self.iter.to_proto())
        proto.body.extend([stmt_to_proto(stmt) for stmt in self.body])
        proto.orelse.extend([stmt_to_proto(stmt) for stmt in self.orelse])

        proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.for_loop.CopyFrom(proto)
        return stmt
