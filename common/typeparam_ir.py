from dataclasses import dataclass

from common.span import SourceSpan
from ir.expr.expr_ir import ExprIR
from ir.ir_node import IRNode
from generated import _pb2


class TypeParamIR(IRNode):
    pass


@dataclass
class TypeVarIR(TypeParamIR):
    name: str
    bound: ExprIR | None
    default_value: ExprIR | None
    span: SourceSpan

    def to_proto(self):
        return _pb2.TypeParamIR(
            type_var=_pb2.TypeVarIR(
                name=self.name,
                bound=self.bound.to_proto() if self.bound is not None else None,
                default_value=(
                    self.default_value.to_proto()
                    if self.default_value is not None
                    else None
                ),
                span=self.span.to_proto(),
            )
        )


@dataclass
class ParamSpecIR(TypeParamIR):
    name: str
    default_value: ExprIR | None
    span: SourceSpan

    def to_proto(self):
        return _pb2.TypeParamIR(
            param_spec=_pb2.ParamSpecIR(
                name=self.name,
                default_value=(
                    self.default_value.to_proto()
                    if self.default_value is not None
                    else None
                ),
                span=self.span.to_proto(),
            )
        )


@dataclass
class TypeVarTupleIR(TypeParamIR):
    name: str
    default_value: ExprIR | None
    span: SourceSpan

    def to_proto(self):
        return _pb2.TypeParamIR(
            type_var_tuple=_pb2.TypeVarTupleIR(
                name=self.name,
                default_value=(
                    self.default_value.to_proto()
                    if self.default_value is not None
                    else None
                ),
                span=self.span.to_proto(),
            )
        )
