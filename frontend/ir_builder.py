from ir.metadata.scope_ir import ScopeIR
from ir.metadata.symbol_ir import SymbolIR
from ir.program_ir import ProgramIR
from ir.stmt.import_ir import ImportIR
from ir.expr.expr_ir import ExprIR

from common.span import SourceSpan

from ir.stmt.binding_ir import BindingIR
from ir.stmt.functiondef_ir import FunctionDefIR
from ir.stmt.classdef_ir import ClassDefIR
from ir.annotation.annotation_ir import AnnotationIR

from common.kind import ScopeKind


class IRBuilder:
    def __init__(self, file_path: str, module_name: str) -> None:
        self.file_path = file_path
        self.module_name = module_name
        self.scopes = []
        self.symbols = []
        self.decls = []
        self.imports = []
        self.body = []

        self.diagnostics = []

        self.next_scope_id = 0
        self.next_symbol_id = 0
        self.next_decl_id = 0
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

    def add_function(
        self,
        symbol_id,
        name,
        scope_id,
        body_scope_id,
        args,
        body,
        returns,
        decorator_list,
        type_comment,
        type_params,
        span,
    ):
        decl_id = self.next_decl_id
        self.next_decl_id += 1

        function = FunctionDefIR(
            id=decl_id,
            symbol_id=symbol_id,
            name=name,
            scope_id=scope_id,
            body_scope_id=body_scope_id,
            body=body,
            args=args,
            returns=returns,
            decorator_list=decorator_list,
            type_comment=type_comment,
            type_params=type_params,
            span=span,
        )
        self.decls.append(function)

        return function

    def add_class(
        self,
        symbol_id: int,
        name: str,
        scope_id: int,  # parent scope
        body_scope_id: int,  # class-local scope
        body,
        bases: list,  # Base classes: Base, nn.Module, etc.
        keywords: list,
        decorator_list: list,
        type_params: list,
        span: SourceSpan,
    ):
        decl_id = self.next_decl_id
        self.next_decl_id += 1

        class_decl = ClassDefIR(
            id=decl_id,
            symbol_id=symbol_id,
            name=name,
            scope_id=scope_id,
            body=body,
            body_scope_id=body_scope_id,
            bases=bases,
            keywords=keywords,
            decorator_list=decorator_list,
            type_params=type_params,
            span=span,
        )
        self.decls.append(class_decl)

        return class_decl

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

    def add_assign(
        self,
        target_id: int,
        annotation: AnnotationIR,
        kind: str,
        scope_id: int,
        value: ExprIR,
        span: SourceSpan,
    ):
        decl_id = self.next_decl_id
        self.next_decl_id += 1
        binding = BindingIR(
            id=decl_id,
            target_id=target_id,
            annotation=annotation,
            kind=kind,
            value=value,
            scope_id=scope_id,
            span=span,
        )
        self.decls.append(binding)

        return binding

    def finish(self) -> ProgramIR:
        return ProgramIR(
            module_name=self.module_name,
            file_path=self.file_path,
            scopes=self.scopes,
            symbols=self.symbols,
            imports=self.imports,
            decls=self.decls,
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
