import ast
from .ir_builder import IRBuilder
from ir.import_ir import ImportIR
from ir.program_ir import ProgramIR
from common.span import SourceSpan
from ir.annotation_ir import AnnotationIR, AnnotationHeadIR
from ir.function_ir import ParamIR, ReturnIR
from ir.identifier_ir import IdentifierIR
from ir.attributeexpr_ir import AttributeExprIR
from common.operators import Operator
from ir.augassign_ir import AugAssignIR
from ir.forloop_ir import ForLoopIR
from ir.subscript_ir import SubscriptIR
from ir.tuple_ir import TupleIR
from ir.none_ir import NoneIR
from ir.slice_ir import SliceIR
from ir.whileloop_ir import WhileLoopIR
from ir.bool_ir import BooleanIR
from ir.exprstmt_ir import ExprStmtIR
from ir.boolop_ir import BoolOpIR
from ir.if_ir import IfIR
from ir.compare_ir import CompareIR
from ir.unaryop_ir import UnaryOpIR
from ir.callexpr_ir import CallExprIR, KeywordArgIR
from ir.list_ir import ListIR
from ir.integer_ir import IntegerIR
from ir.float_ir import FloatIR
from ir.string_ir import StringIR
from ir.binop_ir import BinOpIR
from common.kind import ScopeKind, SymbolKind, BindingKind, ImportKind


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
                kind=ImportKind.IMPORT_MODULE_ALIAS,
                scope_id=self.current_scope(),
                span=SourceSpan.span(node, self.file_path),
            )

            self.builder.add_import(
                local_symbol_id=symbol_id,
                scope_id=self.current_scope(),
                kind=ImportKind.IMPORT_MODULE,
                module_name=module_name,
                imported_name=None,
                alias=alias.asname,
                relative_level=0,
                span=SourceSpan.span(node, self.file_path),
            )

    def visit_ImportFrom(self, node: ast.ImportFrom):
        module_name = node.module  # ex. "a" in: from a import b

        for alias in node.names:
            imported_name = alias.name  # ex "b"
            bound_name = alias.asname or alias.name

            symbol_id = self.builder.declare_symbol(
                name=bound_name,
                kind=ImportKind.IMPORT_FROM,
                scope_id=self.current_scope(),
                span=SourceSpan.span(node, self.file_path),
            )

            self.builder.add_import(
                local_symbol_id=symbol_id,
                scope_id=self.current_scope(),
                kind=ImportKind.IMPORT_FROM,
                module_name=module_name,
                imported_name=imported_name,
                alias=alias.asname,
                relative_level=node.level,
                span=SourceSpan.span(node, self.file_path),
            )

    def visit_FunctionDef(self, node: ast.FunctionDef):
        parent_scope: int = self.current_scope()

        fn_symbol_id = self.builder.declare_symbol(
            name=node.name,
            kind=SymbolKind.SYMBOL_FUNCTION,
            scope_id=parent_scope,
            span=SourceSpan.span(node, self.file_path),
        )

        body_scope_id = self.builder.new_scope(
            name=node.name,
            kind=ScopeKind.SCOPE_FUNCTION,
            parent_id=parent_scope,
            span=SourceSpan.span(node, self.file_path),
        )

        self.scope_stack.append(body_scope_id)

        params = []

        for arg in node.args.args:
            param_symbol_id = self.builder.declare_symbol(
                name=arg.arg,
                kind=SymbolKind.SYMBOL_PARAM,
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

        body = []
        for stmt in node.body:
            lowered = self.visit(stmt)
            if lowered is not None:
                body.append(lowered)

        self.scope_stack.pop()

        self.builder.add_function(
            symbol_id=fn_symbol_id,
            name=node.name,
            scope_id=parent_scope,
            body_scope_id=body_scope_id,
            params=params,
            body=body,
            returns=self.lower_annotation(node.returns) if node.returns else None,
            decorators=[self.parse_expr(d) for d in node.decorator_list],
            span=SourceSpan.span(node, self.file_path),
        )

    def visit_ClassDef(self, node: ast.ClassDef):
        parent_scope = self.current_scope()

        class_symbol_id = self.builder.declare_symbol(
            name=node.name,
            kind=ScopeKind.SCOPE_CLASS,
            scope_id=parent_scope,
            span=SourceSpan.span(node, self.file_path),
        )

        class_scope_id = self.builder.new_scope(
            name=node.name,
            kind=ScopeKind.SCOPE_CLASS,
            parent_id=parent_scope,
            span=SourceSpan.span(node, self.file_path),
        )

        self.scope_stack.append(class_scope_id)

        # TODO modify classIR to have a body 
        body = []
        for stmt in node.body:
            lowered = self.visit(stmt)
            if lowered is not None:
                body.append(lowered)

        self.scope_stack.pop()

        self.builder.add_class(
            symbol_id=class_symbol_id,
            name=node.name,
            scope_id=parent_scope,
            body_scope_id=class_scope_id,
            body=body,
            bases=[self.parse_expr(base) for base in node.bases],
            decorators=[self.parse_expr(d) for d in node.decorator_list],
            span=SourceSpan.span(node, self.file_path),
        )

    def visit_AnnAssign(self, node: ast.AnnAssign):
        annotation_ir = self.lower_annotation(node.annotation)
        self.lower_assignment(
            target=node.target, 
            value=node.value, 
            kind=BindingKind.BINDING_ANNASSIGN, 
            annotation=annotation_ir, 
            span=SourceSpan.span(
                node=node, file_path=self.file_path
            )
        )

    def visit_While(self, node: ast.While):
        parent_scope = self.current_scope()
        loop_scope_id = self.builder.new_scope(
            name="<while>", 
            kind=ScopeKind.SCOPE_BLOCK, 
            parent_id=parent_scope, 
            span=SourceSpan.span(
                node,
                self.file_path
            )
        )

        test_ir = self.parse_expr(node.test)

        self.scope_stack.append(loop_scope_id)

        body = []
        for stmt in node.body:
            lowered = self.visit(stmt)
            if lowered is not None:
                body.append(lowered)

        orelse = []
        for stmt in node.orelse:
            lowered = self.visit(stmt)
            if lowered is not None:
                orelse.append(lowered)

        self.scope_stack.pop()

        return WhileLoopIR(
            test=test_ir,
            scope_id=self.current_scope(),
            body_scope_id=loop_scope_id,
            body=body,
            orelse=orelse,
            span=SourceSpan.span(node, self.file_path),
        )

    def visit_For(self, node: ast.For):
        parent_scope = self.current_scope()
        loop_scope_id = self.builder.new_scope(
            name="<for>", 
            kind=ScopeKind.SCOPE_BLOCK, 
            parent_id=parent_scope, 
            span=SourceSpan.span(
                node, self.file_path
            )
        )

        target_ir = self.parse_expr(node.target)
        iter_ir = self.parse_expr(node.iter)

        self.scope_stack.append(loop_scope_id)

        body = []
        for stmt in node.body:
            lowered = self.visit(stmt)
            if lowered is not None:
                body.append(lowered)

        orelse = []
        for stmt in node.orelse:
            lowered = self.visit(stmt)
            if lowered is not None:
                orelse.append(lowered)

        self.scope_stack.pop()

        return ForLoopIR(
            target=target_ir,
            iter=iter_ir,
            scope_id=self.current_scope(),
            body_scope_id=loop_scope_id,
            body=body,
            orelse=orelse,
            span=SourceSpan.span(node, self.file_path),
        )
    
    def visit_Return(self, node: ast.Return):
        return ReturnIR(
            value=self.parse_expr(node.value) if node.value else None,
            span=SourceSpan.span(node, self.file_path),
        )
    
    def visit_Expr(self, node: ast.Expr):
        return ExprStmtIR(
            value=self.parse_expr(node.value),
            span=SourceSpan.span(node, self.file_path),
        )
    
    def visit_If(self, node: ast.If):
        parent_scope = self.current_scope()

        then_scope_id = self.builder.new_scope(
            name="<if>", 
            kind=ScopeKind.SCOPE_BLOCK, 
            parent_id=parent_scope, 
            span=SourceSpan.span(
                node, self.file_path
            )
        )
        else_scope_id = self.builder.new_scope(
            name="<else>", 
            kind=ScopeKind.SCOPE_BLOCK, 
            parent_id=parent_scope, 
            span=SourceSpan.span(
                node, 
                self.file_path
            )
        )

        test_ir = self.parse_expr(node.test)

        self.scope_stack.append(then_scope_id)
        body = [ir for stmt in node.body if (ir := self.visit(stmt)) is not None]  # := evaluates to RHS, not assign 
        self.scope_stack.pop()

        self.scope_stack.append(else_scope_id)
        orelse = [ir for stmt in node.orelse if (ir := self.visit(stmt)) is not None]
        self.scope_stack.pop()

        return IfIR(
            test=test_ir,
            scope_id=parent_scope,
            then_scope_id=then_scope_id,
            else_scope_id=else_scope_id,
            body=body,
            orelse=orelse,
            span=SourceSpan.span(node, self.file_path),
        )

    def lower_annotation(self, annotation: ast.AST):
        if isinstance(annotation, ast.Name):  # simple types, int float str etc.
            return AnnotationIR(
                head=AnnotationHeadIR(
                    root=annotation.id,
                    attrs=[],
                    scope_id=self.current_scope(),
                    span=SourceSpan.span(annotation, self.file_path),
                ),
                args=[],
            )

        if isinstance(annotation, ast.Subscript):  # complex types, torch.Tensor, np.ndarray, torch.Tensor[2, 3] ...
            head=self.lower_annotation_head(annotation.value)
            if isinstance(annotation.slice, ast.Name):
                return AnnotationIR(
                    head=head,
                    args=[self.parse_expr(annotation.slice)]
                )
            if isinstance(annotation.slice, ast.Tuple):
                return AnnotationIR(
                head=head,
                args=[self.parse_expr(i) for i in annotation.slice.elts] #TODO this breaks for anything non-dimensional
            )
            return AnnotationIR(
                head=head,
                args=[self.parse_expr(annotation.slice)] #TODO this breaks for anything non-dimensional
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
        self.lower_assignment(
            target=node.targets[0],
            value=node.value,  # !! self.parse_value called downstream
            kind=BindingKind.BINDING_ASSIGN, 
            annotation=None, 
            span=SourceSpan.span(
                node=node, 
                file_path=self.file_path)
        )

    def lower_assignment(self, target: ast.AST, value: ast.AST, kind: str, annotation: AnnotationIR, span: SourceSpan):
        if isinstance(target, ast.Name):  # x = 5
            symbol_id = self.builder.declare_symbol(
                name=target.id, 
                kind=SymbolKind.SYMBOL_UNKNOWN, 
                scope_id=self.current_scope(), 
                span=span
            )

            value_ir = self.parse_expr(value) if value is not None else None

            self.builder.add_assign(
                target_id=symbol_id,
                annotation=annotation,
                kind=kind,
                scope_id=self.current_scope(),
                value=value_ir,
                span=SourceSpan.span(
                    node=target, 
                    file_path=span
                ),
            )

        if isinstance(target, ast.Tuple):  # a, b = 3, 4
            if not isinstance(value, ast.Tuple):
                self.builder.add_diagnostic(
                    msg="Cannot unpack data where RHS is not a tuple yet.",
                    span=span,
                )
                return

            if len(target.elts) != len(value.elts):
                self.builder.add_diagnostic(
                    msg=f"Size of LHS ({len(target.elts)}) and RHS ({len(value.elts)}) do not match.",
                    span=span,
                )
                return

            for left, right in zip(target.elts, value.elts):
                self.lower_assignment(
                    target=left, 
                    value=right, 
                    kind=kind, 
                    annotation=annotation, 
                    span=span
                )

    def parse_expr(self, node: ast.AST):
        """
        Recursive function to parse RHS of AnnAssign, Assign
        """
        if isinstance(node, ast.Constant):
            if isinstance(node.value, bool):
                return BooleanIR(node.value)

            if isinstance(node.value, int):
                return IntegerIR(node.value)

            if isinstance(node.value, float):
                return FloatIR(node.value)

            if isinstance(node.value, str):
                return StringIR(node.value)

            # TODO change this, not the right way to go
            if node.value is None:
                return NoneIR()

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
                kwargs=[
                    KeywordArgIR(
                        name=kw.arg,
                        value=self.parse_expr(kw.value),
                        span=SourceSpan.span(kw.value, self.file_path),
                    )
                    for kw in node.keywords
                ],
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.UnaryOp):
            return UnaryOpIR(
                op=Operator.unaryop_to_operator(node.op),
                operand=self.parse_expr(node.operand),
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.Compare):
            return CompareIR(
                left=self.parse_expr(node.left),
                ops=[Operator.cmpop_to_operator(op) for op in node.ops],
                comparators=[self.parse_expr(c) for c in node.comparators],
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.List):
            return ListIR(
                elements=[self.parse_expr(arg) for arg in node.elts],
                span=SourceSpan.span(node, self.file_path)
            )

        if isinstance(node, ast.BoolOp):
            return BoolOpIR(
                op=Operator.boolop_to_operator(node.op),
                values=[self.parse_expr(v) for v in node.values],
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.BinOp):
            return BinOpIR(
                left=self.parse_expr(node.left),
                right=self.parse_expr(node.right),
                op=Operator.binop_to_operator(node.op),
                span=SourceSpan.span(node=node, file_path=self.file_path)
            )

        if isinstance(node, ast.Subscript):
            return SubscriptIR(
                target=self.parse_expr(node.value),
                subscript=self.parse_expr(node.slice),
                span=SourceSpan.span(node, self.file_path)
            )

        if isinstance(node, ast.Tuple):
            return TupleIR(
                elements=tuple([self.parse_expr(arg) for arg in node.elts]),
                span=SourceSpan.span(node, self.file_path)
            )

        if isinstance(node, ast.Slice):
            return SliceIR(
                lower=self.parse_expr(node.lower) if node.lower else None,
                upper=self.parse_expr(node.upper) if node.upper else None,
                step=self.parse_expr(node.step) if node.step else None,
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.JoinedStr):
            return StringIR(
                value="",  # placeholder for ignored f-string content
                span=SourceSpan.span(node, self.file_path),
            )

        if node is None:
            return NoneIR(span=SourceSpan.span(node, self.file_path))

        raise NotImplementedError(f"Unsupported expression node: {type(node).__name__}")

    def parse_dim_expr(self, node: ast.AST): ...
