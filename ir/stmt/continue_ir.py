from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.stmt.stmt_ir import StmtIR


@dataclass
class ContinueIR(StmtIR):
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            continue_stmt=_pb2.ContinueIR(
                span=self.span.to_proto(),
            )
        )
