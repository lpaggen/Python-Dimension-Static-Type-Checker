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
            Self::And(values) => simplify_variadic(values, true),
            Self::Or(values) => simplify_variadic(values, false),
            Self::Not(value) => match value.simplify() {
                Self::Constant(value) => Self::Constant(!value),
                Self::Not(value) => *value,
                Self::And(values) => {
                    let negated: Vec<_> = values.iter().map(Self::not).collect();
                    let refs: Vec<_> = negated.iter().collect();
                    Self::or(&refs)
                }
                Self::Or(values) => {
                    let negated: Vec<_> = values.iter().map(Self::not).collect();
                    let refs: Vec<_> = negated.iter().collect();
                    Self::and(&refs)
                }
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

fn simplify_variadic(values: &[BoolExpr], conjunction: bool) -> BoolExpr {
    let mut flattened = Vec::new();

    for value in values.iter().map(BoolExpr::simplify) {
        match value {
            BoolExpr::And(inner) if conjunction => flattened.extend(inner),
            BoolExpr::Or(inner) if !conjunction => flattened.extend(inner),
            value => flattened.push(value),
        }
    }

    let absorbing = !conjunction;
    if flattened
        .iter()
        .any(|value| value.as_bool() == Some(absorbing))
        || has_complementary_values(&flattened)
    {
        return BoolExpr::Constant(absorbing);
    }

    let identity = conjunction;
    let mut unique = Vec::new();
    for value in flattened {
        if value.as_bool() != Some(identity) && !unique.contains(&value) {
            unique.push(value);
        }
    }

    // Absorption: a or (a and b) == a; a and (a or b) == a.
    let absorption_terms = unique.clone();
    unique.retain(|candidate| {
        let nested = match candidate {
            BoolExpr::And(inner) if !conjunction => Some(inner),
            BoolExpr::Or(inner) if conjunction => Some(inner),
            _ => None,
        };

        nested.is_none_or(|inner| {
            !absorption_terms
                .iter()
                .any(|other| other != candidate && inner.contains(other))
        })
    });

    while let Some((left, right, replacement)) = find_consensus_pair(&unique, conjunction) {
        unique.remove(right);
        unique.remove(left);
        if !unique.contains(&replacement) {
            unique.push(replacement);
        }

        if has_complementary_values(&unique) {
            return BoolExpr::Constant(absorbing);
        }
    }

    match unique.len() {
        0 => BoolExpr::Constant(identity),
        1 => unique.remove(0),
        _ if conjunction => BoolExpr::And(unique),
        _ => BoolExpr::Or(unique),
    }
}

fn find_consensus_pair(
    values: &[BoolExpr],
    conjunction: bool,
) -> Option<(usize, usize, BoolExpr)> {
    for left_index in 0..values.len() {
        for right_index in (left_index + 1)..values.len() {
            let left = term_parts(&values[left_index], conjunction);
            let right = term_parts(&values[right_index], conjunction);
            let common: Vec<_> = left
                .iter()
                .filter(|value| right.contains(value))
                .cloned()
                .collect();
            let left_only: Vec<_> = left
                .iter()
                .filter(|value| !common.contains(value))
                .collect();
            let right_only: Vec<_> = right
                .iter()
                .filter(|value| !common.contains(value))
                .collect();

            if left_only.len() != 1
                || right_only.len() != 1
                || !are_complements(left_only[0], right_only[0])
            {
                continue;
            }

            let replacement = match common.len() {
                0 => BoolExpr::Constant(!conjunction),
                1 => common[0].clone(),
                _ if conjunction => BoolExpr::Or(common),
                _ => BoolExpr::And(common),
            };

            return Some((left_index, right_index, replacement));
        }
    }

    None
}

fn term_parts(value: &BoolExpr, conjunction: bool) -> Vec<BoolExpr> {
    match value {
        BoolExpr::Or(values) if conjunction => values.clone(),
        BoolExpr::And(values) if !conjunction => values.clone(),
        value => vec![value.clone()],
    }
}

fn are_complements(left: &BoolExpr, right: &BoolExpr) -> bool {
    matches!(left, BoolExpr::Not(inner) if inner.as_ref() == right)
        || matches!(right, BoolExpr::Not(inner) if inner.as_ref() == left)
}

fn has_complementary_values(values: &[BoolExpr]) -> bool {
    values.iter().any(|value| match value {
        BoolExpr::Not(inner) => values.iter().any(|other| other == inner.as_ref()),
        value => values
            .iter()
            .any(|other| matches!(other, BoolExpr::Not(inner) if inner.as_ref() == value)),
    })
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
        fn write_expr(
            expression: &BoolExpr,
            formatter: &mut fmt::Formatter<'_>,
            parent_precedence: u8,
        ) -> fmt::Result {
            let precedence = match expression {
                BoolExpr::Implies(_, _) => 1,
                BoolExpr::Or(_) => 2,
                BoolExpr::And(_) => 3,
                BoolExpr::Not(_) => 4,
                _ => 5,
            };
            let parenthesize = precedence < parent_precedence;
            if parenthesize {
                formatter.write_str("(")?;
            }

            match expression {
                BoolExpr::Constant(value) => write!(formatter, "{value}"),
                BoolExpr::Variable(name) => formatter.write_str(
                    name.rsplit_once("::").map_or(name, |(_, label)| label),
                ),
                BoolExpr::Equal(left, right) => write!(formatter, "{left} == {right}"),
                BoolExpr::And(values) | BoolExpr::Or(values) => {
                    let operator = if matches!(expression, BoolExpr::And(_)) {
                        " and "
                    } else {
                        " or "
                    };
                    for (index, value) in values.iter().enumerate() {
                        if index > 0 {
                            formatter.write_str(operator)?;
                        }
                        write_expr(value, formatter, precedence)?;
                    }
                    Ok(())
                }
                BoolExpr::Not(value) => {
                    formatter.write_str("not ")?;
                    write_expr(value, formatter, precedence)
                }
                BoolExpr::Implies(left, right) => {
                    write_expr(left, formatter, precedence)?;
                    formatter.write_str(" implies ")?;
                    write_expr(right, formatter, precedence)
                }
            }?;

            if parenthesize {
                formatter.write_str(")")?;
            }
            Ok(())
        }

        write_expr(self, formatter, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::BoolExpr;

    #[test]
    fn simplifies_complementary_boolean_values() {
        let flag = BoolExpr::new_const("truthy_4_2::flag");

        assert_eq!(BoolExpr::or(&[&flag, &flag.not()]), BoolExpr::from_bool(true));
        assert_eq!(BoolExpr::and(&[&flag, &flag.not()]), BoolExpr::from_bool(false));
    }

    #[test]
    fn simplifies_nested_tautologies_and_absorption() {
        let a = BoolExpr::new_const("truthy_4_2::a");
        let b = BoolExpr::new_const("truthy_4_3::b");
        let left = BoolExpr::or(&[&a.not(), &b]);
        let right = BoolExpr::or(&[&b.not(), &a]);

        assert_eq!(BoolExpr::or(&[&left, &right]), BoolExpr::from_bool(true));
        assert_eq!(
            BoolExpr::or(&[&a, &BoolExpr::and(&[&a, &b])]),
            a,
        );
    }

    #[test]
    fn simplifies_cfg_style_partitioned_guards() {
        let a = BoolExpr::new_const("truthy_4_2::a");
        let b = BoolExpr::new_const("truthy_4_3::b");
        let a_and_b = BoolExpr::and(&[&a, &b]);
        let a_and_not_b = BoolExpr::and(&[&a, &b.not()]);
        let partition = BoolExpr::or(&[&a_and_b, &a_and_not_b, &a.not()]);

        assert_eq!(partition, BoolExpr::from_bool(true));
        assert_eq!(
            BoolExpr::and(&[&partition, &a, &b.not()]),
            BoolExpr::and(&[&a, &b.not()]),
        );
    }

    #[test]
    fn displays_source_label_instead_of_solver_identifier() {
        let flag = BoolExpr::new_const("truthy_4_2::flag");

        assert_eq!(flag.not().to_string(), "not flag");
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
