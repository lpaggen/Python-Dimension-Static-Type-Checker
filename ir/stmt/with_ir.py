from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from ir.ir_node import IRNode
from ir.stmt.stmt_ir import StmtIR


@dataclass
class WithIR(StmtIR):
    items: list[WithItemIR]
    body: list[StmtIR]
    type_comment: str | None
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            with_stmt=_pb2.WithIR(
                items=[item.to_proto() for item in self.items],
                body=[stmt.to_proto() for stmt in self.body],
                type_comment=self.type_comment,
                span=self.span.to_proto(),
            )
        )

@dataclass
class WithItemIR(IRNode):
    context_expr: ExprIR
    optional_vars: ExprIR | None

    def to_proto(self):
        return _pb2.WithItemIR(
            context_expr=self.context_expr.to_proto(),
            optional_vars=(
                self.optional_vars.to_proto()
                if self.optional_vars is not None
                else None
            ),
        )
