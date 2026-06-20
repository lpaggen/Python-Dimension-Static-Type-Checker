from typing import List
from symbol_ir import SymbolIR
from scope_ir import ScopeIR
from import_ir import ImportIR
from decl_ir import DeclIR


class ProgramIR:
    def __int__(self, module_name: str, file_path: str, scopes: List[ScopeIR], symbols: List[SymbolIR], imports: List[ImportIR], decls: List[DeclIR]):
        self.module_name=module_name,
        self.file_path=file_path,
        self.scopes=scopes,
        self.symbols=symbols,
        self.imports=imports,
        self.decls=decls