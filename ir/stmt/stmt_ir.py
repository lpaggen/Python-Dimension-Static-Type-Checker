from ir.ir_node import IRNode
from common.span import SourceSpan


class StmtIR(IRNode):
    def __init__(self, span: SourceSpan):
        super().__init__(span=span)
        self.span = span


def stmt_to_proto(stmt):
    """Serialize a node in statement position.

    Bindings, functions, and classes also have declaration-table encodings,
    so they expose a distinct statement serializer.
    """
    to_stmt_proto = getattr(stmt, "to_stmt_proto", None)
    if to_stmt_proto is not None:
        return to_stmt_proto()
    return stmt.to_proto()
