import ast
from ir_builder import IRBuilder
from import_ir import ImportIR
from program_ir import ProgramIR
from span import SourceSpan
from annotation_ir import AnnotationIR, AnnotationHeadIR
from dimension_ir import DimIR
from function_ir import ParamIR
from identifier_ir import IdentifierIR
from expression_ir import ListIR, IntegerIR, FloatIR, IdentifiedIRNode, CallExprIR, BinOpIR
from attributeexpr_ir import AttributeExprIR
from operators import Operator
from augassign_ir import AugAssignIR



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

    def visit_AugAssign(self, node: ast.AugAssign):
        target = node.target
        op = node.op
        value = self.parse_expr(node.value)
        return AugAssignIR(
            target=target,
            value=value, 
            op=op,
            span=SourceSpan.span(
                node=node
            )
        )

    def visit_Import(self, node: ast.Import):
        for alias in node.names:
            module_name = alias.name
            bound_name = alias.asname or module_name.split(".")[0]

            symbol_id = self.builder.declare_symbol(
                name=bound_name,
                kind="MODULE_ALIAS",
                scope_id=self.current_scope(),
                span=SourceSpan.span(node, self.file_path),
            )

            self.builder.add_import(
                local_symbol_id=symbol_id,
                scope_id=self.current_scope(),
                kind="MODULE",
                module_name=module_name,
                imported_name=None,
                alias=alias.asname,
                relative_level=0,
                span=SourceSpan.span(node, self.file_path),
            )

    def visit_ImportFrom(self, node: ast.ImportFrom):
        for alias in node.names:
            module_name = alias.name
            bound_name = alias.asname or module_name.split(".")[0]

            symbol_id = self.builder.declare_symbol(
                name=bound_name,
                kind="MODULE_ALIAS",
                scope_id=self.current_scope(),
                span=SourceSpan.span(node, self.file_path),
            )

            self.builder.add_import(
                local_symbol_id=symbol_id,
                scope_id=self.current_scope(),
                kind="MODULE",
                module_name=module_name,
                imported_name=None,
                alias=alias.asname,
                relative_level=0,
                span=SourceSpan.span(node, self.file_path),
            )

    def visit_FunctionDef(self, node: ast.FunctionDef):
        parent_scope = self.current_scope()

        fn_symbol_id = self.builder.declare_symbol(
            name=node.name,
            kind="FUNCTION",
            scope_id=parent_scope,
            span=SourceSpan.span(node, self.file_path),
        )

        body_scope_id = self.builder.new_scope(
            name=node.name,
            kind="FUNCTION",
            parent_id=parent_scope,
            span=SourceSpan.span(node, self.file_path),
        )

        self.scope_stack.append(body_scope_id)

        params = []

        for arg in node.args.args:
            param_symbol_id = self.builder.declare_symbol(
                name=arg.arg,
                kind="PARAM",
                scope_id=self.current_scope(),
                span=SourceSpan.span(arg, self.file_path),
            )

            params.append(
                ParamIR(
                    symbol_id=param_symbol_id,
                    name=arg.arg,
                    annotation=self.lower_annotation(arg.annotation) if arg.annotation else None,
                    default=None,
                    span=SourceSpan.span(arg, self.file_path),
                )
            )

        # now visit function body while inside function scope
        for stmt in node.body:
            self.visit(stmt)

        self.scope_stack.pop()

        self.builder.add_function(
            symbol_id=fn_symbol_id,
            name=node.name,
            scope_id=parent_scope,
            body_scope_id=body_scope_id,
            params=params,
            returns=self.lower_annotation(node.returns) if node.returns else None,
            decorators=[self.parse_expr(d) for d in node.decorator_list],
            span=SourceSpan.span(node, self.file_path),
        )

    def visit_ClassDef(self, node: ast.ClassDef):
        parent_scope = self.current_scope()

        class_symbol_id = self.builder.declare_symbol(
            name=node.name,
            kind="CLASS",
            scope_id=parent_scope,
            span=SourceSpan.span(node, self.file_path),
        )

        class_scope_id = self.builder.new_scope(
            name=node.name,
            kind="CLASS",
            parent_id=parent_scope,
            span=SourceSpan.span(node, self.file_path),
        )

        self.scope_stack.append(class_scope_id)

        for stmt in node.body:
            self.visit(stmt)

        self.scope_stack.pop()

        self.builder.add_class(
            symbol_id=class_symbol_id,
            name=node.name,
            scope_id=parent_scope,
            body_scope_id=class_scope_id,
            bases=[self.parse_expr(base) for base in node.bases],
            decorators=[self.parse_expr(d) for d in node.decorator_list],
            span=SourceSpan.span(node, self.file_path),
        )

    def visit_AnnAssign(self, node: ast.AnnAssign):
        annotation = node.annotation
        self.lower_annotation(annotation)
        self.lower_assignment(target=node.target, value=node.value, kind="ANNASSIGN", annotation=annotation, span=SourceSpan.span(node=node, file_path=self.file_path))
        
    def visit_For(self, node: ast.For):
        print(node)

    def lower_annotation(self, annotation: ast.AST): # TODO rework, too strict, ? how handle torch.something etc ?
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
                case _:
                    self.builder.add_diagnostic(f"Warning unknown return type {annotation.id} at {SourceSpan.span(annotation)}")
                    return AnnotationIR(
                        head=annotation.id,
                        args=[]
                    )
                

        if isinstance(annotation, ast.Subscript):  # complex types, torch.Tensor, np.ndarray, torch.Tensor[2, 3] ...
            return AnnotationIR(
                head=self.lower_annotation_head(annotation.value),
                args=[DimIR.toDim(i) for i in annotation.slice.elts] #TODO this breaks for anything non-dimensional
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
                span=SourceSpan.span(node, self.file_path),
            )

        raise NotImplementedError(f"Unsupported annotation head: {type(node).__name__}")

    def visit_Assign(self, node: ast.Assign):
        self.lower_assignment(target=node.targets[0], value=node.value, kind="ASSIGN", annotation=None, span=SourceSpan.span(node=node, file_path=self.file_path))

    def lower_assignment(self, target: ast.AST, value: ast.AST, kind: str, annotation: AnnotationIR, span: SourceSpan):
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
                span=SourceSpan.span(node=target, file_path=self.file_path),
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

    def parse_expr(self, node: ast.AST):
        """
        Recursive function to parse RHS of AnnAssign, Assign
        """
        if isinstance(node, ast.Constant):
            if isinstance(node.value, int):
                return IntegerIR(node.value)
            if isinstance(node.value, float):
                return FloatIR(node.value)
            raise NotImplementedError(f"Unsupported constant: {node.value!r}")

        if isinstance(node, ast.Name):
            return IdentifierIR(
                name=node.id,
                use_scope_id=self.current_scope(),
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.Attribute):
            return AttributeExprIR(
                base=self.parse_expr(node.value),
                attr=node.attr,
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.Call):
            return CallExprIR(
                callee=self.parse_expr(node.func),
                args=[self.parse_expr(arg) for arg in node.args],
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.List):
            return ListIR(
                elements=[self.parse_expr(arg) for arg in node.elts],
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.BinOp):
            return BinOpIR(
                left=self.parse_expr(node.left),
                right=self.parse_expr(node.right),
                op=Operator.binop_tostr(node.op),
                span=SourceSpan.span(node=node, file_path=self.file_path)
            )

        raise NotImplementedError(f"Unsupported expression node: {type(node).__name__}")

    def parse_dim_expr(self, node: ast.AST): ...
