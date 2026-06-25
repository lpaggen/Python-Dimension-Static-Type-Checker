from common.span import SourceSpan
from .annotation_ir import AnnotationIR
from .expression_ir import IdentifiedIRNode
from .ir_node import IRNode


class BindingIR(IdentifiedIRNode):
    def __init__(
        self,
        id: str,
        target_id: int,
        annotation: AnnotationIR,
        kind: str,
        value: IRNode,
        scope_id: int,
        span: SourceSpan,
    ):
        super().__init__(id=id, span=span)
        self.id = id
        self.target_id = target_id
        self.annotation = annotation
        self.kind = kind
        self.value = value
        self.scope_id = scope_id
        self.span = span
