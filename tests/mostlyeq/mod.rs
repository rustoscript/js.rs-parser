use jsrs_common::ast::{Exp, Stmt};
use jsrs_common::ast::Exp::*;
use jsrs_common::ast::Stmt::*;
use nalgebra::ApproxEq;

macro_rules! assert_mostly_eq {
    ($e1:expr, $e2:expr) => { assert!($e1.mostly_eq($e2)) }
}

// Checks for equality between two values where an exact equality cannot be found (i.e. with
// floats)
pub trait MostlyEq {
    fn mostly_eq(&self, other: &Self) -> bool;
}

impl MostlyEq for Exp {
    fn mostly_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&BinExp(ref a1, ref o1, ref b1), &BinExp(ref a2, ref o2, ref b2)) =>
                (&*a1).mostly_eq(a2) && o1 == o2 && (&*b1).mostly_eq(&*b2),
            (&Bool(b1), &Bool(b2)) => b1 == b2,
            (&Float(f1), &Float(f2)) => f1.approx_eq(&f2) || (f1.is_nan() && f2.is_nan()),
            (&Neg(ref e1), &Neg(ref e2)) |
            (&Pos(ref e1), &Pos(ref e2)) |
            (&PostDec(ref e1), &PostDec(ref e2)) |
            (&PostInc(ref e1), &PostInc(ref e2)) |
            (&PreDec(ref e1), &PreDec(ref e2)) |
            (&PreInc(ref e1), &PreInc(ref e2)) => e1.mostly_eq(e2),
            (&Undefined, &Undefined) => true,
            (&Var(ref v1), &Var(ref v2)) => v1 == v2,
            _ => false
        }
    }
}

impl MostlyEq for Stmt {
    fn mostly_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Assign(ref v1, ref e1), &Assign(ref v2, ref e2)) |
            (&Decl(ref v1, ref e1), &Decl(ref v2, ref e2)) => v1 == v2 && e1.mostly_eq(e2),
            (&BareExp(ref e1), &BareExp(ref e2)) => e1.mostly_eq(e2),
            (&If(ref e1, ref s1, ref vec1, ref opt1), &If(ref e2, ref s2, ref vec2, ref opt2)) => {
                if !e1.mostly_eq(e2) || !s1.mostly_eq(s2) || vec1.len() != vec2.len() {
                    return false;
                }

                match (opt1, opt2) {
                    (&Some(ref s1), &Some(ref s2)) => if !s1.mostly_eq(s2) {
                        return false
                    },
                    (&None, &None) => (),
                    _ => return false
                };

                for (&(ref ve1, ref vs1), &(ref ve2, ref vs2)) in vec1.iter().zip(vec2.iter()) {
                    if !ve1.mostly_eq(&ve2) || !vs1.mostly_eq(&vs2) {
                        return false;
                    }
                }

                true
            }
            _ => false
        }
    }
}
