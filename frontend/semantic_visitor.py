import ast

from common.typeparam_ir import ParamSpecIR, TypeVarIR, TypeVarTupleIR
from ir.expr.attribute_ir import AttributeIR
from ir.expr.await_ir import AwaitIR
from ir.expr.binop_ir import BinOpIR
from ir.expr.boolop_ir import BoolOpIR
from ir.expr.call_ir import CallIR, KeywordIR
from ir.expr.compare_ir import CompareIR
from ir.expr.comprehension_ir import CompIR, DictCompIR, GeneratorExpIR, ListCompIR, SetCompIR
from ir.expr.constant_ir import BooleanIR, BytesIR, ComplexIR, EllipsisIR, FloatIR, IntegerIR, NoneIR, StringIR
from ir.expr.conversion_ir import Conversion
from ir.expr.dict_ir import DictIR
from ir.expr.expr_ir import ExprIR
from ir.expr.formattedvalue_ir import FormattedValueIR
from ir.expr.ifexp_ir import IfExpIR
from ir.expr.interpolation_ir import InterpolationIR
from ir.expr.joinedstr_ir import JoinedStrIR
from ir.expr.lambda_ir import LambdaIR
from ir.expr.list_ir import ListIR
from ir.expr.name_ir import NameIR
from ir.expr.namedexpr_ir import NamedExprIR
from ir.expr.set_ir import SetIR
from ir.expr.slice_ir import SliceIR
from ir.expr.starred_ir import StarredIR
from ir.expr.subscript_ir import SubscriptIR
from ir.expr.tuple_ir import TupleIR
from ir.expr.unaryop_ir import UnaryOpIR
from ir.expr.yield_ir import YieldIR
from ir.expr.yieldfrom_ir import YieldFromIR
from ir.stmt.assert_ir import AssertIR
from ir.stmt.asyncfunctiondef_ir import AsyncFunctionDefIR
from ir.stmt.break_ir import BreakIR
from ir.stmt.continue_ir import ContinueIR
from ir.stmt.delete_ir import DeleteIR
from ir.stmt.exprstmt_ir import ExprStmtIR
from ir.stmt.for_ir import ForIR
from ir.stmt.global_ir import GlobalIR
from ir.stmt.if_ir import IfIR
from ir.stmt.asyncfor_ir import AsyncForIR
from ir.stmt.while_ir import WhileIR
from ir.stmt.match_ir import MatchCaseIR, MatchIR
from ir.stmt.nonlocal_ir import NonlocalIR
from ir.stmt.pass_ir import PassIR
from ir.stmt.raise_ir import RaiseIR
from ir.stmt.try_ir import ExceptHandlerIR, TryIR
from ir.stmt.trystar_ir import TryStarIR
from ir.stmt.typealias_ir import TypeAliasIR
from ir.stmt.asyncwith_ir import AsyncWithIR
from ir.stmt.with_ir import WithIR, WithItemIR
from .ir_builder import IRBuilder
from ir.program_ir import ProgramIR
from common.span import SourceSpan
from ir.annotation.annotation_ir import AnnotationIR
from ir.annotation.annotationhead_ir import AnnotationHeadIR
from ir.arg.arg_ir import ArgIR, ArgKind
from ir.stmt.return_ir import ReturnIR
from common.operators import Operator
from ir.stmt.augassign_ir import AugAssignIR
from common.kind import ScopeKind, SymbolKind, BindingKind, ImportKind
from ir.pattern.pattern_ir import AsPatternIR, OrPatternIR, StarPatternIR, ClassPatternIR, ValuePatternIR, CapturePatternIR, MappingPatternIR, SequencePatternIR, WildcardPatternIR, SingletonPatternIR, PatternIR


