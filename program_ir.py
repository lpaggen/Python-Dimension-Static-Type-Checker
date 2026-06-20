from typing import List
from symbol_ir import SymbolIR
from scope_ir import ScopeIR
from import_ir import ImportIR
from binding_ir import BindingIR


class ProgramIR:
    def __init__(self, module_name: str, file_path: str, scopes: List[ScopeIR], symbols: List[SymbolIR], imports: List[ImportIR], decls: List[BindingIR]):
        self.module_name=module_name
        self.file_path=file_path
        self.scopes=scopes
        self.symbols=symbols
        self.imports=imports
        self.decls=decls
