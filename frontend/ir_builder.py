from ir.metadata.scope_ir import ScopeIR
from ir.metadata.symbol_ir import SymbolIR
from ir.program_ir import ProgramIR
from ir.stmt.import_ir import ImportIR

from common.span import SourceSpan

from common.kind import ScopeKind


class IRBuilder:
    def __init__(self, file_path: str, module_name: str) -> None:
        self.file_path = file_path
        self.module_name = module_name
        self.scopes = []
        self.symbols = []
        self.imports = []
        self.body = []

        self.diagnostics = []

        self.next_scope_id = 0
        self.next_symbol_id = 0
        self.next_import_id = 0

        self.global_scope_id = self.new_scope(
            name="<module>",
            kind=ScopeKind.SCOPE_MODULE,
            parent_id=None,
            span=None,
        )

    def new_scope(self, name, kind, parent_id, span) -> int:
        """
        Push a scopeIR to the scope stack and return its ID so other modules can refer to it
        """
        scope_id = self.next_scope_id
        self.next_scope_id += 1

        self.scopes.append(
            ScopeIR(id=scope_id, name=name, kind=kind, parent_id=parent_id, span=span)
        )

        return scope_id

    def declare_symbol(self, name, kind, scope_id, span) -> int:
        """
        Push a symbol to the symbol stack and return its ID
        """
        symbol_id = self.next_symbol_id
        self.next_symbol_id += 1

        self.symbols.append(
            SymbolIR(id=symbol_id, name=name, kind=kind, scope_id=scope_id, span=span)
        )

        return symbol_id

    def add_import(
        self,
        local_symbol_id,
        scope_id,
        kind,
        module_name,
        imported_name,
        alias,
        relative_level,
        span,
    ):
        import_id = self.next_import_id
        self.next_import_id += 1
        import_ir = ImportIR(
            id=import_id,
            local_symbol_id=local_symbol_id,
            scope_id=scope_id,
            kind=kind,
            module_name=module_name,
            imported_name=imported_name,
            alias=alias,
            relative_level=relative_level,
            span=span,
        )
        self.imports.append(import_ir)

        return import_ir

    def finish(self) -> ProgramIR:
        return ProgramIR(
            module_name=self.module_name,
            file_path=self.file_path,
            scopes=self.scopes,
            symbols=self.symbols,
            imports=self.imports,
            body=self.body,
        )

    def add_diagnostic(self, msg: str, span: SourceSpan):
        self.diagnostics.append((msg, span))

    def get_or_declare_symbol(self, name, kind, scope_id, span) -> int:
        """Return the symbol already bound in a lexical scope, or declare it."""
        for symbol in self.symbols:
            if symbol.name == name and symbol.scope_id == scope_id:
                return symbol.id

        return self.declare_symbol(name, kind, scope_id, span)
