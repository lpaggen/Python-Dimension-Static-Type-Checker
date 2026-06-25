from common.span import SourceSpan
from .annotation_ir import AnnotationIR
from .ir_node import IRNode
from .identified_ir_node import IdentifiedIRNode


class ParamIR(IRNode):
    def __init__(
        self,
        symbol_id: int,
        name: str,
        annotation: AnnotationIR,
        default,
        span: SourceSpan,
    ):
        self.symbol_id = symbol_id
        self.name = name
        self.annotation = annotation
        self.default = default
        self.span = span


class FunctionIR(IdentifiedIRNode):
    def __init__(
        self,
        id: int,  # symbol id -> name of function
        symbol_id: int,
        name: str,
        scope_id: int,  # parent scope where function name is bound
        body_scope_id: int,  # function-local scope
        params: list[ParamIR],
        body: list[IRNode],
        returns,
        decorators,
        span: SourceSpan,
    ):
        super().__init__(id=id, span=span)
        self.id = id
        self.symbol_id = symbol_id
        self.name = name
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.params = params
        self.body=body
        self.returns = returns
        self.decorators = decorators
        self.span = span
