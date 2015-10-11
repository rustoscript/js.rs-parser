use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    Star,
    Plus,
    Minus,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Const = 100,
    Mult = 60,
    Add = 50,
}

impl BinOp {
    pub fn precedence(&self) -> Precedence {
        match self {
            &BinOp::Star => Precedence::Mult,
            &BinOp::Plus | &BinOp::Minus => Precedence::Add,
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            &BinOp::Star => write!(fmt, "*"),
            &BinOp::Plus => write!(fmt, "+"),
            &BinOp::Minus => write!(fmt, "-"),
        }
    }
}

#[derive(Debug)]
pub enum Exp {
    Int(i64),
    Float(f64),
    BinExp(Box<Exp>, BinOp, Box<Exp>),
}

impl Exp {
    pub fn precedence(&self) -> Precedence {
        match self {
            &Exp::Int(_) | &Exp::Float(_) => Precedence::Const,
            &Exp::BinExp(_, ref o, _) => o.precedence(),
        }
    }
}

impl Display for Exp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            &Exp::Int(i) => write!(fmt, "{}", i),
            &Exp::Float(f) => write!(fmt, "{}", f),
            &Exp::BinExp(ref e1, ref o, ref e2) => {
                let prec = self.precedence();

                let left = if prec > e1.precedence() {
                    format!("({})", e1)
                } else {
                    format!("{}", e1)
                };

                let right = if prec > e2.precedence() {
                    format!("({})", e2)
                } else {
                    format!("{}", e2)
                };

                write!(fmt, "{} {} {}", left, o, right)
            }
        }
    }
}
