use std::fs;
use std::io;
use std::path::PathBuf;

use crate::ir::expr_ir::ConstantIR;
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
use crate::linker::symbol_ref::SymbolRef;
use crate::pb;

use prost::Message;

pub struct PBDecoder {
    pub path: PathBuf,
}

struct ProgramDecoder {
    program_id: usize,
}

impl PBDecoder {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn decode_dir(&self) -> Result<Vec<ProgramIR>, Box<dyn std::error::Error + Send + Sync>> {
        let paths: Vec<PathBuf> = fs::read_dir(&self.path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .collect();

        let mut handles = Vec::new();

        for (program_id, path) in paths.into_iter().enumerate() {
            handles.push(std::thread::spawn(move || {
                ProgramDecoder {
                    program_id: program_id,
                }
                .decode_file(&path)
            }));
        }

        let mut programs = Vec::new();

        for handle in handles {
            let program = handle.join().map_err(|_| "decoder thread panicked")??;

            programs.push(program);
        }

        Ok(programs)
    }
}

impl ProgramDecoder {
    fn symbol_ref(&self, symbol_id: usize) -> SymbolRef {
        SymbolRef {
            program_id: self.program_id,
            symbol_id,
        }
    }

    fn decode_file(
        &self,
        path: &PathBuf,
    ) -> Result<ProgramIR, Box<dyn std::error::Error + Send + Sync>> {
        let bytes = fs::read(path)?;

        let pb_program = pb::ProgramIr::decode(bytes.as_slice()).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "failed to decode protobuf file '{}': {error}",
                    path.display()
                ),
            )
        })?;

        let scopes = pb_program
            .scopes
            .iter()
            .map(|value| self.convert_scope(value))
            .collect();

        let symbols = pb_program
            .symbols
            .iter()
            .map(|value| self.convert_symbol(value))
            .collect();

        let imports = pb_program
            .imports
            .iter()
            .map(|value| self.convert_import(value))
            .collect();

        let body = pb_program
            .body
            .iter()
            .map(|value| self.convert_stmt(value))
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

        Ok(ProgramIR {
            id: self.program_id,
            module_name: pb_program.module_name,
            file_path: pb_program.file_path,
            scopes,
            symbols,
            imports,
            // decls,
            body,
        })
    }

    fn convert_scope(&self, scope: &pb::ScopeIr) -> ScopeIR {
        let span = self.convert_required_span(&scope.span);
        ScopeIR {
            id: scope.id as usize,
            parent_id: match scope.parent_id {
                Some(id) => Some(id as usize),
                None => None
            },
            name: scope.name.clone(),
            kind: crate::ir::nodes::scope_ir::ScopeKind::from(scope.kind),
            span,
        }
    }

    fn convert_symbol(&self, symbol: &pb::SymbolIr) -> SymbolIR {
        let span = self.convert_required_span(&symbol.span);
        SymbolIR {
            id: symbol.id as usize,
            name: symbol.name.clone(),
            kind: crate::ir::nodes::symbol_ir::SymbolKind::from(symbol.kind),
            scope_id: symbol.scope_id as usize,
            span,
        }
    }

    fn convert_import(&self, import: &pb::ImportIr) -> ImportIR {
        ImportIR {
            id: import.id as usize,
            local_symbol_id: import.local_symbol_id as usize,
            scope_id: import.scope_id as usize,
            kind: ImportKind::from(import.kind),
            module_name: import.module_name.clone(),
            imported_name: import.imported_name.clone(),
            alias: import.alias.clone(),
            relative_level: import.relative_level as usize,
            span: self.convert_required_span(&import.span),
        }
    }

    fn convert_param(&self, param: &pb::ArgIr) -> Result<ArgIR, Box<dyn std::error::Error>> {
        Ok(ArgIR {
            symbol_id: param.symbol_id as usize,
            arg: param.arg.clone(),

            // TODO make cleaner, this makes sense but it's inconsistent with the rest
            kind: functiondef_ir::ArgKind::try_from(param.kind)?,

            annotation: match &param.annotation {
                Some(annotation) => Some(self.convert_expr(annotation)?),
                None => None,
            },

            default: match &param.default {
                Some(default) => Some(Box::new(self.convert_expr(default)?)),
                // Most parameters do not have a default value.
                None => None,
            },

            span: self.convert_required_span(&param.span),
        })
    }

    fn convert_function(
        &self,
        function: &pb::FunctionDefIr,
    ) -> Result<FunctionDefIR, Box<dyn std::error::Error>> {
        let mut stmts: Vec<StmtIR> = Vec::new();
        let mut params: Vec<ArgIR> = Vec::new();
        let mut decorators: Vec<ExprIR> = Vec::new();

        for stmt in &function.body {
            let stmt_ir = self.convert_stmt(stmt)?;
            stmts.push(stmt_ir);
        }

        for param in &function.args {
            let param_ir = self.convert_param(param)?;
            params.push(param_ir);
        }

        for decorator in &function.decorator_list {
            let decorator_ir = self.convert_expr(decorator)?;
            decorators.push(decorator_ir);
        }

        let returns = match &function.returns {
            Some(returns) => Some(self.convert_expr(returns)?),
            None => None,
        };

        Ok(FunctionDefIR {
            id: function.id as usize,
            symbol_id: function.symbol_id as usize, // redundant?
            name: function.name.clone(),
            scope_id: function.scope_id as usize,
            body_scope_id: function.body_scope_id as usize,
            args: params,
            body: stmts,
            returns,
            decorator_list: decorators,
            type_comment: function.type_comment.clone(),
            type_params: function
                .type_params
                .iter()
                .map(|value| self.convert_type_param(value))
                .collect::<Result<_, _>>()?,
            symbol_ref: self.symbol_ref(function.symbol_id as usize),
            span: self.convert_required_span(&function.span),
        })
    }

    fn convert_class(
        &self,
        class_decl: &pb::ClassDefIr,
    ) -> Result<ClassDefIR, Box<dyn std::error::Error>> {
        let mut body: Vec<StmtIR> = Vec::new();
        let mut bases: Vec<ExprIR> = Vec::new();
        let mut decorators: Vec<ExprIR> = Vec::new();

        for stmt in &class_decl.body {
            let stmt_ir = self.convert_stmt(stmt)?;
            body.push(stmt_ir);
        }

        for expr in &class_decl.bases {
            let expr_ir = self.convert_expr(expr)?;
            bases.push(expr_ir);
        }

        for expr in &class_decl.decorator_list {
            let expr_ir = self.convert_expr(expr)?;
            decorators.push(expr_ir);
        }

        Ok(ClassDefIR {
            id: class_decl.id as usize,
            symbol_id: class_decl.symbol_id as usize,
            name: class_decl.name.clone(),
            scope_id: class_decl.scope_id as usize,
            body_scope_id: class_decl.body_scope_id as usize,
            body,
            bases,
            keywords: class_decl
                .keywords
                .iter()
                .map(|value| self.convert_keyword(value))
                .collect::<Result<_, _>>()?,
            decorator_list: decorators,
            type_params: class_decl
                .type_params
                .iter()
                .map(|value| self.convert_type_param(value))
                .collect::<Result<_, _>>()?,
            span: self.convert_required_span(&class_decl.span),
        })
    }

    fn convert_keyword(
        &self,
        keyword: &pb::KeywordArgIr,
    ) -> Result<KeywordIR, Box<dyn std::error::Error>> {
        let value = keyword.value.as_ref().ok_or("keyword has no value")?;
        Ok(KeywordIR {
            arg: keyword.arg.clone(),
            value: Box::new(self.convert_expr(value)?),
            span: self.convert_required_span(&keyword.span),
        })
    }

    fn convert_type_param(
        &self,
        param: &pb::TypeParamIr,
    ) -> Result<TypeParamIR, Box<dyn std::error::Error>> {
        match param.kind.as_ref().ok_or("type parameter has no kind")? {
            pb::type_param_ir::Kind::TypeVar(value) => Ok(TypeParamIR::TypeVar(TypeVarIR {
                name: value.name.clone(),
                bound: value
                    .bound
                    .as_ref()
                    .map(|value| self.convert_expr(value))
                    .transpose()?,
                default_value: value
                    .default_value
                    .as_ref()
                    .map(|value| self.convert_expr(value))
                    .transpose()?,
                span: self.convert_required_span(&value.span),
            })),
            pb::type_param_ir::Kind::ParamSpec(value) => Ok(TypeParamIR::ParamSpec(ParamSpecIR {
                name: value.name.clone(),
                default_value: value
                    .default_value
                    .as_ref()
                    .map(|value| self.convert_expr(value))
                    .transpose()?,
                span: self.convert_required_span(&value.span),
            })),
            pb::type_param_ir::Kind::TypeVarTuple(value) => {
                Ok(TypeParamIR::TypeVarTuple(TypeVarTupleIR {
                    name: value.name.clone(),
                    default_value: value
                        .default_value
                        .as_ref()
                        .map(|value| self.convert_expr(value))
                        .transpose()?,
                    span: self.convert_required_span(&value.span),
                }))
            }
        }
    }

    fn convert_pattern(
        &self,
        pattern: &pb::PatternIr,
    ) -> Result<PatternIR, Box<dyn std::error::Error>> {
        match &pattern.kind {
            Some(pb::pattern_ir::Kind::ValuePattern(valuepattern_ir)) => {
                let value = match &valuepattern_ir.value {
                    Some(value) => self.convert_expr(value)?,
                    None => return Err("value pattern has no value".into()),
                };

                let span = self.convert_required_span(&valuepattern_ir.span);

                Ok(PatternIR::ValuePattern(ValuePatternIR { value, span }))
            }

            Some(pb::pattern_ir::Kind::AsPattern(aspattern_ir)) => {
                let inner_pattern = match aspattern_ir.pattern.as_deref() {
                    Some(pattern) => self.convert_pattern(pattern)?,
                    None => return Err("as pattern has no inner pattern".into()),
                };

                let span = self.convert_required_span(&aspattern_ir.span);

                Ok(PatternIR::AsPattern(AsPatternIR {
                    pattern: Box::new(inner_pattern),
                    name: aspattern_ir.name.clone(),
                    span,
                }))
            }

            Some(pb::pattern_ir::Kind::SequencePattern(sequencepattern_ir)) => {
                let mut patterns: Vec<PatternIR> = Vec::new();

                for pattern in &sequencepattern_ir.patterns {
                    let pattern = self.convert_pattern(pattern)?;
                    patterns.push(pattern);
                }

                let span = self.convert_required_span(&sequencepattern_ir.span);

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

                let span = self.convert_required_span(&singletonpattern_ir.span);

                Ok(PatternIR::SingletonPattern(SingletonPatternIR {
                    value,
                    span,
                }))
            }

            Some(pb::pattern_ir::Kind::MappingPattern(mappingpattern_ir)) => {
                let mut keys = Vec::new();
                for key in &mappingpattern_ir.keys {
                    keys.push(self.convert_expr(key)?);
                }

                let mut patterns = Vec::new();
                for pattern in &mappingpattern_ir.patterns {
                    patterns.push(self.convert_pattern(pattern)?);
                }

                let span = self.convert_required_span(&mappingpattern_ir.span);

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
                    Some(cls) => self.convert_expr(cls)?,
                    None => return Err("class pattern has no class expression".into()),
                };

                let mut patterns = Vec::new();
                for pattern in &classpattern_ir.patterns {
                    patterns.push(self.convert_pattern(pattern)?);
                }

                let mut kwd_patterns = Vec::new();
                for pattern in &classpattern_ir.kwd_patterns {
                    kwd_patterns.push(self.convert_pattern(pattern)?);
                }

                let span = self.convert_required_span(&classpattern_ir.span);

                Ok(PatternIR::ClassPattern(ClassPatternIR {
                    cls,
                    patterns,
                    kwd_attrs: classpattern_ir.kwd_attrs.clone(),
                    kwd_patterns,
                    span,
                }))
            }

            Some(pb::pattern_ir::Kind::StarPattern(starpattern_ir)) => {
                let span = self.convert_required_span(&starpattern_ir.span);

                Ok(PatternIR::StarPattern(StarPatternIR {
                    name: starpattern_ir.name.clone(),
                    span,
                }))
            }

            Some(pb::pattern_ir::Kind::CapturePattern(capturepattern_ir)) => {
                let span = self.convert_required_span(&capturepattern_ir.span);

                Ok(PatternIR::CapturePattern(CapturePatternIR {
                    name: capturepattern_ir.name.clone(),
                    span,
                }))
            }

            Some(pb::pattern_ir::Kind::WildcardPattern(wildcardpattern_ir)) => {
                let span = self.convert_required_span(&wildcardpattern_ir.span);

                Ok(PatternIR::WildcardPattern(WildcardPatternIR { span }))
            }

            Some(pb::pattern_ir::Kind::OrPattern(orpattern_ir)) => {
                let mut patterns = Vec::new();

                for pattern in &orpattern_ir.patterns {
                    patterns.push(self.convert_pattern(pattern)?);
                }

                let span = self.convert_required_span(&orpattern_ir.span);

                Ok(PatternIR::OrPattern(OrPatternIR { patterns, span }))
            }

            None => Err("pattern has no kind".into()),
        }
    }

    fn convert_match_case(
        &self,
        case: &pb::MatchCaseIr,
    ) -> Result<MatchCaseIR, Box<dyn std::error::Error>> {
        let mut body: Vec<StmtIR> = Vec::new();

        for stmt in &case.body {
            let stmt_ir = self.convert_stmt(stmt)?;
            body.push(stmt_ir);
        }

        let span = self.convert_required_span(&case.span);

        let guard = match &case.guard {
            Some(guard) => Some(self.convert_expr(guard)?),
            None => None,
        };

        let pattern = match &case.pattern {
            Some(pattern) => self.convert_pattern(pattern)?,
            None => return Err("case has no pattern".into()),
        };

        Ok(MatchCaseIR {
            scope_id: case.scope_id as usize,
            pattern,
            guard,
            body,
            span,
        })
    }

    fn convert_with_item(
        &self,
        item: &pb::WithItemIr,
    ) -> Result<WithItemIR, Box<dyn std::error::Error>> {
        let context_expr = item
            .context_expr
            .as_ref()
            .ok_or("with item has no context expression")?;

        let optional_vars = item
            .optional_vars
            .as_ref()
            .map(|value| self.convert_expr(value))
            .transpose()?;

        Ok(WithItemIR {
            context_expr: self.convert_expr(context_expr)?,
            optional_vars,
        })
    }

    fn convert_except_handler(
        &self,
        handler: &pb::ExceptHandlerIr,
    ) -> Result<ExceptHandlerIR, Box<dyn std::error::Error>> {
        let exception_type = handler
            .r#type
            .as_ref()
            .map(|value| self.convert_expr(value))
            .transpose()?;

        let body = handler
            .body
            .iter()
            .map(|value| self.convert_stmt(value))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ExceptHandlerIR {
            exception_type,
            name: handler.name.clone(),
            body,
            span: self.convert_required_span(&handler.span),
        })
    }

    fn convert_stmt(&self, stmt: &pb::StmtIr) -> Result<StmtIR, Box<dyn std::error::Error>> {
        match &stmt.kind {
            Some(pb::stmt_ir::Kind::Annassign(annassign_ir)) => {
                let target = annassign_ir
                    .target
                    .as_ref()
                    .ok_or("annassign statement has no target")?;

                let value = annassign_ir
                    .value
                    .as_ref()
                    .map(|value| self.convert_expr(value))
                    .transpose()?;

                let annotation = match &annassign_ir.annotation {
                    Some(annotation) => Some(self.convert_expr(annotation)?),
                    None => None,
                };

                Ok(StmtIR::AnnAssign(AnnAssignIR {
                    target: self.convert_expr(target)?,
                    annotation: annotation.expect("AnnAssign statement has no annotation"),
                    value,
                    simple: annassign_ir.simple as usize,
                    span: self.convert_required_span(&annassign_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::Assign(assign_ir)) => {
                let targets = assign_ir
                    .target
                    .iter()
                    .map(|value| self.convert_expr(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let value = assign_ir
                    .value
                    .as_ref()
                    .ok_or("assign statement has no value")?;

                Ok(StmtIR::Assign(AssignIR {
                    targets,
                    value: self.convert_expr(value)?,
                    type_comment: assign_ir.type_comment.clone(),
                    span: self.convert_required_span(&assign_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::DeleteStmt(delete_ir)) => {
                let targets = delete_ir
                    .targets
                    .iter()
                    .map(|value| self.convert_expr(value))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::Delete(DeleteIR {
                    targets,
                    span: self.convert_required_span(&delete_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::AssertStmt(assert_ir)) => {
                let test = assert_ir
                    .test
                    .as_ref()
                    .ok_or("assert statement has no test")?;

                let msg = assert_ir
                    .msg
                    .as_ref()
                    .map(|value| self.convert_expr(value))
                    .transpose()?;

                Ok(StmtIR::Assert(AssertIR {
                    test: self.convert_expr(test)?,
                    msg,
                    span: self.convert_required_span(&assert_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::RaiseStmt(raise_ir)) => {
                let exc = raise_ir
                    .exc
                    .as_ref()
                    .map(|value| self.convert_expr(value))
                    .transpose()?;

                let cause = raise_ir
                    .cause
                    .as_ref()
                    .map(|value| self.convert_expr(value))
                    .transpose()?;

                Ok(StmtIR::Raise(RaiseIR {
                    exc,
                    cause,
                    span: self.convert_required_span(&raise_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::GlobalStmt(global_ir)) => Ok(StmtIR::Global(GlobalIR {
                names: global_ir.names.clone(),
                span: self.convert_required_span(&global_ir.span),
            })),

            Some(pb::stmt_ir::Kind::NonlocalStmt(nonlocal_ir)) => {
                Ok(StmtIR::Nonlocal(NonlocalIR {
                    names: nonlocal_ir.names.clone(),
                    span: self.convert_required_span(&nonlocal_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::PassStmt(pass_ir)) => Ok(StmtIR::Pass(PassIR {
                span: self.convert_required_span(&pass_ir.span),
            })),

            Some(pb::stmt_ir::Kind::BreakStmt(break_ir)) => Ok(StmtIR::Break(BreakIR {
                span: self.convert_required_span(&break_ir.span),
            })),

            Some(pb::stmt_ir::Kind::ContinueStmt(continue_ir)) => {
                Ok(StmtIR::Continue(ContinueIR {
                    span: self.convert_required_span(&continue_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::WithStmt(with_ir)) => {
                let items = with_ir
                    .items
                    .iter()
                    .map(|value| self.convert_with_item(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let body = with_ir
                    .body
                    .iter()
                    .map(|value| self.convert_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::With(WithIR {
                    items,
                    body,
                    type_comment: with_ir.type_comment.clone(),
                    span: self.convert_required_span(&with_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::AsyncWith(async_with_ir)) => {
                let items = async_with_ir
                    .items
                    .iter()
                    .map(|value| self.convert_with_item(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let body = async_with_ir
                    .body
                    .iter()
                    .map(|value| self.convert_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::AsyncWith(AsyncWithIR {
                    items,
                    body,
                    type_comment: async_with_ir.type_comment.clone(),
                    span: self.convert_required_span(&async_with_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::TryStmt(try_ir)) => {
                let body = try_ir
                    .body
                    .iter()
                    .map(|value| self.convert_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let handlers = try_ir
                    .handlers
                    .iter()
                    .map(|value| self.convert_except_handler(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let orelse = try_ir
                    .orelse
                    .iter()
                    .map(|value| self.convert_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let finalbody = try_ir
                    .finalbody
                    .iter()
                    .map(|value| self.convert_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::Try(TryIR {
                    body,
                    handlers,
                    orelse,
                    finalbody,
                    span: self.convert_required_span(&try_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::TryStarStmt(try_ir)) => {
                let body = try_ir
                    .body
                    .iter()
                    .map(|value| self.convert_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let handlers = try_ir
                    .handlers
                    .iter()
                    .map(|value| self.convert_except_handler(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let orelse = try_ir
                    .orelse
                    .iter()
                    .map(|value| self.convert_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let finalbody = try_ir
                    .finalbody
                    .iter()
                    .map(|value| self.convert_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::TryStar(TryStarIR {
                    body,
                    handlers,
                    orelse,
                    finalbody,
                    span: self.convert_required_span(&try_ir.span),
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
                    .map(|value| self.convert_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let orelse = async_for_ir
                    .orelse
                    .iter()
                    .map(|value| self.convert_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::AsyncFor(AsyncForIR {
                    target: self.convert_expr(target)?,
                    iter: self.convert_expr(iterable)?,
                    body,
                    orelse,
                    type_comment: async_for_ir.type_comment.clone(),
                    span: self.convert_required_span(&async_for_ir.span),
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
                    .map(|value| self.convert_type_param(value))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(StmtIR::TypeAlias(TypeAliasIR {
                    name: self.convert_expr(name)?,
                    type_params,
                    value: self.convert_expr(value)?,
                    span: self.convert_required_span(&type_alias_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::AsyncFunctionDef(function_ir)) => {
                let args = function_ir
                    .args
                    .iter()
                    .map(|value| self.convert_param(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let body = function_ir
                    .body
                    .iter()
                    .map(|value| self.convert_stmt(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let decorators = function_ir
                    .decorator_list
                    .iter()
                    .map(|value| self.convert_expr(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let returns = function_ir
                    .returns
                    .as_ref()
                    .map(|value| self.convert_expr(value))
                    .transpose()?;

                let type_params = function_ir
                    .type_params
                    .iter()
                    .map(|value| self.convert_type_param(value))
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
                    span: self.convert_required_span(&function_ir.span),
                }))
            }

            Some(pb::stmt_ir::Kind::Match(match_ir)) => {
                let mut cases: Vec<MatchCaseIR> = Vec::new();

                for case in &match_ir.cases {
                    let case_ir = self.convert_match_case(case)?;
                    cases.push(case_ir);
                }

                let span = self.convert_required_span(&match_ir.span);

                let subject = match &match_ir.subject {
                    Some(subject) => Box::new(self.convert_expr(subject)?),
                    None => return Err("match statement has no subject".into()),
                };

                Ok(StmtIR::Match(MatchIR {
                    subject,
                    cases,
                    span,
                }))
            }

            Some(pb::stmt_ir::Kind::AugAssign(aug)) => {
                let target = match &aug.target {
                    Some(target) => Box::new(self.convert_expr(target)?),
                    None => return Err("augmented assignment has no target".into()),
                };

                let value = match &aug.value {
                    Some(value) => Box::new(self.convert_expr(value)?),
                    None => return Err("augmented assignment has no value".into()),
                };

                let span = self.convert_required_span(&aug.span);

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
                    Some(value) => Some(Box::new(self.convert_expr(value)?)),
                    None => None,
                };

                let span = self.convert_required_span(&ret.span);

                Ok(StmtIR::Return(ReturnIR { value, span }))
            }

            Some(pb::stmt_ir::Kind::ExprStmt(expr_stmt)) => {
                let value = match &expr_stmt.value {
                    Some(value) => Some(Box::new(self.convert_expr(value)?)),
                    None => return Err("expression statement has no expression".into()),
                };

                let span = self.convert_required_span(&expr_stmt.span);

                Ok(StmtIR::ExprStmt(ExprStmtIR { value, span }))
            }

            Some(pb::stmt_ir::Kind::IfStmt(if_stmt)) => {
                let test = match &if_stmt.test {
                    Some(test) => Box::new(self.convert_expr(test)?),
                    None => return Err("if statement has no test expression".into()),
                };

                let mut body: Vec<StmtIR> = Vec::new();

                for stmt in &if_stmt.body {
                    let stmt = self.convert_stmt(stmt)?;
                    body.push(stmt);
                }

                let mut orelse: Vec<StmtIR> = Vec::new();

                for stmt in &if_stmt.orelse {
                    let stmt = self.convert_stmt(stmt)?;
                    orelse.push(stmt);
                }

                let span = self.convert_required_span(&if_stmt.span);

                Ok(StmtIR::If(IfIR {
                    test,
                    scope_id: if_stmt.scope_id as usize,
                    else_scope_id: if_stmt.else_scope_id as usize,
                    then_scope_id: if_stmt.then_scope_id as usize,
                    body,
                    orelse,
                    span,
                }))
            }

            Some(pb::stmt_ir::Kind::ForLoop(for_loop)) => {
                let target = match &for_loop.target {
                    Some(target) => Box::new(self.convert_expr(target)?),
                    None => return Err("for loop has no target".into()),
                };

                let iter = match &for_loop.iter {
                    Some(iter) => Box::new(self.convert_expr(iter)?),
                    None => return Err("for loop has no iterable expression".into()),
                };

                let mut body: Vec<StmtIR> = Vec::new();

                for stmt in &for_loop.body {
                    let stmt = self.convert_stmt(stmt)?;
                    body.push(stmt);
                }

                let mut orelse: Vec<StmtIR> = Vec::new();

                for stmt in &for_loop.orelse {
                    let stmt = self.convert_stmt(stmt)?;
                    orelse.push(stmt);
                }

                let span = self.convert_required_span(&for_loop.span);

                Ok(StmtIR::For(ForIR {
                    target,
                    iter,
                    scope_id: for_loop.scope_id as usize,
                    body_scope_id: for_loop.body_scope_id as usize,
                    body,
                    orelse,
                    span,
                }))
            }

            Some(pb::stmt_ir::Kind::WhileLoop(while_loop)) => {
                let test = match &while_loop.test {
                    Some(test) => Box::new(self.convert_expr(test)?),
                    None => return Err("while loop has no test expression".into()),
                };

                let mut body: Vec<StmtIR> = Vec::new();

                for stmt in &while_loop.body {
                    let stmt = self.convert_stmt(stmt)?;
                    body.push(stmt);
                }

                let mut orelse: Vec<StmtIR> = Vec::new();

                for stmt in &while_loop.orelse {
                    let stmt = self.convert_stmt(stmt)?;
                    orelse.push(stmt);
                }

                let span = self.convert_required_span(&while_loop.span);

                Ok(StmtIR::While(WhileIR {
                    test,
                    scope_id: while_loop.scope_id as usize,
                    body_scope_id: while_loop.body_scope_id as usize,
                    body,
                    orelse,
                    span,
                }))
            }

            Some(pb::stmt_ir::Kind::ImportStmt(import_stmt)) => {
                let span = self.convert_required_span(&import_stmt.span);

                Ok(StmtIR::Import(ImportIR {
                    id: import_stmt.id as usize,
                    local_symbol_id: import_stmt.local_symbol_id as usize,
                    scope_id: import_stmt.scope_id as usize,
                    kind: crate::ir::nodes::ImportKind::from(import_stmt.kind),
                    module_name: import_stmt.module_name.clone(),
                    imported_name: import_stmt.imported_name.clone(),
                    alias: import_stmt.alias.clone(),
                    relative_level: import_stmt.relative_level as usize,
                    span,
                }))
            }

            Some(pb::stmt_ir::Kind::Function(function)) => {
                Ok(StmtIR::Function(self.convert_function(function)?))
            }

            Some(pb::stmt_ir::Kind::ClassDecl(class_decl)) => {
                Ok(StmtIR::Class(self.convert_class(class_decl)?))
            }

            None => Err("statement has no kind".into()),
            // _ => Err("unsupported statement kind".into()),
        }
    }

    fn convert_span(&self, span: &pb::SourceSpan) -> SourceSpan {
        SourceSpan {
            file: span.file.clone(),
            lineno: span.lineno as usize,
            col_offset: span.col_offset as usize,
            end_lineno: span.end_lineno.map(|value| value as usize),
            end_col_offset: span.end_col_offset.map(|value| value as usize),
        }
    }

    fn convert_joined_str(
        &self,
        spec: &pb::JoinedStrIr,
    ) -> Result<JoinedStrIR, Box<dyn std::error::Error>> {
        Ok(JoinedStrIR {
            values: spec
                .values
                .iter()
                .map(|value| self.convert_expr(value))
                .collect::<Result<Vec<_>, _>>()?,
            span: self.convert_required_span(&spec.span),
        })
    }

    fn convert_generator(&self, comp: &pb::CompIr) -> Result<CompIR, Box<dyn std::error::Error>> {
        let target: Box<ExprIR> = Box::new(
            self.convert_expr(comp.target.as_ref().ok_or("comprehension has no target")?)?,
        );

        let iter: Box<ExprIR> = Box::new(
            self.convert_expr(comp.iter.as_ref().ok_or("comprehension has no iterable")?)?,
        );

        let mut ifs: Vec<ExprIR> = Vec::new();
        for expr in &comp.ifs {
            ifs.push(self.convert_expr(expr)?);
        }

        let is_async: bool = comp.is_async;

        let span: SourceSpan = self.convert_required_span(&comp.span);

        Ok(CompIR {
            target,
            iter,
            ifs,
            is_async,
            span,
        })
    }

    fn convert_expr(&self, expr: &pb::ExprIr) -> Result<ExprIR, Box<dyn std::error::Error>> {
        match &expr.kind {
            Some(pb::expr_ir::Kind::Identifier(identifier)) => Ok(ExprIR::Name(NameIR {
                id: identifier.id.clone(),
                use_scope_id: identifier.use_scope_id as usize,
                symbol_ref: None,
                span: self.convert_required_span(&identifier.span),
            })),

            Some(pb::expr_ir::Kind::AwaitExpr(await_ir)) => {
                let value = await_ir
                    .value
                    .as_deref()
                    .ok_or("await expression has no value")?;

                Ok(ExprIR::AwaitExpr(AwaitIR {
                    value: Box::new(self.convert_expr(value)?),
                    span: self.convert_required_span(&await_ir.span),
                }))
            }

            Some(pb::expr_ir::Kind::YieldExpr(yield_ir)) => {
                let value = yield_ir
                    .value
                    .as_deref()
                    .map(|value| self.convert_expr(value))
                    .transpose()?
                    .map(Box::new);

                Ok(ExprIR::YieldExpr(YieldIR {
                    value,
                    span: self.convert_required_span(&yield_ir.span),
                }))
            }

            Some(pb::expr_ir::Kind::YieldFrom(yield_from_ir)) => {
                let value = yield_from_ir
                    .value
                    .as_deref()
                    .ok_or("yield from expression has no value")?;

                Ok(ExprIR::YieldFromExpr(YieldFromIR {
                    value: Box::new(self.convert_expr(value)?),
                    span: self.convert_required_span(&yield_from_ir.span),
                }))
            }

            Some(pb::expr_ir::Kind::TemplateStr(template_ir)) => {
                let values = template_ir
                    .values
                    .iter()
                    .map(|value| self.convert_expr(value))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(ExprIR::TemplateStr(TemplateStrIR {
                    values,
                    span: self.convert_required_span(&template_ir.span),
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
                    .map(|value| self.convert_joined_str(value))
                    .transpose()?;

                Ok(ExprIR::InterpolationExpr(InterpolationIR {
                    value: Box::new(self.convert_expr(value)?),
                    str: interpolation_ir.str.clone(),
                    conversion: Conversion::try_from(interpolation_ir.conversion)?,
                    format_spec,
                    span: self.convert_required_span(&interpolation_ir.span),
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
                    .map(|value| self.convert_joined_str(value))
                    .transpose()?;

                Ok(ExprIR::FormattedValue(FormattedValueIR {
                    value: Box::new(self.convert_expr(value)?),
                    conversion: Conversion::try_from(formatted_ir.conversion)?,
                    format_spec,
                    span: self.convert_required_span(&formatted_ir.span),
                }))
            }

            Some(pb::expr_ir::Kind::JoinedStr(joinedstr_ir)) => {
                let mut values: Vec<ExprIR> = Vec::new();
                for value in &joinedstr_ir.values {
                    values.push(self.convert_expr(value)?);
                }

                let span = self.convert_required_span(&joinedstr_ir.span);

                Ok(ExprIR::JoinedStr(JoinedStrIR { values, span }))
            }

            Some(pb::expr_ir::Kind::NamedExpr(named_expr_ir)) => {
                let target = named_expr_ir
                    .target
                    .as_ref()
                    .ok_or("named expression has no target")?;

                let value = match &named_expr_ir.value {
                    Some(value) => self.convert_expr(value)?,
                    None => return Err("named expression has no value".into()),
                };

                let span = self.convert_required_span(&named_expr_ir.span);

                Ok(ExprIR::NamedExpr(NamedExprIR {
                    target: Box::new(self.convert_expr(target)?),
                    value: Box::new(value),
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Starred(starred_ir)) => {
                let value = match &starred_ir.value {
                    Some(value) => self.convert_expr(value)?,
                    None => return Err("starred expression has no value".into()),
                };

                let span = self.convert_required_span(&starred_ir.span);

                Ok(ExprIR::StarredExpr(StarredIR {
                    value: Box::new(value),
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Set(set_ir)) => {
                let mut elements: Vec<ExprIR> = Vec::new();

                for element in &set_ir.elts {
                    elements.push(self.convert_expr(element)?);
                }

                let span = self.convert_required_span(&set_ir.span);

                Ok(ExprIR::SetExpr(SetIR {
                    elts: elements,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Dict(dict_ir)) => {
                let keys = dict_ir
                    .keys
                    .iter()
                    .map(|key| key.value.as_ref().map(|value| self.convert_expr(value)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                let values = dict_ir
                    .values
                    .iter()
                    .map(|value| self.convert_expr(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let span = self.convert_required_span(&dict_ir.span);

                Ok(ExprIR::DictExpr(DictIR { keys, values, span }))
            }

            Some(pb::expr_ir::Kind::ListComp(listcomp_ir)) => {
                let elt = match &listcomp_ir.elt {
                    Some(expr) => self.convert_expr(expr)?,
                    None => return Err("list comprehension expression has no elt".into()),
                };

                let mut generators: Vec<CompIR> = Vec::new();
                for comp in &listcomp_ir.generators {
                    generators.push(self.convert_generator(comp)?)
                }

                let span = self.convert_required_span(&listcomp_ir.span);

                Ok(ExprIR::ListComp(ListCompIR {
                    elt: Box::new(elt),
                    generators,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::SetComp(setcomp_ir)) => {
                let elt = match &setcomp_ir.elt {
                    Some(expr) => self.convert_expr(expr)?,
                    None => return Err("set comprehension expression has no elt".into()),
                };

                let mut generators: Vec<CompIR> = Vec::new();
                for comp in &setcomp_ir.generators {
                    generators.push(self.convert_generator(comp)?)
                }

                let span = self.convert_required_span(&setcomp_ir.span);

                Ok(ExprIR::SetComp(SetCompIR {
                    elt: Box::new(elt),
                    generators,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::GeneratorExpr(generatorexpr_ir)) => {
                let elt = match &generatorexpr_ir.elt {
                    Some(expr) => self.convert_expr(expr)?,
                    None => return Err("generator expression has no elt".into()),
                };

                let mut generators: Vec<CompIR> = Vec::new();
                for comp in &generatorexpr_ir.generators {
                    generators.push(self.convert_generator(comp)?)
                }

                let span = self.convert_required_span(&generatorexpr_ir.span);

                Ok(ExprIR::GeneratorExp(GeneratorExpIR {
                    elt: Box::new(elt),
                    generators,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::DictComp(dictcomp_ir)) => {
                let key = match &dictcomp_ir.key {
                    Some(expr) => self.convert_expr(expr)?,
                    None => return Err("dict comprehension expression has no key".into()),
                };

                let value = match &dictcomp_ir.value {
                    Some(expr) => self.convert_expr(expr)?,
                    None => return Err("dict comprehension expression has no value".into()),
                };

                let mut generators: Vec<CompIR> = Vec::new();
                for comp in &dictcomp_ir.generators {
                    generators.push(self.convert_generator(comp)?)
                }

                let span = self.convert_required_span(&dictcomp_ir.span);

                Ok(ExprIR::DictComp(DictCompIR {
                    key: Box::new(key),
                    value: Box::new(value),
                    generators,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::IfExpr(if_expr_ir)) => {
                let test = match &if_expr_ir.test {
                    Some(expr) => self.convert_expr(expr)?,
                    None => return Err("if expression has no test".into()),
                };

                let body = match &if_expr_ir.body {
                    Some(expr) => self.convert_expr(expr)?,
                    None => return Err("if expression has no body".into()),
                };

                let orelse = match &if_expr_ir.orelse {
                    Some(expr) => self.convert_expr(expr)?,
                    None => return Err("if expression has no else expression".into()),
                };

                let span = self.convert_required_span(&if_expr_ir.span);

                Ok(ExprIR::IfExp(IfExpIR {
                    test: Box::new(test),
                    body: Box::new(body),
                    orelse: Box::new(orelse),
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::LambdaExpr(lambda_ir)) => {
                let args = lambda_ir
                    .args
                    .iter()
                    .map(|value| self.convert_param(value))
                    .collect::<Result<Vec<_>, _>>()?;

                let body = match &lambda_ir.body {
                    Some(body) => Box::new(self.convert_expr(body)?),
                    None => return Err("lambda expression has no body".into()),
                };

                Ok(ExprIR::LambdaExpr(LambdaIR {
                    args,
                    body,
                    scope_id: lambda_ir.scope_id as usize,
                    span: self.convert_required_span(&lambda_ir.span),
                }))
            }

            Some(pb::expr_ir::Kind::Constant(constant)) => match constant.kind.as_ref() {
                Some(pb::constant_ir::Kind::EllipsisLit(ellipsis)) => {
                    Ok(ExprIR::Constant(ConstantIR::EllipsisLit(EllipsisIR {
                        span: self.convert_required_span(&ellipsis.span),
                    })))
                }

                Some(pb::constant_ir::Kind::IntegerLit(integer)) => {
                    Ok(ExprIR::Constant(ConstantIR::IntegerLit(IntegerIR {
                        value: integer.value,
                        span: self.convert_required_span(&integer.span),
                    })))
                }

                Some(pb::constant_ir::Kind::FloatLit(float_lit)) => {
                    Ok(ExprIR::Constant(ConstantIR::FloatLit(FloatIR {
                        value: float_lit.value,
                        span: self.convert_required_span(&float_lit.span),
                    })))
                }

                Some(pb::constant_ir::Kind::StringLit(string_lit)) => {
                    Ok(ExprIR::Constant(ConstantIR::StringLit(StringIR {
                        value: string_lit.value.clone(),
                        span: self.convert_required_span(&string_lit.span),
                    })))
                }

                Some(pb::constant_ir::Kind::BoolLit(bool_lit)) => {
                    Ok(ExprIR::Constant(ConstantIR::BooleanLit(BooleanIR {
                        value: bool_lit.value,
                        span: self.convert_required_span(&bool_lit.span),
                    })))
                }

                Some(pb::constant_ir::Kind::NoneLit(none_lit)) => {
                    Ok(ExprIR::Constant(ConstantIR::NoneLit(NoneIR {
                        span: self.convert_required_span(&none_lit.span),
                    })))
                }

                Some(pb::constant_ir::Kind::ComplexLit(complex_lit)) => {
                    Ok(ExprIR::Constant(ConstantIR::ComplexLit(ComplexIR {
                        real: complex_lit.real,
                        imag: complex_lit.imag,
                        span: self.convert_required_span(&complex_lit.span),
                    })))
                }

                Some(pb::constant_ir::Kind::BytesLit(bytes_lit)) => {
                    Ok(ExprIR::Constant(ConstantIR::BytesLit(BytesIR {
                        value: bytes_lit
                            .value
                            .iter()
                            .map(|&x| u8::try_from(x))
                            .collect::<Result<Vec<_>, _>>()?,
                        span: self.convert_required_span(&bytes_lit.span),
                    })))
                }

                None => Err("constant has no kind".into()),
            },

            Some(pb::expr_ir::Kind::List(list)) => {
                let mut elements: Vec<ExprIR> = Vec::new();
                for element in &list.elts {
                    let expr_ir: ExprIR = self.convert_expr(element)?;
                    elements.push(expr_ir);
                }

                Ok(ExprIR::ListExpr(ListIR {
                    elts: elements,
                    span: self.convert_required_span(&list.span),
                }))
            }

            Some(pb::expr_ir::Kind::Tuple(tuple)) => {
                let mut elements: Vec<ExprIR> = Vec::new();
                for element in &tuple.elts {
                    let expr_ir: ExprIR = self.convert_expr(element)?;
                    elements.push(expr_ir);
                }

                Ok(ExprIR::TupleExpr(TupleIR {
                    elts: elements,
                    span: self.convert_required_span(&tuple.span),
                }))
            }

            Some(pb::expr_ir::Kind::Call(call)) => {
                let callee: Box<ExprIR> = match &call.func {
                    Some(callee) => Box::new(self.convert_expr(callee)?),
                    None => {
                        return Err("call has no callee".into());
                    }
                };

                let mut args: Vec<ExprIR> = Vec::new();

                for arg in &call.args {
                    let arg = self.convert_expr(arg)?;
                    args.push(arg);
                }

                let keywords = call
                    .keywords
                    .iter()
                    .map(|value| self.convert_keyword(value))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(ExprIR::Call(CallIR {
                    func: callee,
                    args,
                    keywords,
                    span: self.convert_required_span(&call.span),
                }))
            }

            Some(pb::expr_ir::Kind::Attribute(attribute)) => {
                let base = match &attribute.value {
                    Some(base) => Box::new(self.convert_expr(base)?),
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
                    span: self.convert_required_span(&attribute.span),
                }))
            }

            Some(pb::expr_ir::Kind::Binop(binop)) => {
                let operator = Operator::from(binop.op);

                let left = match &binop.left {
                    Some(left) => Box::new(self.convert_expr(left)?),
                    None => {
                        return Err(Self::missing_expr(
                            "binary expression",
                            "left operand",
                            &binop.span,
                        ));
                    }
                };

                let right = match &binop.right {
                    Some(right) => Box::new(self.convert_expr(right)?),
                    None => {
                        return Err(Self::missing_expr(
                            "binary expression",
                            "right operand",
                            &binop.span,
                        ));
                    }
                };

                let span = self.convert_required_span(&binop.span);

                Ok(ExprIR::BinOpExpr(BinOpIR {
                    left,
                    right,
                    op: operator,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Unaryop(unaryop)) => {
                let operator = Operator::from(unaryop.op);

                let operand = match &unaryop.operand {
                    Some(operand) => Box::new(self.convert_expr(operand)?),
                    None => {
                        return Err(Self::missing_expr(
                            "unary expression",
                            "operand",
                            &unaryop.span,
                        ));
                    }
                };

                let span = self.convert_required_span(&unaryop.span);

                Ok(ExprIR::UnaryOpExpr(UnaryOpIR {
                    op: operator,
                    operand,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Boolop(boolop)) => {
                let mut values = Vec::new();
                for value in &boolop.values {
                    let value_ir = self.convert_expr(value)?;
                    values.push(value_ir);
                }

                let operator = Operator::from(boolop.op);

                let span = self.convert_required_span(&boolop.span);

                Ok(ExprIR::BoolOpExpr(BoolOpIR {
                    values,
                    op: operator,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Compare(compare)) => {
                let left = match &compare.left {
                    Some(left) => Box::new(self.convert_expr(left)?),
                    None => {
                        return Err(Self::missing_expr(
                            "comparison expression",
                            "left operand",
                            &compare.span,
                        ));
                    }
                };

                let span = self.convert_required_span(&compare.span);

                let mut ops: Vec<Operator> = Vec::new();
                for &op in &compare.ops {
                    ops.push(Operator::from(op));
                }

                let mut comparators: Vec<ExprIR> = Vec::new();
                for comparator in &compare.comparators {
                    let comparator = self.convert_expr(comparator)?;
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
                    Some(target) => Box::new(self.convert_expr(target)?),
                    None => {
                        return Err(Self::missing_expr(
                            "subscript expression",
                            "target",
                            &subscript.span,
                        ));
                    }
                };

                let index = match &subscript.slice {
                    Some(subsript) => Box::new(self.convert_expr(subsript)?),
                    None => {
                        return Err(Self::missing_expr(
                            "subscript expression",
                            "index",
                            &subscript.span,
                        ));
                    }
                };

                let span = self.convert_required_span(&subscript.span);

                Ok(ExprIR::SubscriptExpr(SubscriptIR {
                    value: target,
                    slice: index,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Slice(slice)) => {
                let span = self.convert_required_span(&slice.span);

                let upper = match &slice.upper {
                    Some(upper) => Some(Box::new(self.convert_expr(upper)?)),
                    None => None,
                };

                let lower = match &slice.lower {
                    Some(lower) => Some(Box::new(self.convert_expr(lower)?)),
                    None => None,
                };

                let step = match &slice.step {
                    Some(step) => Some(Box::new(self.convert_expr(step)?)),
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
                Err("malformed ExprIR: protobuf oneof field 'kind' is unset (the producer emitted an empty expression)".into())
            }
        }
    }

    fn convert_required_span(&self, span: &Option<pb::SourceSpan>) -> SourceSpan {
        self.convert_span(span.as_ref().expect("IR node is missing its source span"))
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
