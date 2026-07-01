from common.span import SourceSpan
from .annotation_ir import AnnotationIR
from .ir_node import IRNode
from generated import _pb2
from .decl_ir import DeclIR


class BindingIR(DeclIR):
    def __init__(
        self,
        id: str,
        target_id: int,
        annotation: AnnotationIR,
        kind: int,
        value: IRNode,
        scope_id: int,
        span: SourceSpan,
    ):
        super().__init__(span=span)
        self.id = id
        self.target_id = target_id
        self.annotation = annotation
        self.kind = kind
        self.value = value
        self.scope_id = scope_id
        self.span = span

    def to_proto(self):
        proto = _pb2.BindingIR(
            id=self.id,
            target_id=self.target_id,
            kind=self.kind,
            scope_id=self.scope_id,
        )

        if self.annotation is not None:
            proto.annotation.CopyFrom(self.annotation.to_proto())

        if self.value is not None:
            proto.value.CopyFrom(self.value.to_proto())

        if self.span is not None:
            proto.span.CopyFrom(self.span.to_proto())

        stmt = _pb2.DeclIR()
        stmt.binding.CopyFrom(proto)
        return stmt