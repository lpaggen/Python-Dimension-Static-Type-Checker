from enum import Enum
from dataclasses import dataclass

from common.span import SourceSpan
from generated import _pb2
from ir.annotation.annotation_ir import AnnotationIR
from ir.ir_node import IRNode


class ArgKind(Enum):
    POSITIONAL_ONLY = 1
    POSITIONAL_OR_KEYWORD = 2
    VAR_POSITIONAL = 3
    KEYWORD_ONLY = 4
    VAR_KEYWORD = 5


@dataclass
class ArgIR(IRNode):
    symbol_id: int
    arg: str
    kind: ArgKind
    annotation: AnnotationIR | None
    default: object | None
    span: SourceSpan

    def to_proto(self):
        proto = _pb2.ArgIR(
            symbol_id=self.symbol_id,
            arg=self.arg,
            kind=self.kind.value
        )

        if self.annotation is not None:
            proto.annotation.CopyFrom(self.annotation.to_proto())

        if self.default is not None:
            proto.default.CopyFrom(self.default.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        return proto
