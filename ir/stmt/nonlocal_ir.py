from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.stmt.stmt_ir import StmtIR


@dataclass
class NonlocalIR(StmtIR):
    names: list[str]
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            nonlocal_stmt=_pb2.NonlocalIR(
                names=self.names,
                span=self.span.to_proto(),
            )
        )
