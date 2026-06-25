from common.span import SourceSpan
from .identified_ir_node import IdentifiedIRNode
from .ir_node import IRNode


class ClassIR(IdentifiedIRNode):
    def __init__(
        self,
        id: int,
        symbol_id: int,
        name: str,
        scope_id: int,  # parent scope
        body_scope_id: int,  # class-local scope
        body: list[IRNode],
        bases: list,  # Base classes: Base, nn.Module, etc.
        decorators: list,
        span: SourceSpan,
    ):
        super().__init__(id=id, span=span)
        self.id = id
        self.symbol_id = symbol_id
        self.name = name
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.body=body
        self.bases = bases
        self.decorators = decorators
        self.span = span
