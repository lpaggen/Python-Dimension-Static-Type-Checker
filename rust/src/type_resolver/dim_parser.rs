use crate::type_resolver::parse_error::DimParseError;

pub struct DimParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> DimParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn consume(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn skip_ws(&mut self) {
        while self.peek().is_some_and(|c: char| c.is_whitespace()) {
            self.bump();
        }
    }

    fn is_end(&self) -> bool {
        self.pos >= self.input.len()  // TODO check if not need  >
    }

    // !! only for dimension annotations
    fn parse_integer(&mut self) -> Result<i64, DimParseError> {
        match self.peek() {
            Some(c) => {
                self.bump();
                Ok(c.to_digit(10).unwrap() as i64)
            },

            None => Err(self.error("expected integer")) // TODO revise
        }
    }

    fn parse_symbolic(&mut self) -> Result<String, DimParseError> {
        match self.peek() {
            Some(c) => {
                self.bump();
                Ok(c.to_string())
            },

            None => Err(self.error("expected symbol")) // TODO revise
        }
    }

    pub fn parse(&mut self) -> Result<ShapeExpr, DimParseError> {
        let mut dims = Vec::new();

        dims.push(self.parse_expr()?);

        loop {
            self.skip_ws();

            if self.is_end() {
                break;
            }

            if self.peek() != Some('@') {
                return Err(self.error("expected '@'"));
            }

            self.bump();
            dims.push(self.parse_expr()?);
        }

        Ok(ShapeExpr { dims })
    }

    pub fn parse_expr(&mut self) -> Result<DimExpr, DimParseError> {
        let mut left = self.parse_term()?;

        loop {
            match self.peek() {
                Some('+') => {
                    self.bump();
                    let right = self.parse_term()?;
                    left = DimExpr::Add(Box::new(left), Box::new(right));
                }

                Some('-') => {
                    self.bump();
                    let right = self.parse_term()?;
                    left = DimExpr::Sub(Box::new(left), Box::new(right));
                }

                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<DimExpr, DimParseError> {
        let mut left = self.parse_factor()?;

        loop {
            match self.peek() {
                Some('*') => {
                    self.bump();
                    let right = self.parse_factor()?;
                    left = DimExpr::Mul(Box::new(left), Box::new(right));
                }

                Some('/') => {
                    self.bump();
                    let right = self.parse_factor()?;
                    left = DimExpr::Div(Box::new(left), Box::new(right));
                }

                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<DimExpr, DimParseError> {
        self.skip_ws();

        match self.peek() {
            Some('(') => {
                self.bump();
                let expr = self.parse_expr()?;
                self.skip_ws();
                if self.peek() != Some(')') {
                    return Err(self.error("expected closing parenthesis"));
                }

                self.bump();

                Ok(expr)
            }

            Some(c) if c.is_ascii_digit() => {
                let value = self.parse_integer()?;
                Ok(DimExpr::Const(value))
            }

            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                let value = self.parse_symbolic()?;
                Ok(DimExpr::Symbol(value))
            }

            _ => Err(self.error("invalid character in tensor shape annotation")) // fix
        }
    }

    fn error(&self, message: &str) -> DimParseError {
        DimParseError {
            pos: self.pos,
            message: message.to_string(),
        }
    }

}

#[derive(Debug)]
pub enum DimExpr {
    Add(Box<DimExpr>, Box<DimExpr>),
    Sub(Box<DimExpr>, Box<DimExpr>),
    Div(Box<DimExpr>, Box<DimExpr>),
    Mul(Box<DimExpr>, Box<DimExpr>),
    Const(i64),
    Symbol(String)
}

impl DimExpr {
    pub fn dimexpr_to_z3(expr: &DimExpr) -> z3::ast::Int {
        match expr {
            DimExpr::Const(i) => {
                z3::ast::Int::from_i64(*i)
            },

            DimExpr::Symbol(s) => {
                z3::ast::Int::new_const(s.clone())
            },
            
            DimExpr::Add(lhs, rhs) => {
                DimExpr::dimexpr_to_z3(lhs) + DimExpr::dimexpr_to_z3(rhs)
            },

            DimExpr::Sub(lhs, rhs) => {
                DimExpr::dimexpr_to_z3(lhs) - DimExpr::dimexpr_to_z3(rhs)
            },

            DimExpr::Div(lhs, rhs) => {
                DimExpr::dimexpr_to_z3(lhs) / DimExpr::dimexpr_to_z3(rhs)
            },

            DimExpr::Mul(lhs, rhs) => {
                DimExpr::dimexpr_to_z3(lhs) * DimExpr::dimexpr_to_z3(rhs)
            },
        }
    }
}

#[derive(Debug)]
pub struct ShapeExpr {
    dims: Vec<DimExpr>,
}
