from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.stmt.stmt_ir import StmtIR


@dataclass
class PassIR(StmtIR):
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            pass_stmt=_pb2.PassIR(
                span=self.span.to_proto(),
            )
        )
