from dataclasses import dataclass
from common.span import SourceSpan
from generated import _pb2
from ir.stmt.stmt_ir import StmtIR, stmt_to_proto
from ir.stmt.with_ir import WithItemIR

@dataclass
class AsyncWithIR(StmtIR):
    items: list[WithItemIR]
    body: list[StmtIR]
    type_comment: str | None
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.AsyncWithIR(
            items=[item.to_proto() for item in self.items],
            body=[stmt_to_proto(stmt) for stmt in self.body],
            span=self.span.to_proto(),
        )
        if self.type_comment is not None:
            proto.type_comment = self.type_comment
        return _pb2.StmtIR(async_with=proto)
