from dataclasses import dataclass

from common.span import SourceSpan
from common.typeparam_ir import TypeParamIR
from generated import _pb2
from ir.expr.call_ir import KeywordIR
from .stmt_ir import StmtIR, stmt_to_proto
from ..expr.expr_ir import ExprIR
from .decl_ir import DeclIR


@dataclass
class ClassDefIR(DeclIR):
    id: int
    symbol_id: int
    scope_id: int        # parent scope
    body_scope_id: int   # class-local scope
    name: str
    keywords: list[KeywordIR]
    body: list[StmtIR]
    bases: list[ExprIR]  # base classes: Base, nn.Module, etc.
    decorator_list: list[ExprIR]
    type_params: list[TypeParamIR]
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.ClassDefIR(
            id=self.id,
            symbol_id=self.symbol_id,
            name=self.name,
            scope_id=self.scope_id,
            body_scope_id=self.body_scope_id,
        )

        proto.body.extend([stmt_to_proto(stmt) for stmt in self.body])
        proto.bases.extend([base.to_proto() for base in self.bases])
        proto.keywords.extend([keyword.to_proto() for keyword in self.keywords])
        proto.decorator_list.extend(
            [decorator.to_proto() for decorator in self.decorator_list]
        )
        proto.type_params.extend([param.to_proto() for param in self.type_params])

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.DeclIR()
        stmt.class_decl.CopyFrom(proto)
        return stmt

    def to_stmt_proto(self):
        decl = self.to_proto()
        stmt = _pb2.StmtIR()
        stmt.class_decl.CopyFrom(decl.class_decl)
        return stmt
