from common.span import SourceSpan
from ir.annotation.annotation_ir import AnnotationIR
from ir.ir_node import IRNode
from generated import _pb2
from .decl_ir import DeclIR
from dataclasses import dataclass


@dataclass
class BindingIR(DeclIR):
    id: int
    target_id: int
    annotation: AnnotationIR | None
    kind: int
    value: IRNode | None
    scope_id: int
    span: SourceSpan
    def to_proto(self):
        proto = self._binding_proto()
        decl = _pb2.DeclIR()
        decl.binding.CopyFrom(proto)
        return decl

    def to_stmt_proto(self):
        proto = self._binding_proto()
        stmt = _pb2.StmtIR()
        stmt.binding.CopyFrom(proto)
        return stmt

    def _binding_proto(self):
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

        return proto
