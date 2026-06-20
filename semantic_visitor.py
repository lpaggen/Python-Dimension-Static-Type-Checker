import ast
from ir_builder import IRBuilder
from import_ir import ImportIR


class SemanticBuilder(ast.NodeVisitor):
    def __init__(self, module_name: str, file_path: str):
        self.builder = IRBuilder(module_name, file_path)
        self.file_path = file_path
        self.scope_stack = [self.builder.global_scope_id]

    def current_scope(self) -> int:
        return self.scope_stack[-1]

    def build(self, tree: ast.AST) -> ProgramIR:
        self.visit(tree)
        return self.builder.finish()

    def visit_Import(self, node: ast.Import):
        for alias in node.names:
            module_name = alias.name
            bound_name = alias.asname or module_name.split(".")[0]

            symbol_id = self.builder.declare_symbol(
                name=bound_name,
                kind="MODULE_ALIAS",
                scope_id=self.current_scope,
                span=self.span(node),
            )

            self.builder.add_import(
                local_symbol_id=symbol_id,
                scope_id=self.current_scope,
                kind="module",
                module_name=module_name,
                imported_name=None,
                alias=alias.asname,
                relative_level=0,
                span=self.span(node),
            )

    def visit_ImportFrom(self, node: ast.ImportFrom):
        ...

    def visit_FunctionDef(self, node: ast.FunctionDef):
        ...

    def visit_ClassDef(self, node: ast.ClassDef):
        ...

    def visit_AnnAssign(self, node: ast.AnnAssign):
        ...

    def visit_Assign(self, node: ast.Assign):
        ...

    def parse_expr(self, node: ast.AST) -> ExprIR:
        ...

    def parse_annotation(self, node: ast.AST) -> TypeIR:
        ...

    def parse_dim_expr(self, node: ast.AST | None) -> DimExprIR:
        ...

    def span(self, node: ast.AST) -> SourceSpan:
        ...