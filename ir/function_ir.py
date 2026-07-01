from common.span import SourceSpan
from .annotation_ir import AnnotationIR
from .ir_node import IRNode
from generated import _pb2
from .expr_ir import ExprIR
from .stmt_ir import StmtIR
from .decl_ir import DeclIR


class ParamIR(IRNode):
    def __init__(
        self,
        symbol_id: int,
        name: str,
        annotation: AnnotationIR,
        default,
        span: SourceSpan,
    ):
        self.symbol_id = symbol_id
        self.name = name
        self.annotation = annotation
        self.default = default
        self.span = span

    def to_proto(self):
        proto = _pb2.ParamIR(
            symbol_id=self.symbol_id,
            name=self.name,
        )

        if self.annotation is not None:
            proto.annotation.CopyFrom(self.annotation.to_proto())

        if self.default is not None:
            proto.default_value.CopyFrom(self.default.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        return proto

class ReturnIR(StmtIR):
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


class FunctionIR(DeclIR):
    def __init__(
        self,
        id: int,  # symbol id -> name of function
        symbol_id: int,
        name: str,
        scope_id: int,  # parent scope where function name is bound
        body_scope_id: int,  # function-local scope
        params: list[ParamIR],
        body: list[IRNode],
        returns,
        decorators,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.id = id
        self.symbol_id = symbol_id
        self.name = name
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.params = params
        self.body = body
        self.returns = returns
        self.decorators = decorators
        self.span = span

    def to_proto(self):
        proto = _pb2.FunctionIR(
            id=self.id,
            symbol_id=self.symbol_id,
            name=self.name,
            scope_id=self.scope_id,
            body_scope_id=self.body_scope_id,
        )

        proto.params.extend([p.to_proto() for p in self.params])
        proto.body.extend([stmt.to_proto() for stmt in self.body])
        proto.decorators.extend([d.to_proto() for d in self.decorators])

        if self.returns is not None:
            proto.returns.CopyFrom(self.returns.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.DeclIR()
        stmt.function.CopyFrom(proto)
        return stmt