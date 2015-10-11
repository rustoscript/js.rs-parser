use std::fmt::{Display, Error, Formatter};


#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    Star,
    Plus,
    Minus,
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

impl Display for Exp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            &Exp::Int(i) => write!(fmt, "{}", i),
            &Exp::Float(f) => write!(fmt, "{}", f),
            &Exp::BinExp(ref e1, ref o, ref e2) => write!(fmt, "{} {} {}", e1, o, e2),
        }
    }
}
