from dataclasses import dataclass

from common.span import SourceSpan
from common.typeparam_ir import TypeParamIR
from ir.annotation.annotation_ir import AnnotationIR
from ir.arg.arg_ir import ArgIR
from ir.expr.expr_ir import ExprIR
from generated import _pb2
from .stmt_ir import StmtIR, stmt_to_proto
from .decl_ir import DeclIR


@dataclass
class FunctionDefIR(DeclIR):
    id: int             # symbol id -> name of function
    symbol_id: int
    scope_id: int       # parent scope where function name is bound
    body_scope_id: int  # function-local scope
    name: str
    args: list[ArgIR]
    body: list[StmtIR]
    decorator_list: list[ExprIR]
    returns: AnnotationIR | None
    type_comment: str | None
    type_params: list[TypeParamIR]
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.FunctionDefIR(
            id=self.id,
            symbol_id=self.symbol_id,
            name=self.name,
            scope_id=self.scope_id,
            body_scope_id=self.body_scope_id,
        )

        proto.args.extend([arg.to_proto() for arg in self.args])
        proto.body.extend([stmt_to_proto(stmt) for stmt in self.body])
        proto.decorator_list.extend([d.to_proto() for d in self.decorator_list])
        proto.type_params.extend([param.to_proto() for param in self.type_params])

        if self.returns is not None:
            proto.returns.CopyFrom(self.returns.to_proto())

        if self.type_comment is not None:
            proto.type_comment = self.type_comment

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.DeclIR()
        stmt.function.CopyFrom(proto)
        return stmt

    def to_stmt_proto(self):
        decl = self.to_proto()
        stmt = _pb2.StmtIR()
        stmt.function.CopyFrom(decl.function)
        return stmt
