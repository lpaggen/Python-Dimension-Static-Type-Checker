use std::{cell::RefCell, collections::BTreeSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntExpr {
    Constant(i64),
    Variable(String),
    Add(Vec<IntExpr>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BoolExpr {
    Constant(bool),
    Variable(String),
    Equal(IntExpr, IntExpr),
    And(Vec<BoolExpr>),
    Or(Vec<BoolExpr>),
    Not(Box<BoolExpr>),
    Implies(Box<BoolExpr>, Box<BoolExpr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SatResult {
    Sat,
    Unsat,
    Unknown,
}

impl IntExpr {
    pub fn from_i64(value: i64) -> Self {
        Self::Constant(value)
    }

    pub fn add(values: &[&Self]) -> Self {
        Self::Add(values.iter().map(|value| (*value).clone()).collect())
    }

    pub fn eq(&self, other: impl Into<Self>) -> BoolExpr {
        BoolExpr::Equal(self.clone(), other.into())
    }

    fn collect_variables(&self, variables: &mut BTreeSet<(String, &'static str)>) {
        match self {
            Self::Variable(name) => {
                variables.insert((name.clone(), "Int"));
            }
            Self::Add(values) => {
                for value in values {
                    value.collect_variables(variables);
                }
            }
            Self::Constant(_) => {}
        }
    }

    fn to_smt2(&self) -> String {
        match self {
            Self::Constant(value) => value.to_string(),
            Self::Variable(name) => smt_identifier(name),
            Self::Add(values) => format!(
                "(+ {})",
                values
                    .iter()
                    .map(Self::to_smt2)
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        }
    }
}

impl From<&IntExpr> for IntExpr {
    fn from(value: &IntExpr) -> Self {
        value.clone()
    }
}

impl BoolExpr {
    pub fn from_bool(value: bool) -> Self {
        Self::Constant(value)
    }

    pub fn new_const(name: impl Into<String>) -> Self {
        Self::Variable(name.into())
    }

    pub fn and(values: &[&Self]) -> Self {
        Self::And(values.iter().map(|value| (*value).clone()).collect()).simplify()
    }

    pub fn or(values: &[&Self]) -> Self {
        Self::Or(values.iter().map(|value| (*value).clone()).collect()).simplify()
    }

    pub fn not(&self) -> Self {
        Self::Not(Box::new(self.clone())).simplify()
    }

    pub fn implies(&self, other: &Self) -> Self {
        Self::Implies(Box::new(self.clone()), Box::new(other.clone())).simplify()
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Constant(value) => Some(*value),
            _ => None,
        }
    }

    pub fn simplify(&self) -> Self {
        match self {
            Self::And(values) => {
                let values: Vec<_> = values.iter().map(Self::simplify).collect();
                if values.iter().any(|value| value.as_bool() == Some(false)) {
                    Self::Constant(false)
                } else {
                    let mut values: Vec<_> = values
                        .into_iter()
                        .filter(|value| value.as_bool() != Some(true))
                        .collect();
                    match values.len() {
                        0 => Self::Constant(true),
                        1 => values.remove(0),
                        _ => Self::And(values),
                    }
                }
            }
            Self::Or(values) => {
                let values: Vec<_> = values.iter().map(Self::simplify).collect();
                if values.iter().any(|value| value.as_bool() == Some(true)) {
                    Self::Constant(true)
                } else {
                    let mut values: Vec<_> = values
                        .into_iter()
                        .filter(|value| value.as_bool() != Some(false))
                        .collect();
                    match values.len() {
                        0 => Self::Constant(false),
                        1 => values.remove(0),
                        _ => Self::Or(values),
                    }
                }
            }
            Self::Not(value) => match value.simplify() {
                Self::Constant(value) => Self::Constant(!value),
                value => Self::Not(Box::new(value)),
            },
            Self::Implies(left, right) => match (left.simplify(), right.simplify()) {
                (Self::Constant(false), _) | (_, Self::Constant(true)) => Self::Constant(true),
                (Self::Constant(true), right) => right,
                (left, Self::Constant(false)) => left.not(),
                (left, right) => Self::Implies(Box::new(left), Box::new(right)),
            },
            Self::Equal(IntExpr::Constant(left), IntExpr::Constant(right)) => {
                Self::Constant(left == right)
            }
            value => value.clone(),
        }
    }

    fn collect_variables(&self, variables: &mut BTreeSet<(String, &'static str)>) {
        match self {
            Self::Variable(name) => {
                variables.insert((name.clone(), "Bool"));
            }
            Self::Equal(left, right) => {
                left.collect_variables(variables);
                right.collect_variables(variables);
            }
            Self::And(values) | Self::Or(values) => {
                for value in values {
                    value.collect_variables(variables);
                }
            }
            Self::Not(value) => value.collect_variables(variables),
            Self::Implies(left, right) => {
                left.collect_variables(variables);
                right.collect_variables(variables);
            }
            Self::Constant(_) => {}
        }
    }

    fn to_smt2(&self) -> String {
        match self {
            Self::Constant(true) => "true".to_owned(),
            Self::Constant(false) => "false".to_owned(),
            Self::Variable(name) => smt_identifier(name),
            Self::Equal(left, right) => format!("(= {} {})", left.to_smt2(), right.to_smt2()),
            Self::And(values) => smt_variadic("and", values),
            Self::Or(values) => smt_variadic("or", values),
            Self::Not(value) => format!("(not {})", value.to_smt2()),
            Self::Implies(left, right) => {
                format!("(=> {} {})", left.to_smt2(), right.to_smt2())
            }
        }
    }
}

fn smt_variadic(operator: &str, values: &[BoolExpr]) -> String {
    format!(
        "({operator} {})",
        values
            .iter()
            .map(BoolExpr::to_smt2)
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn smt_identifier(name: &str) -> String {
    format!("pdc_{}", name.as_bytes().iter().map(|byte| format!("{byte:02x}")).collect::<String>())
}

impl fmt::Display for IntExpr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_smt2())
    }
}

impl fmt::Display for BoolExpr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_smt2())
    }
}

pub struct Solver {
    assertions: RefCell<Vec<BoolExpr>>,
    scopes: RefCell<Vec<usize>>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            assertions: RefCell::new(Vec::new()),
            scopes: RefCell::new(Vec::new()),
        }
    }

    pub fn assert(&self, expression: &BoolExpr) {
        self.assertions.borrow_mut().push(expression.clone());
    }

    pub fn push(&self) {
        self.scopes.borrow_mut().push(self.assertions.borrow().len());
    }

    pub fn pop(&self, count: u32) {
        for _ in 0..count {
            if let Some(length) = self.scopes.borrow_mut().pop() {
                self.assertions.borrow_mut().truncate(length);
            }
        }
    }

    pub fn check(&self) -> SatResult {
        check_assertions(&self.assertions.borrow())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn check_assertions(assertions: &[BoolExpr]) -> SatResult {
    fn int_expr(expression: &IntExpr) -> z3::ast::Int {
        match expression {
            IntExpr::Constant(value) => z3::ast::Int::from_i64(*value),
            IntExpr::Variable(name) => z3::ast::Int::new_const(smt_identifier(name)),
            IntExpr::Add(values) => {
                let values: Vec<_> = values.iter().map(int_expr).collect();
                let refs: Vec<_> = values.iter().collect();
                z3::ast::Int::add(&refs)
            }
        }
    }

    fn bool_expr(expression: &BoolExpr) -> z3::ast::Bool {
        match expression {
            BoolExpr::Constant(value) => z3::ast::Bool::from_bool(*value),
            BoolExpr::Variable(name) => z3::ast::Bool::new_const(smt_identifier(name)),
            BoolExpr::Equal(left, right) => int_expr(left).eq(int_expr(right)),
            BoolExpr::And(values) => {
                let values: Vec<_> = values.iter().map(bool_expr).collect();
                let refs: Vec<_> = values.iter().collect();
                z3::ast::Bool::and(&refs)
            }
            BoolExpr::Or(values) => {
                let values: Vec<_> = values.iter().map(bool_expr).collect();
                let refs: Vec<_> = values.iter().collect();
                z3::ast::Bool::or(&refs)
            }
            BoolExpr::Not(value) => bool_expr(value).not(),
            BoolExpr::Implies(left, right) => bool_expr(left).implies(bool_expr(right)),
        }
    }

    let solver = z3::Solver::new();
    for assertion in assertions {
        solver.assert(bool_expr(assertion));
    }

    match solver.check() {
        z3::SatResult::Sat => SatResult::Sat,
        z3::SatResult::Unsat => SatResult::Unsat,
        z3::SatResult::Unknown => SatResult::Unknown,
    }
}

#[cfg(target_arch = "wasm32")]
fn check_assertions(assertions: &[BoolExpr]) -> SatResult {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = globalThis, js_name = pdcSolveSmt2)]
        fn solve_smt2(query: &str) -> String;
    }

    let assertions: Vec<_> = assertions.iter().map(BoolExpr::simplify).collect();
    if assertions
        .iter()
        .any(|assertion| assertion.as_bool() == Some(false))
    {
        return SatResult::Unsat;
    }
    if assertions
        .iter()
        .all(|assertion| assertion.as_bool() == Some(true))
    {
        return SatResult::Sat;
    }

    let mut variables = BTreeSet::new();
    for assertion in &assertions {
        assertion.collect_variables(&mut variables);
    }

    let mut query = String::from("(set-logic QF_LIA)\n");
    for (name, sort) in variables {
        query.push_str(&format!("(declare-const {} {sort})\n", smt_identifier(&name)));
    }
    for assertion in &assertions {
        query.push_str(&format!("(assert {})\n", assertion.to_smt2()));
    }
    query.push_str("(check-sat)\n");

    match solve_smt2(&query).trim() {
        "sat" => SatResult::Sat,
        "unsat" => SatResult::Unsat,
        _ => SatResult::Unknown,
    }
}
