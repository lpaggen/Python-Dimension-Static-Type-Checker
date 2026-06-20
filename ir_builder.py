from scope_ir import ScopeIR
from symbol_ir import SymbolIR


class IRBuilder:
    def __init__(self, file_path: str, module_name: str) -> None:
        self.file_path = file_path
        self.module_name = module_name
        self.scopes = []
        self.symbols = []
        self.decls = []
        self.imports = []

        self.next_scope_id = 0
        self.next_symbol_id = 0
        self.next_decl_id = 0
        self.next_import_id = 0

        self.scopes.append(
            self.new_scope(
                name="<module>",
                kind="module",
                parent_id=None,
                span=None
            )
        )

    def new_scope(self, name, kind, parent_id, span) -> int:
        """
        Push a scopeIR to the scope stack and return its ID so other modules can refer to it
        """
        scope_id = self.next_scope_id
        self.next_scope_id += 1

        self.scopes.append(
            ScopeIR(
                id=scope_id,
                name=name,
                kind=kind,
                parent_id=parent_id,
                span=span
            )
        )

        return scope_id

    def declare_symbol(self, name, kind, scope_id, span) -> int:
        """
        Push a symbol to the symbol stack and return its ID
        """
        symbol_id = self.next_symbol_id
        self.next_symbol_id += 1
        
        self.symbols.append(
            SymbolIR(
                id=symbol_id,
                name=name,
                kind=kind,
                scope_id=scope_id,
                span=span
            )    
        )
    
        return symbol_id
    
    def build_ir(self):
        return