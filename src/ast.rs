use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    And,
    Minus,
    Or,
    Plus,
    Slash,
    Star,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Const = 110,
    Sign = 100,
    Inc = 90,
    Mult = 60,
    Add = 50,
    And = 40,
    Or = 30,
}

impl BinOp {
    pub fn precedence(&self) -> Precedence {
        match *self {
            BinOp::And => Precedence::And,
            BinOp::Or => Precedence::Or,
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
            BinOp::And => write!(fmt, "&&"),
            BinOp::Minus => write!(fmt, "-"),
            BinOp::Or => write!(fmt, "||"),
            BinOp::Plus => write!(fmt, "+"),
            BinOp::Slash => write!(fmt, "/"),
            BinOp::Star => write!(fmt, "*"),
        }
    }
}

#[derive(Debug)]
pub enum Exp {
    BinExp(Box<Exp>, BinOp, Box<Exp>),
    Bool(bool),
    Float(f64),
    Neg(Box<Exp>),
    Pos(Box<Exp>),
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
            Exp::Bool(_) | Exp::Float(_) | Exp::Undefined | Exp::Var(_) => Precedence::Const,
            Exp::Neg(_) | Exp::Pos(_) => Precedence::Sign,
            Exp::PostDec(_) | Exp::PostInc(_) | Exp::PreDec(_) | Exp::PreInc(_) => Precedence::Inc,
        }
    }
}

macro_rules! group {
    ($e:expr, $p:expr) => {
        if $e.precedence() < $p {
            format!("({})", $e)
        } else {
            format!("{}", $e)
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
            Exp::Bool(b) => write!(fmt, "{}", b),
            Exp::Float(f) => write!(fmt, "{}", f),
            Exp::Neg(ref e) => write!(fmt, "-{}", group!(e, Precedence::Sign)),
            Exp::Pos(ref e) => write!(fmt, "+{}", group!(e, Precedence::Sign)),
            Exp::PostDec(ref e) => write!(fmt, "{}--", group!(e, Precedence::Inc)),
            Exp::PostInc(ref e) => write!(fmt, "{}++", group!(e, Precedence::Inc)),
            Exp::PreDec(ref e) => write!(fmt, "--{}", group!(e, Precedence::Inc)),
            Exp::PreInc(ref e) => write!(fmt, "++{}", group!(e, Precedence::Inc)),
            Exp::Undefined => write!(fmt, "undefined"),
            Exp::Var(ref v) => write!(fmt, "{}", v),
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Exp),
    BareExp(Exp),
    Decl(String, Exp),
    Seq(Box<Stmt>, Box<Stmt>),
}

impl Display for Stmt {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Stmt::Assign(ref v, ref exp) => write!(fmt, "{} = {};\n", v, exp),
            Stmt::Decl(ref v, ref exp) => write!(fmt, "var {} = {};\n", v, exp),
            Stmt::BareExp(ref exp) => write!(fmt, "{};\n", exp),
            Stmt::Seq(ref s1, ref s2) => write!(fmt, "{}{}", s1, s2),
        }
    }
}
