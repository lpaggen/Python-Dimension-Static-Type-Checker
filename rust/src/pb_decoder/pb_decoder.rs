use std::fs;
use std::io;
use std::path::PathBuf;

use crate::ir::expr_ir::ConstantIR;
use crate::ir::nodes::annotation_ir::AnnotationHeadIR;
use crate::ir::nodes::match_ir::MatchCaseIR;
use crate::ir::nodes::pattern_ir::AsPatternIR;
use crate::ir::nodes::pattern_ir::CapturePatternIR;
use crate::ir::nodes::pattern_ir::ClassPatternIR;
use crate::ir::nodes::pattern_ir::MappingPatternIR;
use crate::ir::nodes::pattern_ir::OrPatternIR;
use crate::ir::nodes::pattern_ir::SequencePatternIR;
use crate::ir::nodes::pattern_ir::SingletonPatternIR;
use crate::ir::nodes::pattern_ir::StarPatternIR;
use crate::ir::nodes::pattern_ir::ValuePatternIR;
use crate::ir::nodes::pattern_ir::WildcardPatternIR;
use crate::ir::nodes::scope_ir::ScopeIR;
use crate::ir::nodes::symbol_ir::SymbolIR;
use crate::ir::nodes::*;
use crate::ir::{expr_ir::ExprIR, operator::Operator, span_ir::SourceSpan, stmt_ir::StmtIR};
use crate::pb;

use prost::Message;

pub struct PBDecoder {
    pub path: PathBuf,
}

impl PBDecoder {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn decode_dir(&self) -> Result<Vec<ProgramIR>, Box<dyn std::error::Error>> {
        let mut programs = Vec::new();

