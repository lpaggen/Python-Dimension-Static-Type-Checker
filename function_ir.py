from span import SourceSpan
from annotation_ir import AnnotationIR


class ParamIR:
    def __init__(self, symbol_id: int, name: str, annotation: AnnotationIR, default, span: SourceSpan):
        self.symbol_id = symbol_id
        self.name = name
        self.annotation = annotation
        self.default = default
        self.span = span

class FunctionIR:
    def __init__(
        self,
        id: int,
        symbol_id: int,
        name: str,
        scope_id: int,  # parent scope where function name is bound
        body_scope_id: int,  # function-local scope
        params: list[ParamIR],
        returns,
        decorators,
        span: SourceSpan,
    ):
        self.id = id
        self.symbol_id = symbol_id
        self.name = name
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.params = params
        self.returns = returns
        self.decorators = decorators
        self.span = span
