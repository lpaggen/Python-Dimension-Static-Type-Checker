from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from ir.stmt.stmt_ir import StmtIR


@dataclass
class AssertIR(StmtIR):
    test: ExprIR
    msg: ExprIR | None
    span: SourceSpan | None

    def to_proto(self):
        return _pb2.StmtIR(
            assert_stmt=_pb2.AssertIR(
                test=self.test.to_proto(),
                msg=self.test.to_proto() if self.msg is not None else None,
                span=self.span.to_proto()
            )
        )
