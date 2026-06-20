import ast
from ir_builder import IRBuilder
from import_ir import ImportIR
from program_ir import ProgramIR
from span import SourceSpan
from type import Type, TensorType, IntegerType, FloatType
from annotation_ir import AnnotationIR, AnnotationHeadIR
from dimension import Dim


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
                scope_id=self.current_scope(),
                span=self.span(node),
            )

            self.builder.add_import(
                local_symbol_id=symbol_id,
                scope_id=self.current_scope(),
                kind="module",
                module_name=module_name,
                imported_name=None,
                alias=alias.asname,
                relative_level=0,
                span=self.span(node),
            )

    def visit_ImportFrom(self, node: ast.ImportFrom):
        for alias in node.names:
            module_name = alias.name
            bound_name = alias.asname or module_name.split(".")[0]

            symbol_id = self.builder.declare_symbol(
                name=bound_name,
                kind="MODULE_ALIAS",
                scope_id=self.current_scope(),
                span=self.span(node),
            )

            self.builder.add_import(
                local_symbol_id=symbol_id,
                scope_id=self.current_scope(),
                kind="module",
                module_name=module_name,
                imported_name=None,
                alias=alias.asname,
                relative_level=0,
                span=self.span(node),
            )

    def visit_FunctionDef(self, node: ast.FunctionDef): ...

    def visit_ClassDef(self, node: ast.ClassDef): ...

    def visit_AnnAssign(self, node: ast.AnnAssign):
        annotation = node.annotation
        self.lower_annotation(annotation)
        self.lower_assignment(target=node.target, value=node.value, kind="ANNASSIGN", annotation=annotation, span=self.span(node=node))

    def lower_annotation(self, annotation: ast.AST):
        if isinstance(annotation, ast.Name):  # simple types, int float str etc.
            match annotation.id:
                case "int":
                    return AnnotationIR(
                        head="int",
                        args=[]
                    )
                case "float":
                    return AnnotationIR(
                        head="float",
                        args=[]
                    )

        if isinstance(annotation, ast.Subscript):  # complex types, torch.Tensor, np.ndarray, torch.Tensor[2, 3] ...
            return AnnotationIR(
                head=self.lower_annotation_head(annotation.value),
                args=[Dim.toDim(i) for i in annotation.slice.elts]
            )

        return AnnotationIR(  # case where annotation = Attribute
                head=self.lower_annotation_head(annotation.value),
                args=[]
            )
    
    def lower_annotation_head(self, node: ast.AST) -> AnnotationHeadIR:
        parts = []
        current = node

        while isinstance(current, ast.Attribute):
            parts.append(current.attr)
            current = current.value

        if isinstance(current, ast.Name):
            root = current.id
            attrs = list(reversed(parts))

            return AnnotationHeadIR(
                root=root,
                attrs=attrs,
                scope_id=self.current_scope(),
                span=self.span(node),
            )

        raise NotImplementedError(f"Unsupported annotation head: {type(node).__name__}")

    def visit_Assign(self, node: ast.Assign):
        self.lower_assignment(target=node.targets[0], value=node.value, kind="ASSIGN", annotation=None, span=self.span(node=node))

    def lower_assignment(self, target: ast.AST, value: ast.AST, kind: str, annotation: Type, span: SourceSpan):
        if isinstance(target, ast.Name):  # x = 5
            symbol_id = self.builder.declare_symbol(
                name=target.id, kind="UNKNOWN", scope_id=self.current_scope(), span=span
            )

            self.builder.add_assign(
                target_id=symbol_id,
                annotation=annotation,
                kind=kind,
                scope_id=self.current_scope(),
                value=self.parse_expr(value),
                span=self.span(node=target),
            )

        if isinstance(target, ast.Tuple):  # a, b = 3, 4
            if not isinstance(value, ast.Tuple):
                self.builder.add_diagnostic(
                    msg="Cannot unpack data where RHS is not a tuple yet.",
                    span=self.span,
                )
                return

            if len(target.elts) != len(value.elts):
                self.builder.add_diagnostic(
                    msg=f"Size of LHS ({len(target.elts)}) and RHS ({len(value.elts)}) do not match.",
                    span=self.span,
                )
                return

            for left, right in zip(target.elts, value.elts):
                self.lower_assignment(target=left, value=right, kind=kind, annotation=annotation, span=span)

    def parse_expr(self, node: ast.AST): ...

    def parse_annotation(self, node: ast.AST): ...

    def parse_dim_expr(self, node: ast.AST): ...

    def span(self, node: ast.AST) -> SourceSpan: ...
