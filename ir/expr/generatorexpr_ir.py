# from dataclasses import dataclass

# from common.span import SourceSpan
# from generated import _pb2
# from ir.expr.comprehension_ir import CompIR
# from ir.expr.expr_ir import ExprIR


# @dataclass
# class GeneratorExpIR(ExprIR):
#     elt: ExprIR
#     generators: list[CompIR]
#     span: SourceSpan | None

#     def to_proto(self):
#         return _pb2.ExprIR(
#             generator_exp=_pb2.GeneratorExpIR(
#                 elt=self.elt.to_proto(),
#                 generators=[gen.to_proto() for gen in self.generators],
#                 span=self.span.to_proto() if self.span is not None else None,
#             )
#         )

# it's defined in comprehension_ir.py already
