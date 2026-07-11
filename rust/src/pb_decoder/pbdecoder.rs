use std::fs;
use std::io;
use std::path::PathBuf;

use crate::ir::nodes::annotation_ir::AnnotationHeadIR;
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

                programs.push(ProgramIR {
                    module_name: pb_program.module_name,
                    file_path: pb_program.file_path,
                    scopes,
                    symbols,
                    imports,
                    decls,
                });
            }
        }

        Ok(programs)
    }

    fn convert_scope(scope: &pb::ScopeIr) -> ScopeIR {
        let span = scope.span.as_ref().map(Self::convert_span);
        ScopeIR {
            id: scope.id,
            parent_id: scope.parent_id,
            name: scope.name.clone(),
            kind: crate::ir::nodes::scope_ir::ScopeKind::from(scope.kind),
            span
        }
    }

    fn convert_symbol(symbol: &pb::SymbolIr) -> SymbolIR {
        let span = symbol.span.as_ref().map(Self::convert_span);
        SymbolIR { 
            id: symbol.id, 
            name: symbol.name.clone(), 
            kind: crate::ir::nodes::symbol_ir::SymbolKind::from(symbol.kind), 
            scope_id: symbol.scope_id, 
            span 
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
            span: import.span.as_ref().map(Self::convert_span),
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

        let span = match &binding.span {
            Some(span) => Some(Self::convert_span(span)),
            None => None,
        };

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

        let span = match &head.span {
            Some(span) => Some(Self::convert_span(span)),
            None => None,
        };

        Ok(AnnotationHeadIR {
            root: head.root.clone(),
            attrs,
            scope_id,
            span,
        })
    }

    fn convert_param(param: &pb::ParamIr) -> Result<ParamIR, Box<dyn std::error::Error>> {
        Ok(ParamIR {
            symbol_id: param.symbol_id,
            name: param.name.clone(),

            annotation: match &param.annotation {
                Some(annotation) => Some(Self::convert_annotation(annotation)?),
                None => None,
            },

            default: match &param.default_value {
                Some(default) => Some(Box::new(Self::convert_expr(default)?)),
                // Most parameters do not have a default value.
                None => None,
            },

            span: match &param.span {
                Some(span) => Some(Self::convert_span(span)),
                None => None,
            },
        })
    }

    fn convert_function(
        function: &pb::FunctionIr,
    ) -> Result<FunctionIR, Box<dyn std::error::Error>> {
        let mut stmts: Vec<StmtIR> = Vec::new();
        let mut params: Vec<ParamIR> = Vec::new();
        let mut decorators: Vec<ExprIR> = Vec::new();

        for stmt in &function.body {
            let stmt_ir = Self::convert_stmt(stmt)?;
            stmts.push(stmt_ir);
        }

        for param in &function.params {
            let param_ir = Self::convert_param(param)?;
            params.push(param_ir);
        }

        for decorator in &function.decorators {
            let decorator_ir = Self::convert_expr(decorator)?;
            decorators.push(decorator_ir);
        }

        let returns = match &function.returns {
            Some(returns) => Some(Self::convert_annotation(returns)?),
            None => None,
        };

        // todo decorators

        Ok(FunctionIR {
            id: function.id,
            symbol_id: function.symbol_id,
            name: function.name.clone(),
            scope_id: function.scope_id,
            body_scope_id: function.body_scope_id,
            params: params,
            body: stmts,
            returns: returns,
            decorators: decorators,
            span: match &function.span {
                Some(span) => Some(Self::convert_span(span)),
                None => None,
            },
        })
    }

    fn convert_class(class_decl: &pb::ClassIr) -> Result<ClassIR, Box<dyn std::error::Error>> {
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

        for expr in &class_decl.decorators {
            let expr_ir = Self::convert_expr(expr)?;
            decorators.push(expr_ir);
        }

        Ok(ClassIR {
            id: class_decl.id,
            symbol_id: class_decl.symbol_id,
            name: class_decl.name.clone(),
            scope_id: class_decl.scope_id,
            body_scope_id: class_decl.body_scope_id,
            body: body,
            bases: bases,
            decorators: decorators,
            span: match &class_decl.span {
                Some(span) => Some(Self::convert_span(span)),
                None => None,
            },
        })
    }

    fn convert_stmt(stmt: &pb::StmtIr) -> Result<StmtIR, Box<dyn std::error::Error>> {
        match &stmt.kind {
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

                span: match &binding.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                },
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

                let span = match &aug.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

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

                let span = match &ret.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(StmtIR::Return(ReturnIR { value, span }))
            }

            Some(pb::stmt_ir::Kind::ExprStmt(expr_stmt)) => {
                let value = match &expr_stmt.value {
                    Some(value) => Some(Box::new(Self::convert_expr(value)?)),
                    None => return Err("expression statement has no expression".into()),
                };

                let span = match &expr_stmt.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

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

                let span = match &if_stmt.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

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

                let span = match &for_loop.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(StmtIR::ForLoop(ForLoopIR {
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

                let span = match &while_loop.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(StmtIR::WhileLoop(WhileLoopIR {
                    test,
                    scope_id: while_loop.scope_id,
                    body_scope_id: while_loop.body_scope_id,
                    body,
                    orelse,
                    span,
                }))
            }

            Some(pb::stmt_ir::Kind::ImportStmt(import_stmt)) => {
                let span = match &import_stmt.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

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

            None => {
                return Err("expected statement, found None".into());
            }
        }
    }

    fn convert_span(span: &pb::SourceSpan) -> SourceSpan {
        SourceSpan {
            file: span.file.clone(),
            line: span.line,
            col: span.col,
            end_line: span.end_line,
            end_col: span.end_col,
        }
    }

    fn convert_expr(expr: &pb::ExprIr) -> Result<ExprIR, Box<dyn std::error::Error>> {
        match &expr.kind {
            Some(pb::expr_ir::Kind::Identifier(identifier)) => {
                Ok(ExprIR::Identifier(IdentifierIR {
                    name: identifier.name.clone(),
                    use_scope_id: identifier.use_scope_id,
                    span: match &identifier.span {
                        Some(span) => Some(Self::convert_span(span)),
                        None => None,
                    },
                }))
            }

            Some(pb::expr_ir::Kind::Integer(integer)) => Ok(ExprIR::Integer(IntegerIR {
                value: integer.value,
                span: match &integer.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                },
            })),

            Some(pb::expr_ir::Kind::FloatLit(float_lit)) => Ok(ExprIR::Float(FloatIR {
                value: float_lit.value,
                span: match &float_lit.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                },
            })),

            Some(pb::expr_ir::Kind::StringLit(string_lit)) => Ok(ExprIR::String(StringIR {
                value: string_lit.value.clone(),
                span: match &string_lit.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                },
            })),

            Some(pb::expr_ir::Kind::BoolLit(bool_lit)) => Ok(ExprIR::Bool(BooleanIR {
                value: bool_lit.value,
                span: match &bool_lit.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                },
            })),

            Some(pb::expr_ir::Kind::NoneLit(_none_lit)) => Ok(ExprIR::None(NoneIR {
                span: match &_none_lit.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                },
            })),

            Some(pb::expr_ir::Kind::List(list)) => {
                let mut elements: Vec<ExprIR> = Vec::new();
                for element in &list.elements {
                    let expr_ir: ExprIR = Self::convert_expr(element)?;
                    elements.push(expr_ir);
                }

                Ok(ExprIR::List(ListIR {
                    elements: elements,
                    span: match &list.span {
                        Some(span) => Some(Self::convert_span(span)),
                        None => None,
                    },
                }))
            }

            Some(pb::expr_ir::Kind::Tuple(tuple)) => {
                let mut elements: Vec<ExprIR> = Vec::new();
                for element in &tuple.elements {
                    let expr_ir: ExprIR = Self::convert_expr(element)?;
                    elements.push(expr_ir);
                }

                Ok(ExprIR::List(ListIR {
                    elements: elements,
                    span: match &tuple.span {
                        Some(span) => Some(Self::convert_span(span)),
                        None => None,
                    },
                }))
            }

            Some(pb::expr_ir::Kind::Call(call)) => {
                let callee: Box<ExprIR> = match &call.callee {
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

                let mut kwargs: Vec<KeywordArgIR> = Vec::new();

                for kwarg in &call.kwargs {
                    let name = match &kwarg.name {
                        Some(name) => name.clone(),
                        None => return Err("keyword argument has no name".into()),
                    };

                    let value = match &kwarg.value {
                        Some(value) => Self::convert_expr(value)?,
                        None => return Err("keyword argument has no value".into()),
                    };

                    let span = match &kwarg.span {
                        Some(span) => Some(Self::convert_span(span)),
                        None => None,
                    };

                    kwargs.push(KeywordArgIR {
                        name,
                        value: Box::new(value),
                        span,
                    });
                }

                Ok(ExprIR::CallExpr(CallExprIR {
                    callee: callee,
                    args: args,
                    kwargs: kwargs,
                    span: match &call.span {
                        Some(span) => Some(Self::convert_span(span)),
                        None => None,
                    },
                }))
            }

            Some(pb::expr_ir::Kind::Attribute(attribute)) => {
                let base = match &attribute.base {
                    Some(base) => Box::new(Self::convert_expr(base)?),
                    None => {
                        return Err(Self::missing_expr(
                            "attribute expression",
                            "base",
                            &attribute.span,
                        ));
                    }
                };

                Ok(ExprIR::AttributeExpr(AttributeExprIR {
                    target: base,
                    attr: attribute.attr.clone(),
                    span: match &attribute.span {
                        Some(span) => Some(Self::convert_span(span)),
                        None => None,
                    },
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
                let span = match &binop.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };
                Ok(ExprIR::BinOp(BinOpIR {
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

                let span = match &unaryop.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(ExprIR::UnaryOp(UnaryOpIR {
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

                let span = match &boolop.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(ExprIR::BoolOp(BoolOpIR {
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

                let span = match &compare.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                let mut ops: Vec<Operator> = Vec::new();
                for &op in &compare.ops {
                    ops.push(Operator::from(op));
                }

                let mut comparators: Vec<ExprIR> = Vec::new();
                for comparator in &compare.comparators {
                    let comparator = Self::convert_expr(comparator)?;
                    comparators.push(comparator);
                }

                Ok(ExprIR::Compare(CompareIR {
                    left,
                    ops,
                    comparators,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Subscript(subscript)) => {
                let target = match &subscript.target {
                    Some(target) => Box::new(Self::convert_expr(target)?),
                    None => {
                        return Err(Self::missing_expr(
                            "subscript expression",
                            "target",
                            &subscript.span,
                        ));
                    }
                };

                let index = match &subscript.subscript {
                    Some(subsript) => Box::new(Self::convert_expr(subsript)?),
                    None => {
                        return Err(Self::missing_expr(
                            "subscript expression",
                            "index",
                            &subscript.span,
                        ));
                    }
                };

                let span = match &subscript.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

                Ok(ExprIR::Subscript(SubscriptIR {
                    target,
                    subscript: index,
                    span,
                }))
            }

            Some(pb::expr_ir::Kind::Slice(slice)) => {
                let span = match &slice.span {
                    Some(span) => Some(Self::convert_span(span)),
                    None => None,
                };

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

                Ok(ExprIR::Slice(SliceIR {
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

    fn missing_expr(
        node: &str,
        field: &str,
        span: &Option<pb::SourceSpan>,
    ) -> Box<dyn std::error::Error> {
        let location = match span {
            Some(span) => format!(
                " at {}:{}:{}-{}:{}",
                span.file, span.line, span.col, span.end_line, span.end_col
            ),
            None => " (source span unavailable)".to_owned(),
        };

        format!("malformed {node}{location}: required protobuf field '{field}' is missing").into()
    }
}