class SemanticBuilder(ast.NodeVisitor):
    def __init__(self, module_name: str, file_path: str):
        self.builder = IRBuilder(module_name=module_name, file_path=file_path)
        self.file_path = file_path
        self.scope_stack = [self.builder.global_scope_id]

    def current_scope(self) -> int:
        return self.scope_stack[-1]

    def current_lexical_scope(self) -> int:
        """Return the nearest module, function, or class scope.

        Block scopes remain on ``scope_stack`` so IR nodes retain their
        control-flow location, but Python blocks do not own name bindings.
        """
        for scope_id in reversed(self.scope_stack):
            scope = next(scope for scope in self.builder.scopes if scope.id == scope_id)
            if scope.kind != ScopeKind.SCOPE_BLOCK:
                return scope_id

        raise RuntimeError("scope stack contains no lexical scope")

    def build(self, tree: ast.AST) -> ProgramIR:
        self.visit(tree)
        return self.builder.finish()

    def visit_Module(self, node: ast.Module):
        self.builder.body = self.lower_statements(node.body)

    def visit_Delete(self, node: ast.Delete):
        return DeleteIR(
            targets=[self.parse_expr(target) for target in node.targets],
            span=SourceSpan.span(node, self.file_path),
        )


    def visit_Assert(self, node: ast.Assert):
        return AssertIR(
            test=self.parse_expr(node.test),
            msg=self.parse_expr(node.msg) if node.msg is not None else None,
            span=SourceSpan.span(node, self.file_path),
        )


    def visit_Raise(self, node: ast.Raise):
        return RaiseIR(
            exc=self.parse_expr(node.exc) if node.exc is not None else None,
            cause=self.parse_expr(node.cause) if node.cause is not None else None,
            span=SourceSpan.span(node, self.file_path),
        )


    def visit_Global(self, node: ast.Global):
        return GlobalIR(
            names=node.names,
            span=SourceSpan.span(node, self.file_path),
        )


    def visit_Nonlocal(self, node: ast.Nonlocal):
        return NonlocalIR(
            names=node.names,
            span=SourceSpan.span(node, self.file_path),
        )


    def visit_Pass(self, node: ast.Pass):
        return PassIR(
            span=SourceSpan.span(node, self.file_path),
        )


    def visit_Break(self, node: ast.Break):
        return BreakIR(
            span=SourceSpan.span(node, self.file_path),
        )


    def visit_Continue(self, node: ast.Continue):
        return ContinueIR(
            span=SourceSpan.span(node, self.file_path),
        )


    def lower_with_item(self, item: ast.withitem):
        return WithItemIR(
            context_expr=self.parse_expr(item.context_expr),
            optional_vars=(
                self.parse_expr(item.optional_vars)
                if item.optional_vars is not None
                else None
            ),
        )


    def visit_With(self, node: ast.With):
        return WithIR(
            items=[self.lower_with_item(item) for item in node.items],
            body=self.lower_statements(node.body),
            type_comment=node.type_comment,
            span=SourceSpan.span(node, self.file_path),
        )


    def lower_except_handler(self, handler: ast.ExceptHandler):
        return ExceptHandlerIR(
            type=(
                self.parse_expr(handler.type)
                if handler.type is not None
                else None
            ),
            name=handler.name,
            body=self.lower_statements(handler.body),
            span=SourceSpan.span(handler, self.file_path),
        )


    def visit_Try(self, node: ast.Try):
        return TryIR(
            body=self.lower_statements(node.body),
            handlers=[
                self.lower_except_handler(handler)
                for handler in node.handlers
            ],
            orelse=self.lower_statements(node.orelse),
            finalbody=self.lower_statements(node.finalbody),
            span=SourceSpan.span(node, self.file_path),
        )


    def visit_TryStar(self, node: ast.TryStar):
        return TryStarIR(
            body=self.lower_statements(node.body),
            handlers=[
                self.lower_except_handler(handler)
                for handler in node.handlers
            ],
            orelse=self.lower_statements(node.orelse),
            finalbody=self.lower_statements(node.finalbody),
            span=SourceSpan.span(node, self.file_path),
        )


    def visit_TypeAlias(self, node: ast.TypeAlias):
        return TypeAliasIR(
            name=self.parse_expr(node.name),
            type_params=[
                self.lower_type_param(param)
                for param in node.type_params
            ],
            value=self.parse_expr(node.value),
            span=SourceSpan.span(node, self.file_path),
        )

    def lower_type_param(self, param: ast.type_param):
        span = SourceSpan.span(param, self.file_path)
        default_value = (
            self.parse_expr(param.default_value)
            if param.default_value is not None
            else None
        )

        if isinstance(param, ast.TypeVar):
            return TypeVarIR(
                name=param.name,
                bound=self.parse_expr(param.bound) if param.bound is not None else None,
                default_value=default_value,
                span=span,
            )
        if isinstance(param, ast.ParamSpec):
            return ParamSpecIR(
                name=param.name,
                default_value=default_value,
                span=span,
            )
        if isinstance(param, ast.TypeVarTuple):
            return TypeVarTupleIR(
                name=param.name,
                default_value=default_value,
                span=span,
            )
        raise NotImplementedError(f"Unsupported type parameter: {type(param).__name__}")

    def lower_statements(self, statements):
        """Lower statements in source order and flatten multi-binding nodes."""
        lowered_statements = []

        def append_lowered(lowered):
            if lowered is None:
                return
            if isinstance(lowered, list):
                for item in lowered:
                    append_lowered(item)
                return
            lowered_statements.append(lowered)

        for statement in statements:
            append_lowered(self.visit(statement))
        return lowered_statements

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
        imports = []
        for alias in node.names:
            module_name = alias.name
            bound_name = alias.asname or module_name.split(".")[0]

            symbol_id = self.builder.get_or_declare_symbol(
                name=bound_name,
                kind=SymbolKind.SYMBOL_MODULE_ALIAS,
                scope_id=self.current_lexical_scope(),
                span=SourceSpan.span(node, self.file_path),
            )

            imports.append(
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
            )
        return imports

    def visit_ImportFrom(self, node: ast.ImportFrom):
        module_name = node.module  # ex. "a" in: from a import b
        imports = []

        for alias in node.names:
            imported_name = alias.name  # ex "b"
            bound_name = alias.asname or alias.name

            symbol_id = self.builder.get_or_declare_symbol(
                name=bound_name,
                kind=SymbolKind.SYMBOL_VARIABLE,
                scope_id=self.current_lexical_scope(),
                span=SourceSpan.span(node, self.file_path),
            )

            imports.append(
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
            )
        return imports

    def visit_FunctionDef(self, node: ast.FunctionDef):
        binding_scope = self.current_scope()
        parent_scope = self.current_lexical_scope()

        fn_symbol_id = self.builder.get_or_declare_symbol(
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

        params = self.lower_params(node.args)

        body = self.lower_statements(node.body)

        self.scope_stack.pop()

        return self.builder.add_function(
            symbol_id=fn_symbol_id,
            name=node.name,
            scope_id=binding_scope,
            body_scope_id=body_scope_id,
            args=params,
            body=body,
            returns=self.lower_annotation(node.returns) if node.returns else None,
            decorator_list=[self.parse_expr(d) for d in node.decorator_list],
            type_comment=node.type_comment,
            type_params=[self.lower_type_param(param) for param in node.type_params],
            span=SourceSpan.span(node, self.file_path),
        )

    def visit_ClassDef(self, node: ast.ClassDef):
        binding_scope = self.current_scope()
        parent_scope = self.current_lexical_scope()

        class_symbol_id = self.builder.get_or_declare_symbol(
            name=node.name,
            kind=SymbolKind.SYMBOL_CLASS,
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
        body = self.lower_statements(node.body)

        self.scope_stack.pop()

        return self.builder.add_class(
            symbol_id=class_symbol_id,
            name=node.name,
            scope_id=binding_scope,
            body_scope_id=class_scope_id,
            body=body,
            bases=[self.parse_expr(base) for base in node.bases],
            keywords=[
                KeywordIR(
                    arg=keyword.arg,
                    value=self.parse_expr(keyword.value),
                    span=SourceSpan.span(keyword, self.file_path),
                )
                for keyword in node.keywords
            ],
            decorator_list=[self.parse_expr(d) for d in node.decorator_list],
            type_params=[self.lower_type_param(param) for param in node.type_params],
            span=SourceSpan.span(node, self.file_path),
        )

    def visit_AnnAssign(self, node: ast.AnnAssign):
        annotation_ir = self.lower_annotation(node.annotation)
        return self.lower_assignment(
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

        body = self.lower_statements(node.body)
        orelse = self.lower_statements(node.orelse)

        self.scope_stack.pop()

        return WhileIR(
            test=test_ir,
            scope_id=self.current_scope(),
            body_scope_id=loop_scope_id,
            body=body,
            orelse=orelse,
            span=SourceSpan.span(node, self.file_path),
        )

    def visit_AsyncFor(self, node: ast.AsyncFor):
        parent_scope = self.current_scope()
        loop_scope_id = self.builder.new_scope(
            name="<for>", 
            kind=ScopeKind.SCOPE_BLOCK, 
            parent_id=parent_scope, 
            span=SourceSpan.span(
                node, self.file_path
            )
        )

        self.declare_assignment_target(node.target, SourceSpan.span(node.target, self.file_path))
        target = self.parse_expr(node.target)
        iter = self.parse_expr(node.iter)

        self.scope_stack.append(loop_scope_id)

        body = self.lower_statements(node.body)
        orelse = self.lower_statements(node.orelse)

        self.scope_stack.pop()

        return AsyncForIR(
            target=target,
            iter=iter,
            body=body,
            orelse=orelse,
            type_comment=node.type_comment,
            span=SourceSpan.span(node, self.file_path),
        )

    def visit_AsyncFunctionDef(self, node: ast.AsyncFunctionDef):
        parent_scope = self.current_scope()

        function_scope_id = self.builder.new_scope(
            name=node.name,
            kind=ScopeKind.SCOPE_FUNCTION,
            parent_id=parent_scope,
            span=SourceSpan.span(node, self.file_path),
        )

        self.scope_stack.append(function_scope_id)

        args = self.lower_params(node.args)
        body = self.lower_statements(node.body)

        self.scope_stack.pop()

        return AsyncFunctionDefIR(
            name=node.name,
            args=args,
            body=body,
            decorator_list=[self.parse_expr(d) for d in node.decorator_list],
            returns=self.parse_expr(node.returns) if node.returns is not None else None,
            type_comment=node.type_comment,
            type_params=[self.lower_type_param(param) for param in node.type_params],
            scope_id=function_scope_id,
            span=SourceSpan.span(node, self.file_path),
        )

    def visit_AsyncWith(self, node: ast.AsyncWith):
        return AsyncWithIR(
            items=[self.lower_with_item(item) for item in node.items],
            body=self.lower_statements(node.body),
            type_comment=node.type_comment,
            span=SourceSpan.span(node, self.file_path)
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

        self.declare_assignment_target(node.target, SourceSpan.span(node.target, self.file_path))
        target_ir = self.parse_expr(node.target)
        iter_ir = self.parse_expr(node.iter)

        self.scope_stack.append(loop_scope_id)

        body = self.lower_statements(node.body)
        orelse = self.lower_statements(node.orelse)

        self.scope_stack.pop()

        return ForIR(
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

    def visit_Match(self, node: ast.Match) -> MatchIR:
        subject = self.parse_expr(node.subject)

        cases = []
        for case in node.cases:
            parent_scope_id = self.current_scope()
            case_scope_id = self.builder.new_scope(
                name="<case>",
                kind=ScopeKind.SCOPE_BLOCK,
                parent_id=parent_scope_id,
                span=SourceSpan.span(node, self.file_path)
            )
            self.scope_stack.append(case_scope_id)

            cases.append(
                MatchCaseIR(
                    scope_id=case_scope_id,
                    pattern=self.parse_pattern(case.pattern),
                    guard=self.parse_expr(case.guard) if case.guard is not None else None,
                    body=self.lower_statements(case.body),
                    span=SourceSpan.span(node, self.file_path)
                )
            )

            self.scope_stack.pop()

        return MatchIR(
            subject=subject,
            cases=cases,
            span=SourceSpan.span(node, self.file_path)
        )

    def parse_pattern(self, pattern: ast.pattern) -> PatternIR:
        span = SourceSpan.span(pattern, self.file_path)

        match pattern:
            case ast.MatchValue(value=value):
                return ValuePatternIR(
                    value=self.parse_expr(value),
                    span=span,
                )

            case ast.MatchSingleton(value=value):
                return SingletonPatternIR(
                    value=value,
                    span=span,
                )

            case ast.MatchSequence(patterns=patterns):
                return SequencePatternIR(
                    patterns=[
                        self.parse_pattern(p)
                        for p in patterns
                    ],
                    span=span,
                )

            case ast.MatchMapping(
                keys=keys,
                patterns=patterns,
                rest=rest,
            ):
                return MappingPatternIR(
                    keys=[
                        self.parse_expr(key)
                        for key in keys
                    ],
                    patterns=[
                        self.parse_pattern(p)
                        for p in patterns
                    ],
                    rest=rest,
                    span=span,
                )

            case ast.MatchClass(
                cls=cls,
                patterns=patterns,
                kwd_attrs=kwd_names,
                kwd_patterns=kwd_patterns,
            ):
                return ClassPatternIR(
                    cls=self.parse_expr(cls),
                    patterns=[
                        self.parse_pattern(p)
                        for p in patterns
                    ],
                    kwd_attrs=kwd_names,
                    kwd_patterns=[
                        self.parse_pattern(p)
                        for p in kwd_patterns
                    ],
                    span=span,
                )

            case ast.MatchStar(name=name):
                return StarPatternIR(
                    name=name,
                    span=span,
                )

            case ast.MatchAs(
                pattern=inner_pattern,
                name=name,
            ):
                if inner_pattern is None and name is not None:
                    return CapturePatternIR(
                        name=name,
                        span=span,
                    )

                if inner_pattern is None and name is None:
                    return WildcardPatternIR(
                        span=span,
                    )

                if inner_pattern is not None and name is not None:
                    return AsPatternIR(
                        pattern=self.parse_pattern(inner_pattern),
                        name=name,
                        span=span,
                    )

                raise ValueError(
                    f"Unexpected MatchAs: {ast.dump(pattern)}"
                )

            case ast.MatchOr(patterns=patterns):
                return OrPatternIR(
                    patterns=[
                        self.parse_pattern(p)
                        for p in patterns
                    ],
                    span=span,
                )

            case _:
                raise NotImplementedError(
                    f"Unsupported pattern: {ast.dump(pattern)}"
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
        body = self.lower_statements(node.body)
        self.scope_stack.pop()

        self.scope_stack.append(else_scope_id)
        orelse = self.lower_statements(node.orelse)
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
        return [
            self.lower_assignment(
                target=target,
                value=node.value,
                kind=BindingKind.BINDING_ASSIGN,
                annotation=None,
                span=SourceSpan.span(node=node, file_path=self.file_path),
            )
            for target in node.targets
        ]

    def lower_assignment(self, target: ast.AST, value: ast.AST, kind: str, annotation: AnnotationIR, span: SourceSpan):
        if isinstance(target, ast.Name):  # x = 5
            symbol_id = self.builder.get_or_declare_symbol(
                name=target.id,
                kind=SymbolKind.SYMBOL_VARIABLE,
                scope_id=self.current_lexical_scope(),
                span=span,
            )

            value_ir = self.parse_expr(value) if value is not None else None

            return self.builder.add_assign(
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

            return [
                self.lower_assignment(
                    target=left,
                    value=right,
                    kind=kind,
                    annotation=annotation,
                    span=span,
                )
                for left, right in zip(target.elts, value.elts)
            ]

    def declare_assignment_target(self, target: ast.AST, span: SourceSpan):
        """Declare names bound by targets which are not represented by BindingIR."""
        if isinstance(target, ast.Name):
            self.builder.get_or_declare_symbol(
                name=target.id,
                kind=SymbolKind.SYMBOL_VARIABLE,
                scope_id=self.current_lexical_scope(),
                span=span,
            )
            return

        if isinstance(target, (ast.Tuple, ast.List)):
            for element in target.elts:
                self.declare_assignment_target(element, span)

    def lower_single_param(
        self,
        arg: ast.arg,
        kind: ArgKind,
        default: ExprIR | None,
    ) -> ArgIR:
        symbol_id = self.builder.declare_symbol(
            name=arg.arg,
            kind=SymbolKind.SYMBOL_PARAM,
            scope_id=self.current_scope(),
            span=SourceSpan.span(arg, self.file_path),
        )

        return ArgIR(
            symbol_id=symbol_id,
            arg=arg.arg,
            kind=kind,
            annotation=(
                self.lower_annotation(arg.annotation)
                if arg.annotation is not None
                else None
            ),
            default=default,
            span=SourceSpan.span(arg, self.file_path),
        )

    def lower_params(self, args: ast.arguments) -> list[ArgIR]:
        """lowers ast.arguments[posonlyargs, args, vararg, kwonlyargs, kw_defaults, kwarg, defaults]"""
        params = []

        # !! defaults are right-aligned, ex: params[a, b=1, c=2] -> defaults[1, 2]
        positional = args.posonlyargs + args.args
        defaults = args.defaults
        default_start = len(positional) - len(defaults)  # for above example this gives 1

        # first parse the posonly, then move to the posOrKeyword
        # there are N posonlyargs in order, then there are M posorkeyword
        for i, arg in enumerate(positional):
            if i < len(positional):
                kind = ArgKind.POSITIONAL_ONLY
            else:
                kind = ArgKind.POSITIONAL_OR_KEYWORD

            # resolve defaults, start with None, then resolve_expr existing defaults
            # recall defaults can only occur after non-defaults
            default = None
            if i >= default_start:
                default = self.parse_expr(defaults[i - default_start])

            params.append(self.lower_single_param(arg, kind, default))

        # vararg -> ...,  *d  , ...
        if args.vararg is not None:
            params.append(
                self.lower_single_param(
                    arg=args.vararg, 
                    kind=ArgKind.VAR_POSITIONAL,
                    default=None
                )
            )

        # can only come after vararg, has either None default or a default is provided
        # ex -> *d, e, f=2 -> kwonlyargs[e, f] , kw_defaults[None, 2]
        for arg, _default in zip(args.kwonlyargs, args.kw_defaults):
            default = self.parse_expr(_default) if default is not None else None
            params.append(self.lower_single_param(arg=arg, kind=ArgKind.KEYWORD_ONLY, default=default))

        # **arg
        if args.kwarg is not None:
            params.append(
                self.lower_single_param(
                    arg=args.kwarg,
                    kind=ArgKind.VAR_KEYWORD,
                    default=None,
                )
            )

        return params

    def parse_comp(self, comp: ast.comprehension) -> CompIR:
        return CompIR(
            target=self.parse_expr(comp.target),
            iter=self.parse_expr(comp.iter),
            ifs=[self.parse_expr(cond) for cond in comp.ifs],
            is_async=bool(comp.is_async),
            span=SourceSpan.span(comp, self.file_path),
        )

    def lower_formatted_value(
        self,
        value: ast.FormattedValue | ast.Constant,
    ):
        if isinstance(value, ast.FormattedValue):
            return FormattedValueIR(
                value=self.parse_expr(value.value),
                conversion=Conversion(value.conversion),
                format_spec=(  # may change to JoinedStr
                    self.parse_expr(value.format_spec)
                    if value.format_spec is not None
                    else None
                ),
                span=SourceSpan.span(value, self.file_path),
            )

        if isinstance(value, ast.Constant):
            return self.parse_expr(value)

    def parse_expr(self, node: ast.AST):
        """
        Recursive function to parse RHS of AnnAssign, Assign
        """
        if isinstance(node, ast.Constant):
            if isinstance(node.value, bool):
                return BooleanIR(
                    node.value,
                    span=SourceSpan.span(node, self.file_path)
                )

            if isinstance(node.value, int):
                return IntegerIR(
                    node.value,
                    span=SourceSpan.span(node, self.file_path)
                )

            if isinstance(node.value, float):
                return FloatIR(
                    node.value,
                    span=SourceSpan.span(node, self.file_path)
                )

            if isinstance(node.value, str):
                return StringIR(
                    node.value,
                    span=SourceSpan.span(node, self.file_path)
                )

            if isinstance(node.value, bytes):
                return BytesIR(
                    value=node.value,
                    span=SourceSpan.span(node, self.file_path),
                )

            # could use "complex" but Rust doesn't support it, this is the same thing
            if isinstance(node.value, complex):
                return ComplexIR(
                    real=node.value.real,
                    imag=node.value.imag,
                    span=SourceSpan.span(node, self.file_path),
                )

            if node.value is Ellipsis:
                return EllipsisIR(
                    value=node.value,
                    span=SourceSpan.span(node, self.file_path),
                )

            # TODO double check
            if node.value is None:
                return NoneIR(
                    value=node.value,
                    span=SourceSpan.span(node, self.file_path)
                )

            raise NotImplementedError(f"Unsupported constant: {node.value!r}")

        if isinstance(node, ast.IfExp):
            return IfExpIR(
                test=self.parse_expr(node.test),
                body=self.parse_expr(node.body),
                orelse=self.parse_expr(node.orelse),
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.JoinedStr):
            return JoinedStrIR(
                values=[self.lower_formatted_value(value) for value in node.values],
                span=SourceSpan.span(node, self.file_path)
            )

        if isinstance(node, ast.ListComp):
            return ListCompIR(
                elt=self.parse_expr(node.elt),
                generators=[self.parse_comp(c) for c in node.generators],
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.SetComp):
            return SetCompIR(
                elt=self.parse_expr(node.elt),
                generators=[self.parse_comp(c) for c in node.generators],
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.DictComp):
            return DictCompIR(
                key=self.parse_expr(node.key),
                value=self.parse_expr(node.value),
                generators=[self.parse_comp(c) for c in node.generators],
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.Starred):
            return StarredIR(
                value=self.parse_expr(node.value),
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.Lambda):
            parent_scope = self.current_scope()

            lambda_scope_id = self.builder.new_scope(
                parent_id=parent_scope,
                name="<lambda>",
                kind=ScopeKind.SCOPE_FUNCTION,
                span=SourceSpan.span(node, self.file_path),
            )

            self.scope_stack.append(lambda_scope_id)

            # The semantic ArgIR also carries the normalized default and parameter kind.
            args = self.lower_params(node.args)

            body = self.parse_expr(node.body)

            self.scope_stack.pop()

            return LambdaIR(
                args=args,
                body=body,
                scope_id=lambda_scope_id,
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.NamedExpr):
            return NamedExprIR(
                target=self.parse_expr(node.target),
                value=self.parse_expr(node.value),
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.Name):
            return NameIR(
                id=node.id,
                use_scope_id=self.current_scope(),
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.Set):
            return SetIR(
                elts=[self.parse_expr(element) for element in node.elts],
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.Dict):
            return DictIR(
                keys=[
                    self.parse_expr(key) if key is not None else None
                    for key in node.keys
                ],
                values=[self.parse_expr(value) for value in node.values],
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.GeneratorExp):
            return GeneratorExpIR(
                elt=self.parse_expr(node.elt),
                generators=[self.parse_comp(comp) for comp in node.generators],
                span=SourceSpan.span(node, self.file_path)
            )

        if isinstance(node, ast.Attribute):
            return AttributeIR(
                value=self.parse_expr(node.value),
                attr=node.attr,
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.Call):
            return CallIR(
                func=self.parse_expr(node.func),
                args=[self.parse_expr(arg) for arg in node.args],
                keywords=[
                    KeywordIR(
                        arg=kw.arg,
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
                elts=[self.parse_expr(arg) for arg in node.elts],
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
                value=self.parse_expr(node.value),
                slice=self.parse_expr(node.slice),
                span=SourceSpan.span(node, self.file_path)
            )

        if isinstance(node, ast.Tuple):
            return TupleIR(
                elts=tuple([self.parse_expr(arg) for arg in node.elts]),
                span=SourceSpan.span(node, self.file_path)
            )

        if isinstance(node, ast.Slice):
            return SliceIR(
                lower=self.parse_expr(node.lower) if node.lower else None,
                upper=self.parse_expr(node.upper) if node.upper else None,
                step=self.parse_expr(node.step) if node.step else None,
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.Await):
            return AwaitIR(
                value=self.parse_expr(node.value),
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.Yield):
            return YieldIR(
                value=self.parse_expr(node.value) if node.value is not None else None,
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.YieldFrom):
            return YieldFromIR(
                value=self.parse_expr(node.value),
                span=SourceSpan.span(node, self.file_path),
            )

        if isinstance(node, ast.FormattedValue):
            return self.lower_formatted_value(node)

        if isinstance(node, ast.Interpolation):
            return InterpolationIR(
                value=self.parse_expr(node.value),
                str=node.str,
                conversion=Conversion(node.conversion),
                format_spec=(
                    self.parse_expr(node.format_spec)
                    if node.format_spec is not None
                    else None
                ),
                span=SourceSpan.span(node, self.file_path),
            )

        raise NotImplementedError(f"Unsupported expression node: {type(node).__name__}")
