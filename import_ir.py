from span import SourceSpan
from typing import Union
from expression_ir import IdentifiedIRNode


class ImportIR(IdentifiedIRNode):
    def __init__(self, id: int, local_symbol_id: int, scope_id: int, kind: str, module_name: str, imported_name: Union[int, None], alias: Union[str, None], relative_level: int, span: Union[SourceSpan, None]):
        super().__init__(id=id, span=span)
        self.id=id
        self.local_symbol_id=local_symbol_id
        self.scope_id=scope_id
        self.kind=kind
        self.module_name=module_name
        self.imported_name=imported_name
        self.alias=alias
        self.relative_level=relative_level
        self.span=span
