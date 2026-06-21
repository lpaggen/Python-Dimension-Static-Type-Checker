from span import SourceSpan


class ClassIR:
    def __init__(
        self,
        id: int,
        symbol_id: int,
        name: str,
        scope_id: int,       # parent scope
        body_scope_id: int,  # class-local scope
        bases: list,         # Base classes: Base, nn.Module, etc.
        decorators: list,
        span: SourceSpan,
    ):
        self.id = id
        self.symbol_id = symbol_id
        self.name = name
        self.scope_id = scope_id
        self.body_scope_id = body_scope_id
        self.bases = bases
        self.decorators = decorators
        self.span = span
