from type import Type, TensorType
from literal import TensorLiteral, Literal
from span import SourceSpan
from annotation_ir import AnnotationIR


class BindingIR:
    def __init__(self, id: str, target_id: int, annotation: AnnotationIR, kind: str, value: Literal, scope_id: int, span: SourceSpan):
        self.id=id
        self.target_id=target_id
        self.annotation=annotation
        self.kind=kind
        self.value=value
        self.scope_id=scope_id
        self.span=span

# class DimDeclIR():
#     pass
