use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    Minus,
    Plus,
    Slash,
    Star,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Const = 110,
    Inc = 100,
    Mult = 60,
    Add = 50,
}

impl BinOp {
    pub fn precedence(&self) -> Precedence {
        match *self {
            BinOp::Minus | BinOp::Plus => Precedence::Add,
            BinOp::Slash | BinOp::Star => Precedence::Mult,
        }
    }

    pub fn is_commutative(&self) -> bool {
        match *self {
            BinOp::Minus | BinOp::Slash => false,
            _ => true
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            BinOp::Minus => write!(fmt, "-"),
            BinOp::Plus => write!(fmt, "+"),
            BinOp::Slash => write!(fmt, "/"),
            BinOp::Star => write!(fmt, "*"),
        }
    }
}

#[derive(Debug)]
pub enum Exp {
    BinExp(Box<Exp>, BinOp, Box<Exp>),
    Float(f64),
    PostDec(Box<Exp>),
    PostInc(Box<Exp>),
    PreDec(Box<Exp>),
    PreInc(Box<Exp>),
    Undefined,
    Var(String),
}

impl Exp {
    pub fn precedence(&self) -> Precedence {
        match *self {
            Exp::BinExp(_, ref o, _) => o.precedence(),
            Exp::PostDec(_) | Exp::PostInc(_) | Exp::PreDec(_) | Exp::PreInc(_) => Precedence::Inc,
            Exp::Undefined | Exp::Float(_) | Exp::Var(_) => Precedence::Const,
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
            Exp::Float(f)   => write!(fmt, "{}", f),
            Exp::Var(ref v) => write!(fmt, "{}", v),
            Exp::PostDec(ref e) => write!(fmt, "{}--", e),
            Exp::PostInc(ref e) => write!(fmt, "{}++", e),
            Exp::PreDec(ref e) => write!(fmt, "--{}", e),
            Exp::PreInc(ref e) => write!(fmt, "++{}", e),
            Exp::Undefined => write!(fmt, "undefined"),
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Exp),
    BareExp(Exp),
    Decl(String, Exp),
}

impl Display for Stmt {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Stmt::Assign(ref v, ref exp) => write!(fmt, "{} = {};\n", v, exp),
            Stmt::Decl(ref v, ref exp) => write!(fmt, "var {} = {};\n", v, exp),
            Stmt::BareExp(ref exp) => write!(fmt, "{};\n", exp),
        }
    }
}
