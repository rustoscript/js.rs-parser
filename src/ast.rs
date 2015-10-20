use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    Plus,
    Minus,
    Star,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Const = 100,
    Mult = 60,
    Add = 50,
}

impl BinOp {
    pub fn precedence(&self) -> Precedence {
        match *self {
            BinOp::Plus | BinOp::Minus => Precedence::Add,
            BinOp::Star => Precedence::Mult,
        }
    }

    pub fn is_commutative(&self) -> bool {
        match *self {
            BinOp::Minus => false,
            _ => true
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            BinOp::Plus => write!(fmt, "+"),
            BinOp::Minus => write!(fmt, "-"),
            BinOp::Star => write!(fmt, "*"),
        }
    }
}

#[derive(Debug)]
pub enum Exp {
    BinExp(Box<Exp>, BinOp, Box<Exp>),
    Int(i64),
    Float(f64),
    Var(String),
}

impl Exp {
    pub fn precedence(&self) -> Precedence {
        match *self {
            Exp::BinExp(_, ref o, _) => o.precedence(),
            Exp::Int(_) | Exp::Float(_) | Exp::Var(_) => Precedence::Const,
        }
    }
}

impl Display for Exp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Exp::BinExp(ref e1, ref o, ref e2) => {
                let prec = self.precedence();

                // Put grouping parentheses if the left subexpression has a lower-precedence
                // operator, e.g. (1 + 2) * 3
                let left = if prec > e1.precedence() {
                    format!("({})", e1)
                } else {
                    format!("{}", e1)
                };

                let right_prec = e2.precedence();

                // Put grouping parentheses around the right subexpression if it has a
                // lower-precedence operator,  __OR__ if `o` is not commutative and the precedence
                // is the same, e.g. (1 + 2) * 3 __OR__ 1 - (2 + 3)
                let right = if prec > right_prec || (!o.is_commutative() && prec == right_prec) {
                    format!("({})", e2)
                } else {
                    format!("{}", e2)
                };

                write!(fmt, "{} {} {}", left, o, right)
            }
            Exp::Int(i)     => write!(fmt, "{}", i),
            Exp::Float(f)   => write!(fmt, "{}", f),
            Exp::Var(ref v) => write!(fmt, "{}", v),

        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Exp),
    Decl(String, Exp),
}

impl Display for Stmt {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Stmt::Assign(ref v, ref exp) => write!(fmt, "{} = {};\n", v, exp),
            Stmt::Decl(ref v, ref exp) => write!(fmt, "var {} = {};\n", v, exp),
        }
    }
}
