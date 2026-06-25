from common.span import SourceSpan
from .expr_ir import ExprIR
from .stmt_ir import StmtIR
from generated import _pb2


class ExprStmtIR(StmtIR):
    def __init__(self, value: ExprIR, span: SourceSpan):
        super().__init__(span=span)
        self.span = span
        self.value = value

    def to_proto(self):
        proto = _pb2.ReturnIR()

        if self.value is not None:
            proto.value.CopyFrom(self.value.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.return_stmt.CopyFrom(proto)
        return stmt