from scope_ir import ScopeIR
from symbol_ir import SymbolIR
from program_ir import ProgramIR
from import_ir import ImportIR
from typing import Optional
# from type import Type
from span import SourceSpan
from binding_ir import BindingIR
# from literal import Literal
from function_ir import FunctionIR
from class_ir import ClassIR
from annotation_ir import AnnotationIR
from expression_ir import ExprIR


class IRBuilder:
    def __init__(self, file_path: str, module_name: str) -> None:
        self.file_path = file_path
        self.module_name = module_name
        self.scopes = []
        self.symbols = []
        self.decls = []
        self.imports = []
        
        self.diagnostics = []

        self.next_scope_id = 0
        self.next_symbol_id = 0
        self.next_decl_id = 0
        self.next_import_id = 0

        self.global_scope_id = self.scopes.append(
            self.new_scope(
                name="<module>",
                kind="MODULE",
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

    def add_function(
        self,
        symbol_id,
        name,
        scope_id,
        body_scope_id,
        params,
        returns,
        decorators,
        span,
        ):
        decl_id = self.next_decl_id
        self.next_decl_id += 1

        self.decls.append(
            FunctionIR(
                id=decl_id,
                symbol_id=symbol_id,
                name=name,
                scope_id=scope_id,
                body_scope_id=body_scope_id,
                params=params,
                returns=returns,
                decorators=decorators,
                span=span,
            )
        )

        return decl_id

    def add_class(
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
        decl_id = self.next_decl_id
        self.next_decl_id += 1

        self.decls.append(
            ClassIR(
                id=decl_id,
                symbol_id=symbol_id,
                name=name,
                scope_id=scope_id,
                body_scope_id=body_scope_id,
                bases=bases,
                decorators=decorators,
                span=span,
            )
        )

        return decl_id

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

    def add_import(self, local_symbol_id, scope_id, kind, module_name, imported_name, alias, relative_level, span):
        import_id = self.next_import_id
        self.next_import_id += 1
        self.imports.append(
            ImportIR(
                id=import_id,
                local_symbol_id=local_symbol_id,
                scope_id=scope_id,
                kind=kind,
                module_name=module_name,
                imported_name=imported_name,
                alias=alias,
                relative_level=relative_level,
                span=span
            )
        )

        return import_id

    def add_assign(self, target_id: int, annotation: AnnotationIR, kind: str, scope_id: int, value: ExprIR, span: SourceSpan):
        decl_id = self.next_decl_id
        self.decls.append(
            BindingIR(
                id=decl_id,
                target_id=target_id,
                annotation=annotation,
                kind=kind,
                value=value,
                scope_id=scope_id,
                span=span
            )
        )
        
        return decl_id


    def finish(self) -> ProgramIR:
        return ProgramIR(
            module_name=self.module_name,
            file_path=self.file_path,
            scopes=self.scopes,
            symbols=self.symbols,
            imports=self.imports,
            decls=self.decls,
        )

    def add_diagnostic(self, msg: str, span: SourceSpan):
        self.diagnostics.append((msg, span))
