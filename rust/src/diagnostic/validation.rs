use crate::{
    diagnostic::diagnostic::{Diagnostic, DiagnosticKind, Severity},
    ir::{expr::ExprIR, program_ir::ProgramIR, span_ir::SourceSpan, stmt::StmtIR},
};

fn unsupported(span: Option<SourceSpan>, feature: &str) -> Diagnostic {
    Diagnostic {
        severity: Severity::ERROR,
        span,
        kind: DiagnosticKind::UnsupportedFeature,
        message: format!("unsupported feature: {feature}"),
    }
}

fn validate_body(body: &[StmtIR], diagnostics: &mut Vec<Diagnostic>) {
    for statement in body {
        match statement {
            StmtIR::If(value) => {
                validate_body(&value.body, diagnostics);
                validate_body(&value.orelse, diagnostics);
            }
            StmtIR::Function(value) => validate_body(&value.body, diagnostics),

            StmtIR::Class(value) => diagnostics.push(unsupported(
                value.span.clone(),
                "class definitions",
            )),
            StmtIR::For(value) => diagnostics.push(unsupported(
                value.span.clone(),
                "for loops",
            )),
            StmtIR::While(value) => diagnostics.push(unsupported(
                value.span.clone(),
                "while loops",
            )),
            StmtIR::AsyncFor(value) => diagnostics.push(unsupported(
                value.span.clone(),
                "async for loops",
            )),

            StmtIR::Assign(value) => {
                if value.targets.iter().any(|target| !matches!(target, ExprIR::Name(_))) {
                    diagnostics.push(unsupported(value.span.clone(), "non-name assignment targets"));
                }
            }
            StmtIR::AnnAssign(value) => {
                if !matches!(&value.target, ExprIR::Name(_)) {
                    diagnostics.push(unsupported(
                        value.span.clone(),
                        "non-name annotated assignment targets",
                    ));
                }
            }
            // String-only expression statements are module/function docstrings.
            StmtIR::ExprStmt(value)
                if matches!(value.value.as_deref(), Some(ExprIR::Constant(crate::ir::expr::ConstantIR::StringLit(_)))) => {}
            StmtIR::ExprStmt(value) => diagnostics.push(unsupported(
                value.span.clone(),
                "expression statements",
            )),

            // These are analyzed or harmless structural statements.
            StmtIR::Import(_)
            | StmtIR::Return(_)
            | StmtIR::Pass(_) => {}

            other => diagnostics.push(unsupported(statement_span(other), statement_name(other))),
        }
    }
}

fn statement_name(statement: &StmtIR) -> &'static str {
    match statement {
        StmtIR::AugAssign(_) => "augmented assignments",
        StmtIR::Match(_) => "match statements",
        StmtIR::Delete(_) => "delete statements",
        StmtIR::Assert(_) => "assert statements",
        StmtIR::Raise(_) => "raise statements",
        StmtIR::AsyncFunctionDef(_) => "async functions",
        StmtIR::Global(_) => "global statements",
        StmtIR::Nonlocal(_) => "nonlocal statements",
        StmtIR::Break(_) => "break statements",
        StmtIR::Continue(_) => "continue statements",
        StmtIR::With(_) => "with statements",
        StmtIR::AsyncWith(_) => "async with statements",
        StmtIR::Try(_) => "try statements",
        StmtIR::TryStar(_) => "try-star statements",
        StmtIR::TypeAlias(_) => "type aliases",
        _ => "statement",
    }
}

fn statement_span(statement: &StmtIR) -> Option<SourceSpan> {
    match statement {
        StmtIR::AugAssign(value) => value.span.clone(),
        StmtIR::Match(value) => value.span.clone(),
        StmtIR::Delete(value) => value.span.clone(),
        StmtIR::Assert(value) => value.span.clone(),
        StmtIR::Raise(value) => value.span.clone(),
        StmtIR::AsyncFunctionDef(value) => value.span.clone(),
        StmtIR::Global(value) => value.span.clone(),
        StmtIR::Nonlocal(value) => value.span.clone(),
        StmtIR::Break(value) => value.span.clone(),
        StmtIR::Continue(value) => value.span.clone(),
        StmtIR::With(value) => value.span.clone(),
        StmtIR::AsyncWith(value) => value.span.clone(),
        StmtIR::Try(value) => value.span.clone(),
        StmtIR::TryStar(value) => value.span.clone(),
        StmtIR::TypeAlias(value) => value.span.clone(),
        _ => None,
    }
}

pub fn validate_programs(programs: &[ProgramIR]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for program in programs {
        validate_body(&program.body, &mut diagnostics);
    }
    diagnostics
}
