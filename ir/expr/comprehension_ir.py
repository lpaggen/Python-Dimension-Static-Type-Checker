from common.span import SourceSpan
from generated import _pb2
from ir.expr.expr_ir import ExprIR
from ir.ir_node import IRNode
from dataclasses import dataclass


@dataclass
class CompIR(IRNode):
    target: ExprIR
    iter: ExprIR
    ifs: list[ExprIR]
    is_async: bool
    span: SourceSpan

    def to_proto(self):
        return _pb2.CompIR(
            target=self.target.to_proto(),
            iter=self.iter.to_proto(),
            ifs=[cond.to_proto() for cond in self.ifs],
            is_async=self.is_async,
            span=self.span.to_proto(),
        )


@dataclass
class ListCompIR(ExprIR):
    elt: ExprIR
    generators: list[CompIR]
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(
            list_comp=_pb2.ListCompIR(
                elt=self.elt.to_proto(),
                generators=[gen.to_proto() for gen in self.generators],
                span=self.span.to_proto(),
            )
        )

@dataclass
class SetCompIR(ExprIR):
    elt: ExprIR
    generators: list[CompIR]
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(
            set_comp=_pb2.SetCompIR(
                elt=self.elt.to_proto(),
                generators=[gen.to_proto() for gen in self.generators],
                span=self.span.to_proto(),
            )
        )

@dataclass
class DictCompIR(ExprIR):
    key: ExprIR
    value: ExprIR
    generators: list[CompIR]
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(
            dict_comp=_pb2.DictCompIR(
                key=self.key.to_proto(),
                value=self.value.to_proto(),
                generators=[gen.to_proto() for gen in self.generators],
                span=self.span.to_proto(),
            )
        )

@dataclass
class GeneratorExpIR(ExprIR):
    elt: ExprIR
    generators: list[CompIR]
    span: SourceSpan

    def to_proto(self):
        return _pb2.ExprIR(
            generator_expr=_pb2.GeneratorExprIR(
                elt=self.elt.to_proto(),
                generators=[gen.to_proto() for gen in self.generators],
                span=self.span.to_proto(),
            )
        )
