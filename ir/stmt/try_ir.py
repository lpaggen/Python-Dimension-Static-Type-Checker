from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from ir.ir_node import IRNode
from ir.stmt.stmt_ir import StmtIR


@dataclass
class TryIR(StmtIR):
    body: list[StmtIR]
    handlers: list[ExceptHandlerIR]
    orelse: list[StmtIR]
    finalbody: list[StmtIR]
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            try_stmt=_pb2.TryIR(
                body=[stmt.to_proto() for stmt in self.body],
                handlers=[handler.to_proto() for handler in self.handlers],
                orelse=[stmt.to_proto() for stmt in self.orelse],
                finalbody=[stmt.to_proto() for stmt in self.finalbody],
                span=self.span.to_proto(),
            )
        )

@dataclass
class ExceptHandlerIR(IRNode):
    type: ExprIR | None
    name: str | None
    body: list[StmtIR]
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExceptHandlerIR(
            type=self.type.to_proto() if self.type is not None else None,
            name=self.name,
            body=[stmt.to_proto() for stmt in self.body],
            span=self.span.to_proto(),
        )
