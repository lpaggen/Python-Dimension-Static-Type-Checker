from dataclasses import dataclass
from typing import List
from common.span import SourceSpan
from ir.annotation.annotationhead_ir import AnnotationHeadIR
from ir.expr.expr_ir import ExprIR
from generated import _pb2
from ir.ir_node import IRNode


@dataclass
class AnnotationIR(IRNode):
    head: AnnotationHeadIR
    args: List[ExprIR]

    def to_proto(self):
        proto = _pb2.AnnotationIR()

        if self.head is not None:
            proto.head.CopyFrom(self.head.to_proto())

        proto.args.extend([arg.to_proto() for arg in self.args])

        return proto
