
from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from ir.stmt.stmt_ir import StmtIR


@dataclass
class DeleteIR(StmtIR):
    targets: list[ExprIR]
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            delete_stmt=_pb2.DeleteIR(
                targets=[target.to_proto() for target in self.targets],
                span=self.span.to_proto(),
            )
        )