        // TODO multithreading, should be straightforward

        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let bytes = fs::read(&path)?;
                let pb_program = pb::ProgramIr::decode(bytes.as_slice()).map_err(|error| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "failed to decode protobuf file '{}': {error}",
                            path.display()
                        ),
                    )
                })?;

                let mut decls: Vec<DeclIR> = Vec::new();

                for (index, decl) in pb_program.decls.iter().enumerate() {
                    let decl_ir = Self::convert_decl(decl).map_err(|error| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "failed to decode declaration {index} in protobuf file '{}': {error}",
                                path.display()
                            ),
                        )
                    })?;
                    decls.push(decl_ir);
                }

                let scopes = pb_program.scopes.iter().map(Self::convert_scope).collect();

                let symbols = pb_program
                    .symbols
                    .iter()
                    .map(Self::convert_symbol)
                    .collect();

                let imports = pb_program
                    .imports
                    .iter()
                    .map(Self::convert_import)
                    .collect();

                let body = pb_program
                    .body
                    .iter()
                    .map(Self::convert_stmt)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|error| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "failed to decode module body in protobuf file '{}': {error}",
                                path.display()
                            ),
                        )
                    })?;

                programs.push(ProgramIR {
                    module_name: pb_program.module_name,
                    file_path: pb_program.file_path,
                    scopes,
                    symbols,
                    imports,
                    decls,
                    body,
                });
            }
        }

        Ok(programs)
    }

    fn convert_scope(scope: &pb::ScopeIr) -> ScopeIR {
        let span = Self::convert_optional_span(&scope.span);
        ScopeIR {
            id: scope.id,
            parent_id: scope.parent_id,
            name: scope.name.clone(),
            kind: crate::ir::nodes::scope_ir::ScopeKind::from(scope.kind),
            span,
        }
    }

    fn convert_symbol(symbol: &pb::SymbolIr) -> SymbolIR {
        let span = Self::convert_optional_span(&symbol.span);
        SymbolIR {
            id: symbol.id,
            name: symbol.name.clone(),
            kind: crate::ir::nodes::symbol_ir::SymbolKind::from(symbol.kind),
            scope_id: symbol.scope_id,
            span,
        }
    }

    fn convert_import(import: &pb::ImportIr) -> ImportIR {
        ImportIR {
            id: import.id,
            local_symbol_id: import.local_symbol_id,
            scope_id: import.scope_id,
            kind: ImportKind::from(import.kind),
            module_name: import.module_name.clone(),
            imported_name: import.imported_name.clone(),
            alias: import.alias.clone(),
            relative_level: import.relative_level,
            span: Self::convert_optional_span(&import.span),
        }
    }

    fn convert_decl(decl: &pb::DeclIr) -> Result<DeclIR, Box<dyn std::error::Error>> {
        match &decl.kind {
            Some(pb::decl_ir::Kind::Binding(binding)) => {
                let binding_ir = Self::convert_binding(binding)?;
                Ok(DeclIR::Binding(binding_ir))
            }

            Some(pb::decl_ir::Kind::Function(function)) => {
                let function_ir = Self::convert_function(function)?;
                Ok(DeclIR::Function(function_ir))
            }

            Some(pb::decl_ir::Kind::ClassDecl(class_decl)) => {
                let class_ir = Self::convert_class(class_decl)?;
                Ok(DeclIR::Class(class_ir))
            }

            None => Err("empty DeclIR".into()),
        }
    }

    fn convert_binding(binding: &pb::BindingIr) -> Result<BindingIR, Box<dyn std::error::Error>> {
        let value = match &binding.value {
            Some(value) => Some(Box::new(Self::convert_expr(value)?)),
            // An annotated declaration such as `name: Type` has no value.
            None => None,
        };

        let span = Self::convert_optional_span(&binding.span);

        let annotation = match &binding.annotation {
            Some(annotation) => Some(Self::convert_annotation(annotation)?),
            None => None,
        };

        Ok(BindingIR {
            id: binding.id,
            target_id: binding.target_id,
            annotation: annotation,
            kind: crate::ir::nodes::binding_ir::BindingKind::from(binding.kind), // TODO util to convert back to enum at some stage ?
            value: value,
            scope_id: binding.scope_id,
            span: span,
        })
    }

    fn convert_annotation(
        annotation: &pb::AnnotationIr,
    ) -> Result<AnnotationIR, Box<dyn std::error::Error>> {
        let head = match &annotation.head {
            Some(head) => Self::convert_annotation_head(head)?,
            None => return Err("missing annotation head".into()),
        };

        let mut args = Vec::new();

        for arg in &annotation.args {
            args.push(Self::convert_expr(arg)?);
        }

        Ok(AnnotationIR { head, args })
    }

    fn convert_annotation_head(
        head: &pb::AnnotationHeadIr,
    ) -> Result<AnnotationHeadIR, Box<dyn std::error::Error>> {
        let mut attrs: Vec<String> = Vec::new();

        for attr in &head.attrs {
            attrs.push(attr.clone());
        }

        let scope_id = match head.scope_id {
            Some(scope_id) => scope_id,
            None => return Err("annotation head has no scope_id".into()),
        };

        let span = Self::convert_optional_span(&head.span);

        Ok(AnnotationHeadIR {
            root: head.root.clone(),
            attrs,
            scope_id,
            span,
        })
    }

    fn convert_param(param: &pb::ArgIr) -> Result<ArgIR, Box<dyn std::error::Error>> {
        Ok(ArgIR {
            symbol_id: param.symbol_id,
            arg: param.arg.clone(),

            // TODO make cleaner, this makes sense but it's inconsistent with the rest
            kind: functiondef_ir::ArgKind::try_from(param.kind)?,

            annotation: match &param.annotation {
                Some(annotation) => Some(Self::convert_annotation(annotation)?),
                None => None,
            },

            default: match &param.default {
                Some(default) => Some(Box::new(Self::convert_expr(default)?)),
                // Most parameters do not have a default value.
                None => None,
            },

            span: Self::convert_optional_span(&param.span),
        })
    }

    fn convert_function(
        function: &pb::FunctionDefIr,
    ) -> Result<FunctionDefIR, Box<dyn std::error::Error>> {
        let mut stmts: Vec<StmtIR> = Vec::new();
        let mut params: Vec<ArgIR> = Vec::new();
        let mut decorators: Vec<ExprIR> = Vec::new();

        for stmt in &function.body {
            let stmt_ir = Self::convert_stmt(stmt)?;
            stmts.push(stmt_ir);
        }

        for param in &function.args {
            let param_ir = Self::convert_param(param)?;
            params.push(param_ir);
        }

        for decorator in &function.decorator_list {
            let decorator_ir = Self::convert_expr(decorator)?;
            decorators.push(decorator_ir);
        }

        let returns = match &function.returns {
            Some(returns) => Some(Self::convert_annotation(returns)?),
            None => None,
        };

        // todo decorators

        Ok(FunctionDefIR {
            id: function.id,
            symbol_id: function.symbol_id,
            name: function.name.clone(),
            scope_id: function.scope_id,
            body_scope_id: function.body_scope_id,
            args: params,
            body: stmts,
            returns: returns,
            decorator_list: decorators,
            type_comment: function.type_comment.clone(),
            type_params: function
                .type_params
                .iter()
                .map(Self::convert_type_param)
                .collect::<Result<_, _>>()?,
            span: Self::convert_optional_span(&function.span),
        })
    }

    fn convert_class(
        class_decl: &pb::ClassDefIr,
    ) -> Result<ClassDefIR, Box<dyn std::error::Error>> {
        let mut body: Vec<StmtIR> = Vec::new();
        let mut bases: Vec<ExprIR> = Vec::new();
        let mut decorators: Vec<ExprIR> = Vec::new();

        for stmt in &class_decl.body {
            let stmt_ir = Self::convert_stmt(stmt)?;
            body.push(stmt_ir);
        }

        for expr in &class_decl.bases {
            let expr_ir = Self::convert_expr(expr)?;
            bases.push(expr_ir);
        }

        for expr in &class_decl.decorator_list {
            let expr_ir = Self::convert_expr(expr)?;
            decorators.push(expr_ir);
        }

        Ok(ClassDefIR {
            id: class_decl.id,
            symbol_id: class_decl.symbol_id,
            name: class_decl.name.clone(),
            scope_id: class_decl.scope_id,
            body_scope_id: class_decl.body_scope_id,
            body: body,
            bases: bases,
            keywords: class_decl
                .keywords
                .iter()
                .map(Self::convert_keyword)
                .collect::<Result<_, _>>()?,
            decorator_list: decorators,
            type_params: class_decl
                .type_params
                .iter()
                .map(Self::convert_type_param)
                .collect::<Result<_, _>>()?,
            span: Self::convert_optional_span(&class_decl.span),
        })
    }

    fn convert_keyword(
        keyword: &pb::KeywordArgIr,
    ) -> Result<KeywordIR, Box<dyn std::error::Error>> {
        let value = keyword.value.as_ref().ok_or("keyword has no value")?;
        Ok(KeywordIR {
            arg: keyword.arg.clone(),
            value: Box::new(Self::convert_expr(value)?),
            span: Self::convert_optional_span(&keyword.span),
        })
    }

    fn convert_type_param(
        param: &pb::TypeParamIr,
    ) -> Result<TypeParamIR, Box<dyn std::error::Error>> {
        match param.kind.as_ref().ok_or("type parameter has no kind")? {
            pb::type_param_ir::Kind::TypeVar(value) => Ok(TypeParamIR::TypeVar(TypeVarIR {
                name: value.name.clone(),
                bound: value.bound.as_ref().map(Self::convert_expr).transpose()?,
                default_value: value
                    .default_value
                    .as_ref()
                    .map(Self::convert_expr)
                    .transpose()?,
                span: Self::convert_optional_span(&value.span),
            })),
            pb::type_param_ir::Kind::ParamSpec(value) => Ok(TypeParamIR::ParamSpec(ParamSpecIR {
                name: value.name.clone(),
                default_value: value
                    .default_value
                    .as_ref()
                    .map(Self::convert_expr)
                    .transpose()?,
                span: Self::convert_optional_span(&value.span),
            })),
            pb::type_param_ir::Kind::TypeVarTuple(value) => {
                Ok(TypeParamIR::TypeVarTuple(TypeVarTupleIR {
                    name: value.name.clone(),
                    default_value: value
                        .default_value
                        .as_ref()
                        .map(Self::convert_expr)
                        .transpose()?,
                    span: Self::convert_optional_span(&value.span),
                }))
            }
        }
    }

    fn convert_pattern(pattern: &pb::PatternIr) -> Result<PatternIR, Box<dyn std::error::Error>> {
        match &pattern.kind {
            Some(pb::pattern_ir::Kind::ValuePattern(valuepattern_ir)) => {
                let value = match &valuepattern_ir.value {
                    Some(value) => Self::convert_expr(value)?,
                    None => return Err("value pattern has no value".into()),
                };

                let span = Self::convert_optional_span(&valuepattern_ir.span);

                Ok(PatternIR::ValuePattern(ValuePatternIR { value, span }))
            }

            Some(pb::pattern_ir::Kind::AsPattern(aspattern_ir)) => {
                let inner_pattern = match aspattern_ir.pattern.as_deref() {
                    Some(pattern) => Self::convert_pattern(pattern)?,
                    None => return Err("as pattern has no inner pattern".into()),
                };

                let span = Self::convert_optional_span(&aspattern_ir.span);

                Ok(PatternIR::AsPattern(AsPatternIR {
                    pattern: Box::new(inner_pattern),
                    name: aspattern_ir.name.clone(),
                    span,
                }))
            }

            Some(pb::pattern_ir::Kind::SequencePattern(sequencepattern_ir)) => {
                let mut patterns: Vec<PatternIR> = Vec::new();

                for pattern in &sequencepattern_ir.patterns {
                    let pattern = Self::convert_pattern(pattern)?;
                    patterns.push(pattern);
                }

                let span = Self::convert_optional_span(&sequencepattern_ir.span);

                Ok(PatternIR::SequencePattern(SequencePatternIR {
                    patterns,
                    span,
                }))
            }

            Some(pb::pattern_ir::Kind::SingletonPattern(singletonpattern_ir)) => {
                let value = pb::SingletonValue::try_from(singletonpattern_ir.value)?;

                let value = match value {
                    pb::SingletonValue::SingletonNone => None,
                    pb::SingletonValue::SingletonTrue => Some(true),
                    pb::SingletonValue::SingletonFalse => Some(false),

                    _ => {
                        return Err("singleton pattern has unspecified value".into());
                    }
                };

                let span = Self::convert_optional_span(&singletonpattern_ir.span);

                Ok(PatternIR::SingletonPattern(SingletonPatternIR {
                    value,
                    span,
                }))
            }

            Some(pb::pattern_ir::Kind::MappingPattern(mappingpattern_ir)) => {
                let mut keys = Vec::new();
                for key in &mappingpattern_ir.keys {
                    keys.push(Self::convert_expr(key)?);
                }

                let mut patterns = Vec::new();
                for pattern in &mappingpattern_ir.patterns {
                    patterns.push(Self::convert_pattern(pattern)?);
                }

                let span = Self::convert_optional_span(&mappingpattern_ir.span);

                let rest = mappingpattern_ir.rest.clone();

                Ok(PatternIR::MappingPattern(MappingPatternIR {
                    keys,
                    patterns,
                    rest,
                    span,
                }))
            }

            Some(pb::pattern_ir::Kind::ClassPattern(classpattern_ir)) => {
                let cls = match &classpattern_ir.cls {
                    Some(cls) => Self::convert_expr(cls)?,
                    None => return Err("class pattern has no class expression".into()),
                };

                let mut patterns = Vec::new();
                for pattern in &classpattern_ir.patterns {
                    patterns.push(Self::convert_pattern(pattern)?);
                }

                let mut kwd_patterns = Vec::new();
                for pattern in &classpattern_ir.kwd_patterns {
                    kwd_patterns.push(Self::convert_pattern(pattern)?);
                }

                let span = Self::convert_optional_span(&classpattern_ir.span);

                Ok(PatternIR::ClassPattern(ClassPatternIR {
                    cls,
                    patterns,
                    kwd_attrs: classpattern_ir.kwd_attrs.clone(),
                    kwd_patterns,
                    span,
                }))
            }

            Some(pb::pattern_ir::Kind::StarPattern(starpattern_ir)) => {
                let span = Self::convert_optional_span(&starpattern_ir.span);

                Ok(PatternIR::StarPattern(StarPatternIR {
                    name: starpattern_ir.name.clone(),
                    span,
                }))
            }

            Some(pb::pattern_ir::Kind::CapturePattern(capturepattern_ir)) => {
                let span = Self::convert_optional_span(&capturepattern_ir.span);

                Ok(PatternIR::CapturePattern(CapturePatternIR {
                    name: capturepattern_ir.name.clone(),
                    span,
                }))
            }

            Some(pb::pattern_ir::Kind::WildcardPattern(wildcardpattern_ir)) => {
                let span = Self::convert_optional_span(&wildcardpattern_ir.span);

                Ok(PatternIR::WildcardPattern(WildcardPatternIR { span }))
            }

            Some(pb::pattern_ir::Kind::OrPattern(orpattern_ir)) => {
                let mut patterns = Vec::new();

                for pattern in &orpattern_ir.patterns {
                    patterns.push(Self::convert_pattern(pattern)?);
                }

                let span = Self::convert_optional_span(&orpattern_ir.span);

                Ok(PatternIR::OrPattern(OrPatternIR { patterns, span }))
            }

            None => Err("pattern has no kind".into()),
        }
    }

    fn convert_match_case(
        case: &pb::MatchCaseIr,
    ) -> Result<MatchCaseIR, Box<dyn std::error::Error>> {
        let mut body: Vec<StmtIR> = Vec::new();

        for stmt in &case.body {
            let stmt_ir = Self::convert_stmt(stmt)?;
            body.push(stmt_ir);
        }

        let span = Self::convert_optional_span(&case.span);

        let guard = match &case.guard {
            Some(guard) => Some(Self::convert_expr(guard)?),
            None => None,
        };

        let pattern = match &case.pattern {
            Some(pattern) => Self::convert_pattern(pattern)?,
            None => return Err("case has no pattern".into()),
        };

        Ok(MatchCaseIR {
            scope_id: case.scope_id,
            pattern,
            guard,
            body,
            span,
        })
    }

    fn convert_with_item(item: &pb::WithItemIr) -> Result<WithItemIR, Box<dyn std::error::Error>> {
        let context_expr = item
            .context_expr
            .as_ref()
            .ok_or("with item has no context expression")?;

        let optional_vars = item
            .optional_vars
            .as_ref()
            .map(Self::convert_expr)
            .transpose()?;

        Ok(WithItemIR {
            context_expr: Self::convert_expr(context_expr)?,
            optional_vars,
        })
    }

    fn convert_except_handler(
        handler: &pb::ExceptHandlerIr,
    ) -> Result<ExceptHandlerIR, Box<dyn std::error::Error>> {
        let exception_type = handler
            .r#type
            .as_ref()
            .map(Self::convert_expr)
            .transpose()?;

        let body = handler
            .body
            .iter()
            .map(Self::convert_stmt)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ExceptHandlerIR {
            exception_type,
            name: handler.name.clone(),
            body,
            span: Self::convert_optional_span(&handler.span),
        })
    }

    fn convert_stmt(stmt: &pb::StmtIr) -> Result<StmtIR, Box<dyn std::error::Error>> {
        match &stmt.kind {
            Some(pb::stmt_ir::Kind::DeleteStmt(delete_ir)) => {
                let targets = delete_ir
                    .targets
                    .iter()
                    .map(Self::convert_expr)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::Delete(DeleteIR {
                    targets,
                    span: Self::convert_optional_span(&delete_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::AssertStmt(assert_ir)) => {
                let test = assert_ir
                    .test
                    .as_ref()
                    .ok_or("assert statement has no test")?;

                let msg = assert_ir.msg.as_ref().map(Self::convert_expr).transpose()?;

                Ok(StmtIR::Assert(AssertIR {
                    test: Self::convert_expr(test)?,
                    msg,
                    span: Self::convert_optional_span(&assert_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::RaiseStmt(raise_ir)) => {
                let exc = raise_ir.exc.as_ref().map(Self::convert_expr).transpose()?;

                let cause = raise_ir
                    .cause
                    .as_ref()
                    .map(Self::convert_expr)
                    .transpose()?;

                Ok(StmtIR::Raise(RaiseIR {
                    exc,
                    cause,
                    span: Self::convert_optional_span(&raise_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::GlobalStmt(global_ir)) => Ok(StmtIR::Global(GlobalIR {
                names: global_ir.names.clone(),
                span: Self::convert_optional_span(&global_ir.span),
            })),

            Some(pb::stmt_ir::Kind::NonlocalStmt(nonlocal_ir)) => {
                Ok(StmtIR::Nonlocal(NonlocalIR {
                    names: nonlocal_ir.names.clone(),
                    span: Self::convert_optional_span(&nonlocal_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::PassStmt(pass_ir)) => Ok(StmtIR::Pass(PassIR {
                span: Self::convert_optional_span(&pass_ir.span),
            })),

            Some(pb::stmt_ir::Kind::BreakStmt(break_ir)) => Ok(StmtIR::Break(BreakIR {
                span: Self::convert_optional_span(&break_ir.span),
            })),

            Some(pb::stmt_ir::Kind::ContinueStmt(continue_ir)) => {
                Ok(StmtIR::Continue(ContinueIR {
                    span: Self::convert_optional_span(&continue_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::WithStmt(with_ir)) => {
                let items = with_ir
                    .items
                    .iter()
                    .map(Self::convert_with_item)
                    .collect::<Result<Vec<_>, _>>()?;

                let body = with_ir
                    .body
                    .iter()
                    .map(Self::convert_stmt)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::With(WithIR {
                    items,
                    body,
                    type_comment: with_ir.type_comment.clone(),
                    span: Self::convert_optional_span(&with_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::AsyncWith(async_with_ir)) => {
                let items = async_with_ir
                    .items
                    .iter()
                    .map(Self::convert_with_item)
                    .collect::<Result<Vec<_>, _>>()?;

                let body = async_with_ir
                    .body
                    .iter()
                    .map(Self::convert_stmt)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::AsyncWith(AsyncWithIR {
                    items,
                    body,
                    type_comment: async_with_ir.type_comment.clone(),
                    span: Self::convert_optional_span(&async_with_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::TryStmt(try_ir)) => {
                let body = try_ir
                    .body
                    .iter()
                    .map(Self::convert_stmt)
                    .collect::<Result<Vec<_>, _>>()?;

                let handlers = try_ir
                    .handlers
                    .iter()
                    .map(Self::convert_except_handler)
                    .collect::<Result<Vec<_>, _>>()?;

                let orelse = try_ir
                    .orelse
                    .iter()
                    .map(Self::convert_stmt)
                    .collect::<Result<Vec<_>, _>>()?;

                let finalbody = try_ir
                    .finalbody
                    .iter()
                    .map(Self::convert_stmt)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::Try(TryIR {
                    body,
                    handlers,
                    orelse,
                    finalbody,
                    span: Self::convert_optional_span(&try_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::TryStarStmt(try_ir)) => {
                let body = try_ir
                    .body
                    .iter()
                    .map(Self::convert_stmt)
                    .collect::<Result<Vec<_>, _>>()?;

                let handlers = try_ir
                    .handlers
                    .iter()
                    .map(Self::convert_except_handler)
                    .collect::<Result<Vec<_>, _>>()?;

                let orelse = try_ir
                    .orelse
                    .iter()
                    .map(Self::convert_stmt)
                    .collect::<Result<Vec<_>, _>>()?;

                let finalbody = try_ir
                    .finalbody
                    .iter()
                    .map(Self::convert_stmt)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::TryStar(TryStarIR {
                    body,
                    handlers,
                    orelse,
                    finalbody,
                    span: Self::convert_optional_span(&try_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::AsyncFor(async_for_ir)) => {
                let target = async_for_ir
                    .target
                    .as_ref()
                    .ok_or("async for has no target")?;

                let iterable = async_for_ir
                    .iter
                    .as_ref()
                    .ok_or("async for has no iterable")?;

                let body = async_for_ir
                    .body
                    .iter()
                    .map(Self::convert_stmt)
                    .collect::<Result<Vec<_>, _>>()?;

                let orelse = async_for_ir
                    .orelse
                    .iter()
                    .map(Self::convert_stmt)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::AsyncFor(AsyncForIR {
                    target: Self::convert_expr(target)?,
                    iter: Self::convert_expr(iterable)?,
                    body,
                    orelse,
                    type_comment: async_for_ir.type_comment.clone(),
                    span: Self::convert_optional_span(&async_for_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::TypeAlias(type_alias_ir)) => {
                let name = type_alias_ir
                    .name
                    .as_ref()
                    .ok_or("type alias has no name")?;

                let value = type_alias_ir
                    .value
                    .as_ref()
                    .ok_or("type alias has no value")?;

                let type_params = type_alias_ir
                    .type_params
                    .iter()
                    .map(Self::convert_type_param)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::TypeAlias(TypeAliasIR {
                    name: Self::convert_expr(name)?,
                    type_params,
                    value: Self::convert_expr(value)?,
                    span: Self::convert_optional_span(&type_alias_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::AsyncFunctionDef(function_ir)) => {
                let args = function_ir
                    .args
                    .iter()
                    .map(Self::convert_param)
                    .collect::<Result<Vec<_>, _>>()?;

                let body = function_ir
                    .body
                    .iter()
                    .map(Self::convert_stmt)
                    .collect::<Result<Vec<_>, _>>()?;

                let decorators = function_ir
                    .decorator_list
                    .iter()
                    .map(Self::convert_expr)
                    .collect::<Result<Vec<_>, _>>()?;

                let returns = function_ir
                    .returns
                    .as_ref()
                    .map(Self::convert_expr)
                    .transpose()?;

                let type_params = function_ir
                    .type_params
                    .iter()
                    .map(Self::convert_type_param)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::AsyncFunctionDef(AsyncFunctionDefIR {
                    name: function_ir.name.clone(),
                    args,
                    body,
                    decorator_list: decorators,
                    returns,
                    type_comment: function_ir.type_comment.clone(),
                    scope_id: function_ir.scope_id,
                    type_params,
                    span: Self::convert_optional_span(&function_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::Match(match_ir)) => {
                let mut cases: Vec<MatchCaseIR> = Vec::new();

                for case in &match_ir.cases {
                    let case_ir = Self::convert_match_case(case)?;
                    cases.push(case_ir);
                }

                let span = Self::convert_optional_span(&match_ir.span);

                let subject = match &match_ir.subject {
                    Some(subject) => Box::new(Self::convert_expr(subject)?),
                    None => return Err("match statement has no subject".into()),
                };

                Ok(StmtIR::Match(MatchIR {
                    subject,
                    cases,
                    span,
                }))
            }

            Some(pb::stmt_ir::Kind::Binding(binding)) => Ok(StmtIR::Binding(BindingIR {
                id: binding.id,
                target_id: binding.target_id,

                annotation: match &binding.annotation {
                    Some(annotation) => Some(Self::convert_annotation(annotation)?),
                    None => None,
                },

                kind: crate::ir::nodes::binding_ir::BindingKind::from(binding.kind),

                value: match &binding.value {
                    Some(value) => Some(Box::new(Self::convert_expr(value)?)),
                    None => None,
                },

                scope_id: binding.scope_id,

                span: Self::convert_optional_span(&binding.span),
            })),

            Some(pb::stmt_ir::Kind::AugAssign(aug)) => {
                let target = match &aug.target {
                    Some(target) => Box::new(Self::convert_expr(target)?),
                    None => return Err("augmented assignment has no target".into()),
                };

                let value = match &aug.value {
                    Some(value) => Box::new(Self::convert_expr(value)?),
                    None => return Err("augmented assignment has no value".into()),
                };

                let span = Self::convert_optional_span(&aug.span);

                Ok(StmtIR::AugAssign(AugAssignIR {
                    target,
                    op: Operator::from(aug.op),
                    value,
                    span,
                }))
            }

            Some(pb::stmt_ir::Kind::ReturnStmt(ret)) => {
                // A bare `return` is valid, so None remains None.
                let value = match &ret.value {
                    Some(value) => Some(Box::new(Self::convert_expr(value)?)),
                    None => None,
                };

                let span = Self::convert_optional_span(&ret.span);

                Ok(StmtIR::Return(ReturnIR { value, span }))
            }

            Some(pb::stmt_ir::Kind::ExprStmt(expr_stmt)) => {
                let value = match &expr_stmt.value {
                    Some(value) => Some(Box::new(Self::convert_expr(value)?)),
                    None => return Err("expression statement has no expression".into()),
                };

                let span = Self::convert_optional_span(&expr_stmt.span);

                Ok(StmtIR::ExprStmt(ExprStmtIR { value, span }))
            }

            Some(pb::stmt_ir::Kind::IfStmt(if_stmt)) => {
                let test = match &if_stmt.test {
                    Some(test) => Box::new(Self::convert_expr(test)?),
                    None => return Err("if statement has no test expression".into()),
                };

                let mut body: Vec<StmtIR> = Vec::new();

                for stmt in &if_stmt.body {
                    let stmt = Self::convert_stmt(stmt)?;
                    body.push(stmt);
                }

                let mut orelse: Vec<StmtIR> = Vec::new();

                for stmt in &if_stmt.orelse {
                    let stmt = Self::convert_stmt(stmt)?;
                    orelse.push(stmt);
                }

                let span = Self::convert_optional_span(&if_stmt.span);

                Ok(StmtIR::If(IfIR {
                    test,
                    scope_id: if_stmt.scope_id,
                    else_scope_id: if_stmt.else_scope_id,
                    then_scope_id: if_stmt.then_scope_id,
                    body,
                    orelse,
                    span,
                }))
            }

            Some(pb::stmt_ir::Kind::ForLoop(for_loop)) => {
                let target = match &for_loop.target {
                    Some(target) => Box::new(Self::convert_expr(target)?),
                    None => return Err("for loop has no target".into()),
                };

                let iter = match &for_loop.iter {
                    Some(iter) => Box::new(Self::convert_expr(iter)?),
                    None => return Err("for loop has no iterable expression".into()),
                };

                let mut body: Vec<StmtIR> = Vec::new();

                for stmt in &for_loop.body {
                    let stmt = Self::convert_stmt(stmt)?;
                    body.push(stmt);
                }

                let mut orelse: Vec<StmtIR> = Vec::new();

                for stmt in &for_loop.orelse {
                    let stmt = Self::convert_stmt(stmt)?;
                    orelse.push(stmt);
                }

                let span = Self::convert_optional_span(&for_loop.span);

                Ok(StmtIR::For(ForIR {
                    target,
                    iter,
                    scope_id: for_loop.scope_id,
                    body_scope_id: for_loop.body_scope_id,
                    body,
                    orelse,
                    span,
                }))
            }

            Some(pb::stmt_ir::Kind::WhileLoop(while_loop)) => {
                let test = match &while_loop.test {
                    Some(test) => Box::new(Self::convert_expr(test)?),
                    None => return Err("while loop has no test expression".into()),
                };

                let mut body: Vec<StmtIR> = Vec::new();

                for stmt in &while_loop.body {
                    let stmt = Self::convert_stmt(stmt)?;
                    body.push(stmt);
                }

                let mut orelse: Vec<StmtIR> = Vec::new();

                for stmt in &while_loop.orelse {
                    let stmt = Self::convert_stmt(stmt)?;
                    orelse.push(stmt);
                }

                let span = Self::convert_optional_span(&while_loop.span);

                Ok(StmtIR::While(WhileIR {
                    test,
                    scope_id: while_loop.scope_id,
                    body_scope_id: while_loop.body_scope_id,
                    body,
                    orelse,
                    span,
                }))
            }

            Some(pb::stmt_ir::Kind::ImportStmt(import_stmt)) => {
                let span = Self::convert_optional_span(&import_stmt.span);

                Ok(StmtIR::Import(ImportIR {
                    id: import_stmt.id,
                    local_symbol_id: import_stmt.local_symbol_id,
                    scope_id: import_stmt.scope_id,
                    kind: crate::ir::nodes::ImportKind::from(import_stmt.kind),
                    module_name: import_stmt.module_name.clone(),
                    imported_name: import_stmt.imported_name.clone(),
                    alias: import_stmt.alias.clone(),
                    relative_level: import_stmt.relative_level,
                    span,
                }))
            }

            Some(pb::stmt_ir::Kind::Function(function)) => {
                Ok(StmtIR::Function(Self::convert_function(function)?))
            }

            Some(pb::stmt_ir::Kind::ClassDecl(class_decl)) => {
                Ok(StmtIR::Class(Self::convert_class(class_decl)?))
            }

            None => Err("statement has no kind".into()),
            // _ => Err("unsupported statement kind".into()),
        }
    }

    fn convert_span(span: &pb::SourceSpan) -> SourceSpan {
        SourceSpan {
            file: span.file.clone(),
            lineno: span.lineno,
            col_offset: span.col_offset,
            end_lineno: span.end_lineno,
            end_col_offset: span.end_col_offset,
        }
    }

    fn convert_joined_str(
        spec: &pb::JoinedStrIr,
    ) -> Result<JoinedStrIR, Box<dyn std::error::Error>> {
        Ok(JoinedStrIR {
            values: spec
                .values
                .iter()
                .map(Self::convert_expr)
                .collect::<Result<Vec<_>, _>>()?,
            span: Self::convert_optional_span(&spec.span),
        })
    }

    fn convert_generator(comp: &pb::CompIr) -> Result<CompIR, Box<dyn std::error::Error>> {
        let target: Box<ExprIR> = Box::new(Self::convert_expr(
            comp.target.as_ref().ok_or("comprehension has no target")?,
        )?);

        let iter: Box<ExprIR> = Box::new(Self::convert_expr(
            comp.iter.as_ref().ok_or("comprehension has no iterable")?,
        )?);

        let mut ifs: Vec<ExprIR> = Vec::new();
        for expr in &comp.ifs {
            ifs.push(Self::convert_expr(expr)?);
        }

        let is_async: bool = comp.is_async;

        let span: Option<SourceSpan> = Self::convert_optional_span(&comp.span);

        Ok(CompIR {
            target,
            iter,
            ifs,
            is_async,
            span,
        })
    }

    fn convert_expr(expr: &pb::ExprIr) -> Result<ExprIR, Box<dyn std::error::Error>> {
        match &expr.kind {
            Some(pb::expr_ir::Kind::Identifier(identifier)) => Ok(ExprIR::Name(NameIR {
                id: identifier.id.clone(),
                use_scope_id: identifier.use_scope_id,
                span: Self::convert_optional_span(&identifier.span),
            })),

            Some(pb::expr_ir::Kind::AwaitExpr(await_ir)) => {
                let value = await_ir
                    .value
                    .as_deref()
                    .ok_or("await expression has no value")?;

                Ok(ExprIR::AwaitExpr(AwaitIR {
                    value: Box::new(Self::convert_expr(value)?),
                    span: Self::convert_optional_span(&await_ir.span),
                }))
            }

            Some(pb::expr_ir::Kind::YieldExpr(yield_ir)) => {
                let value = yield_ir
                    .value
                    .as_deref()
                    .map(Self::convert_expr)
                    .transpose()?
                    .map(Box::new);

                Ok(ExprIR::YieldExpr(YieldIR {
                    value,
                    span: Self::convert_optional_span(&yield_ir.span),
                }))
            }

            Some(pb::expr_ir::Kind::YieldFrom(yield_from_ir)) => {
                let value = yield_from_ir
                    .value
                    .as_deref()
                    .ok_or("yield from expression has no value")?;

                Ok(ExprIR::YieldFromExpr(YieldFromIR {
                    value: Box::new(Self::convert_expr(value)?),
                    span: Self::convert_optional_span(&yield_from_ir.span),
                }))
            }

            Some(pb::expr_ir::Kind::TemplateStr(template_ir)) => {
                let values = template_ir
                    .values
                    .iter()
                    .map(Self::convert_expr)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(ExprIR::TemplateStr(TemplateStrIR {
                    values,
                    span: Self::convert_optional_span(&template_ir.span),
                }))
            }

            Some(pb::expr_ir::Kind::Interpolation(interpolation_ir)) => {
                let value = interpolation_ir
                    .value
                    .as_deref()
                    .ok_or("interpolation has no value")?;

                let format_spec = interpolation_ir
                    .format_spec
                    .as_ref()
                    .map(Self::convert_joined_str)
                    .transpose()?;

                Ok(ExprIR::InterpolationExpr(InterpolationIR {
                    value: Box::new(Self::convert_expr(value)?),
                    str: interpolation_ir.str.clone(),
                    conversion: Conversion::try_from(interpolation_ir.conversion)?,
                    format_spec,
                    span: Self::convert_optional_span(&interpolation_ir.span),
                }))
            }

            Some(pb::expr_ir::Kind::FormattedValue(formatted_ir)) => {
                let value = formatted_ir
                    .value
                    .as_deref()
                    .ok_or("formatted value has no value")?;

                let format_spec = formatted_ir
                    .format_spec
                    .as_ref()
                    .map(Self::convert_joined_str)
                    .transpose()?;

                Ok(ExprIR::FormattedValue(FormattedValueIR {
                    value: Box::new(Self::convert_expr(value)?),
                    conversion: Conversion::try_from(formatted_ir.conversion)?,
                    format_spec,
                    span: Self::convert_optional_span(&formatted_ir.span),
                }))
            }

            Some(pb::expr_ir::Kind::JoinedStr(joinedstr_ir)) => {
                let mut values: Vec<ExprIR> = Vec::new();
                for value in &joinedstr_ir.values {
                    values.push(Self::convert_expr(value)?);
                }

                let span = match &joinedstr_ir.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(ExprIR::JoinedStr(JoinedStrIR { values, span }))
            }

            Some(pb::expr_ir::Kind::NamedExpr(named_expr_ir)) => {
                let target = named_expr_ir
                    .target
                    .as_ref()
                    .ok_or("named expression has no target")?;

                let value = match &named_expr_ir.value {
                    Some(value) => Self::convert_expr(value)?,
                    None => return Err("named expression has no value".into()),
                };

                let span = match &named_expr_ir.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(ExprIR::NamedExpr(NamedExprIR {
                    target: Box::new(Self::convert_expr(target)?),
                    value: Box::new(value),
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Starred(starred_ir)) => {
                let value = match &starred_ir.value {
                    Some(value) => Self::convert_expr(value)?,
                    None => return Err("starred expression has no value".into()),
                };

                let span = match &starred_ir.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(ExprIR::StarredExpr(StarredIR {
                    value: Box::new(value),
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Set(set_ir)) => {
                let mut elements: Vec<ExprIR> = Vec::new();

                for element in &set_ir.elts {
                    elements.push(Self::convert_expr(element)?);
                }

                let span = Self::convert_optional_span(&set_ir.span);

                Ok(ExprIR::SetExpr(SetIR {
                    elts: elements,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Dict(dict_ir)) => {
                let keys = dict_ir
                    .keys
                    .iter()
                    .map(|key| key.value.as_ref().map(Self::convert_expr).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                let values = dict_ir
                    .values
                    .iter()
                    .map(Self::convert_expr)
                    .collect::<Result<Vec<_>, _>>()?;

                let span = Self::convert_optional_span(&dict_ir.span);

                Ok(ExprIR::DictExpr(DictIR { keys, values, span }))
            }

            Some(pb::expr_ir::Kind::ListComp(listcomp_ir)) => {
                let elt = match &listcomp_ir.elt {
                    Some(expr) => Self::convert_expr(expr)?,
                    None => return Err("list comprehension expression has no elt".into()),
                };

                let mut generators: Vec<CompIR> = Vec::new();
                for comp in &listcomp_ir.generators {
                    generators.push(Self::convert_generator(comp)?)
                }

                let span = match &listcomp_ir.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(ExprIR::ListComp(ListCompIR {
                    elt: Box::new(elt),
                    generators,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::SetComp(setcomp_ir)) => {
                let elt = match &setcomp_ir.elt {
                    Some(expr) => Self::convert_expr(expr)?,
                    None => return Err("set comprehension expression has no elt".into()),
                };

                let mut generators: Vec<CompIR> = Vec::new();
                for comp in &setcomp_ir.generators {
                    generators.push(Self::convert_generator(comp)?)
                }

                let span = match &setcomp_ir.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(ExprIR::SetComp(SetCompIR {
                    elt: Box::new(elt),
                    generators,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::GeneratorExpr(generatorexpr_ir)) => {
                let elt = match &generatorexpr_ir.elt {
                    Some(expr) => Self::convert_expr(expr)?,
                    None => return Err("generator expression has no elt".into()),
                };

                let mut generators: Vec<CompIR> = Vec::new();
                for comp in &generatorexpr_ir.generators {
                    generators.push(Self::convert_generator(comp)?)
                }

                let span = match &generatorexpr_ir.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(ExprIR::GeneratorExp(GeneratorExpIR {
                    elt: Box::new(elt),
                    generators,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::DictComp(dictcomp_ir)) => {
                let key = match &dictcomp_ir.key {
                    Some(expr) => Self::convert_expr(expr)?,
                    None => return Err("dict comprehension expression has no key".into()),
                };

                let value = match &dictcomp_ir.value {
                    Some(expr) => Self::convert_expr(expr)?,
                    None => return Err("dict comprehension expression has no value".into()),
                };

                let mut generators: Vec<CompIR> = Vec::new();
                for comp in &dictcomp_ir.generators {
                    generators.push(Self::convert_generator(comp)?)
                }

                let span = match &dictcomp_ir.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(ExprIR::DictComp(DictCompIR {
                    key: Box::new(key),
                    value: Box::new(value),
                    generators,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::IfExpr(if_expr_ir)) => {
                let test = match &if_expr_ir.test {
                    Some(expr) => Self::convert_expr(expr)?,
                    None => return Err("if expression has no test".into()),
                };

                let body = match &if_expr_ir.body {
                    Some(expr) => Self::convert_expr(expr)?,
                    None => return Err("if expression has no body".into()),
                };

                let orelse = match &if_expr_ir.orelse {
                    Some(expr) => Self::convert_expr(expr)?,
                    None => return Err("if expression has no else expression".into()),
                };

                let span = match &if_expr_ir.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(ExprIR::IfExp(IfExpIR {
                    test: Box::new(test),
                    body: Box::new(body),
                    orelse: Box::new(orelse),
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Constant(constant)) => match constant.kind.as_ref() {
                Some(pb::constant_ir::Kind::EllipsisLit(ellipsis)) => {
                    Ok(ExprIR::Constant(ConstantIR::EllipsisLit(EllipsisIR {
                        span: Self::convert_optional_span(&ellipsis.span),
                    })))
                }

                Some(pb::constant_ir::Kind::IntegerLit(integer)) => {
                    Ok(ExprIR::Constant(ConstantIR::IntegerLit(IntegerIR {
                        value: integer.value,
                        span: Self::convert_optional_span(&integer.span),
                    })))
                }

                Some(pb::constant_ir::Kind::FloatLit(float_lit)) => {
                    Ok(ExprIR::Constant(ConstantIR::FloatLit(FloatIR {
                        value: float_lit.value,
                        span: Self::convert_optional_span(&float_lit.span),
                    })))
                }

                Some(pb::constant_ir::Kind::StringLit(string_lit)) => {
                    Ok(ExprIR::Constant(ConstantIR::StringLit(StringIR {
                        value: string_lit.value.clone(),
                        span: Self::convert_optional_span(&string_lit.span),
                    })))
                }

                Some(pb::constant_ir::Kind::BoolLit(bool_lit)) => {
                    Ok(ExprIR::Constant(ConstantIR::BooleanLit(BooleanIR {
                        value: bool_lit.value,
                        span: Self::convert_optional_span(&bool_lit.span),
                    })))
                }

                Some(pb::constant_ir::Kind::NoneLit(none_lit)) => {
                    Ok(ExprIR::Constant(ConstantIR::NoneLit(NoneIR {
                        span: Self::convert_optional_span(&none_lit.span),
                    })))
                }

                Some(pb::constant_ir::Kind::ComplexLit(complex_lit)) => {
                    Ok(ExprIR::Constant(ConstantIR::ComplexLit(ComplexIR {
                        real: complex_lit.real,
                        imag: complex_lit.imag,
                        span: Self::convert_optional_span(&complex_lit.span),
                    })))
                }

                Some(pb::constant_ir::Kind::BytesLit(bytes_lit)) => {
                    Ok(ExprIR::Constant(ConstantIR::BytesLit(BytesIR {
                        value: bytes_lit
                            .value
                            .iter()
                            .map(|&x| u8::try_from(x))
                            .collect::<Result<Vec<_>, _>>()?,
                        span: Self::convert_optional_span(&bytes_lit.span),
                    })))
                }

                None => Err("constant has no kind".into()),
            },

            Some(pb::expr_ir::Kind::List(list)) => {
                let mut elements: Vec<ExprIR> = Vec::new();
                for element in &list.elts {
                    let expr_ir: ExprIR = Self::convert_expr(element)?;
                    elements.push(expr_ir);
                }

                Ok(ExprIR::ListExpr(ListIR {
                    elts: elements,
                    span: Self::convert_optional_span(&list.span),
                }))
            }

            Some(pb::expr_ir::Kind::Tuple(tuple)) => {
                let mut elements: Vec<ExprIR> = Vec::new();
                for element in &tuple.elts {
                    let expr_ir: ExprIR = Self::convert_expr(element)?;
                    elements.push(expr_ir);
                }

                Ok(ExprIR::TupleExpr(TupleIR {
                    elts: elements,
                    span: Self::convert_optional_span(&tuple.span),
                }))
            }

            Some(pb::expr_ir::Kind::Call(call)) => {
                let callee: Box<ExprIR> = match &call.func {
                    Some(callee) => Box::new(Self::convert_expr(callee)?),
                    None => {
                        return Err("call has no callee".into());
                    }
                };

                let mut args: Vec<ExprIR> = Vec::new();

                for arg in &call.args {
                    let arg = Self::convert_expr(arg)?;
                    args.push(arg);
                }

                let keywords = call
                    .keywords
                    .iter()
                    .map(Self::convert_keyword)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(ExprIR::Call(CallIR {
                    func: callee,
                    args: args,
                    keywords,
                    span: Self::convert_optional_span(&call.span),
                }))
            }

            Some(pb::expr_ir::Kind::Attribute(attribute)) => {
                let base = match &attribute.value {
                    Some(base) => Box::new(Self::convert_expr(base)?),
                    None => {
                        return Err(Self::missing_expr(
                            "attribute expression",
                            "base",
                            &attribute.span,
                        ));
                    }
                };

                Ok(ExprIR::Attribute(AttributeIR {
                    value: base,
                    attr: attribute.attr.clone(),
                    span: Self::convert_optional_span(&attribute.span),
                }))
            }

            Some(pb::expr_ir::Kind::Binop(binop)) => {
                let operator = Operator::from(binop.op);

                let left = match &binop.left {
                    Some(left) => Box::new(Self::convert_expr(left)?),
                    None => {
                        return Err(Self::missing_expr(
                            "binary expression",
                            "left operand",
                            &binop.span,
                        ));
                    }
                };

                let right = match &binop.right {
                    Some(right) => Box::new(Self::convert_expr(right)?),
                    None => {
                        return Err(Self::missing_expr(
                            "binary expression",
                            "right operand",
                            &binop.span,
                        ));
                    }
                };

                let span = Self::convert_optional_span(&binop.span);

                Ok(ExprIR::BinOpExpr(BinOpIR {
                    left: left,
                    right: right,
                    op: operator,
                    span: span,
                }))
            }

            Some(pb::expr_ir::Kind::Unaryop(unaryop)) => {
                let operator = Operator::from(unaryop.op);

                let operand = match &unaryop.operand {
                    Some(operand) => Box::new(Self::convert_expr(operand)?),
                    None => {
                        return Err(Self::missing_expr(
                            "unary expression",
                            "operand",
                            &unaryop.span,
                        ));
                    }
                };

                let span = Self::convert_optional_span(&unaryop.span);

                Ok(ExprIR::UnaryOpExpr(UnaryOpIR {
                    op: operator,
                    operand: operand,
                    span: span,
                }))
            }

            Some(pb::expr_ir::Kind::Boolop(boolop)) => {
                let mut values = Vec::new();
                for value in &boolop.values {
                    let value_ir = Self::convert_expr(value)?;
                    values.push(value_ir);
                }

                let operator = Operator::from(boolop.op);

                let span = Self::convert_optional_span(&boolop.span);

                Ok(ExprIR::BoolOpExpr(BoolOpIR {
                    values,
                    op: operator,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Compare(compare)) => {
                let left = match &compare.left {
                    Some(left) => Box::new(Self::convert_expr(left)?),
                    None => {
                        return Err(Self::missing_expr(
                            "comparison expression",
                            "left operand",
                            &compare.span,
                        ));
                    }
                };

                let span = Self::convert_optional_span(&compare.span);

                let mut ops: Vec<Operator> = Vec::new();
                for &op in &compare.ops {
                    ops.push(Operator::from(op));
                }

                let mut comparators: Vec<ExprIR> = Vec::new();
                for comparator in &compare.comparators {
                    let comparator = Self::convert_expr(comparator)?;
                    comparators.push(comparator);
                }

                Ok(ExprIR::CompareExpr(CompareIR {
                    left,
                    ops,
                    comparators,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Subscript(subscript)) => {
                let target = match &subscript.value {
                    Some(target) => Box::new(Self::convert_expr(target)?),
                    None => {
                        return Err(Self::missing_expr(
                            "subscript expression",
                            "target",
                            &subscript.span,
                        ));
                    }
                };

                let index = match &subscript.slice {
                    Some(subsript) => Box::new(Self::convert_expr(subsript)?),
                    None => {
                        return Err(Self::missing_expr(
                            "subscript expression",
                            "index",
                            &subscript.span,
                        ));
                    }
                };

                let span = Self::convert_optional_span(&subscript.span);

                Ok(ExprIR::SubscriptExpr(SubscriptIR {
                    value: target,
                    slice: index,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Slice(slice)) => {
                let span = Self::convert_optional_span(&slice.span);

                let upper = match &slice.upper {
                    Some(upper) => Some(Box::new(Self::convert_expr(upper)?)),
                    None => None,
                };

                let lower = match &slice.lower {
                    Some(lower) => Some(Box::new(Self::convert_expr(lower)?)),
                    None => None,
                };

                let step = match &slice.step {
                    Some(step) => Some(Box::new(Self::convert_expr(step)?)),
                    None => None,
                };

                Ok(ExprIR::SliceExpr(SliceIR {
                    lower,
                    upper,
                    step,
                    span,
                }))
            }

            None => {
                return Err("malformed ExprIR: protobuf oneof field 'kind' is unset (the producer emitted an empty expression)".into());
            }
        }
    }

    fn convert_optional_span(span: &Option<pb::SourceSpan>) -> Option<SourceSpan> {
        span.as_ref().map(Self::convert_span)
    }

    fn missing_expr(
        node: &str,
        field: &str,
        span: &Option<pb::SourceSpan>,
    ) -> Box<dyn std::error::Error> {
        let location = match span {
            Some(span) => format!(
                " at {}:{}:{}-{}:{}",
                span.file,
                span.lineno,
                span.col_offset,
                span.end_lineno
                    .map_or_else(|| "?".to_owned(), |value| value.to_string()),
                span.end_col_offset
                    .map_or_else(|| "?".to_owned(), |value| value.to_string())
            ),
            None => " (source span unavailable)".to_owned(),
        };

        format!("malformed {node}{location}: required protobuf field '{field}' is missing").into()
    }
}
