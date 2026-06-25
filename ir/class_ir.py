from common.span import SourceSpan
from .identified_ir_node import IdentifiedIRNode
from generated import _pb2
from .stmt_ir import StmtIR
from .expr_ir import ExprIR


class ClassIR(IdentifiedIRNode):
    def __init__(
        self,
        id: int,
        symbol_id: int,
        name: str,
        scope_id: int,  # parent scope
        body_scope_id: int,  # class-local scope
        body: list[StmtIR],
        bases: list[ExprIR],  # Base classes: Base, nn.Module, etc.
        decorators: list,
        span: SourceSpan,
    ):
        super().__init__(id=id, span=span)
        self.id = id
        self.symbol_id = symbol_id
        self.name = name
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.body = body
        self.bases = bases
        self.decorators = decorators
        self.span = span

    def to_proto(self):
        proto = _pb2.ClassIR(
            id=self.id,
            symbol_id=self.symbol_id,
            name=self.name,
            scope_id=self.scope_id,
            body_scope_id=self.body_scope_id,
        )

        proto.body.extend([stmt.to_proto() for stmt in self.body])
        proto.bases.extend([base.to_proto() for base in self.bases])
        proto.decorators.extend([decorator.to_proto() for decorator in self.decorators])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.StmtIR()
        stmt.class_decl.CopyFrom(proto)
        return stmt
