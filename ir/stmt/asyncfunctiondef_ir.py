from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from common.typeparam_ir import TypeParamIR
from ir.arg.arg_ir import ArgIR
from ir.stmt.stmt_ir import StmtIR


@dataclass
class AsyncFunctionDefIR(StmtIR):
    name: str
    args: list[ArgIR]
    body: list[StmtIR]
    decorator_list: list[ExprIR]
    returns: ExprIR | None
    type_comment: str | None
    type_params: list[TypeParamIR]
    scope_id: int
    span: SourceSpan

    def to_proto(self):
        return _pb2.StmtIR(
            async_function_def=_pb2.AsyncFunctionDefIR(
                name=self.name,
                args=[arg.to_proto() for arg in self.args],
                body=[stmt.to_proto() for stmt in self.body],
                decorator_list=[d.to_proto() for d in self.decorator_list],
                returns=self.returns.to_proto() if self.returns is not None else None,
                type_comment=self.type_comment,
                type_params=[param.to_proto() for param in self.type_params],
                scope_id=self.scope_id,
                span=self.span.to_proto(),
            )
        )
