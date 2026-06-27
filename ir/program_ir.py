from typing import List
from .symbol_ir import SymbolIR
from .scope_ir import ScopeIR
from .import_ir import ImportIR
from .binding_ir import BindingIR
from generated import _pb2


class ProgramIR:
    def __init__(
        self,
        module_name: str,
        file_path: str,
        scopes: List[ScopeIR],
        symbols: List[SymbolIR],
        imports: List[ImportIR],
        decls: List[BindingIR],
    ):
        self.module_name = module_name
        self.file_path = file_path
        self.scopes = scopes
        self.symbols = symbols
        self.imports = imports
        self.decls = decls

    def to_proto(self):
        proto = _pb2.ProgramIR(
            module_name=self.module_name,
            file_path=self.file_path,
        )

        proto.scopes.extend([s.to_proto() for s in self.scopes])
        proto.symbols.extend([s.to_proto() for s in self.symbols])
        proto.imports.extend([i.to_proto() for i in self.imports])
        proto.decls.extend([stmt.to_proto() for stmt in self.decls])

        return proto
