from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from ir.arg.arg_ir import ArgIR
from dataclasses import dataclass


@dataclass
class LambdaIR(ExprIR):
    args: list[ArgIR]
    body: ExprIR
    scope_id: int
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(
            lambda_expr=_pb2.LambdaIR(
                args=[arg.to_proto() for arg in self.args],
                body=self.body.to_proto(),
                scope_id=self.scope_id,
                span=self.span.to_proto(),
            )
        )
