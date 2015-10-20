use jsrs_parser::ast::Exp::{self, BinExp, Float, Int, Var };
use nalgebra::ApproxEq;

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
            (&Float(f1), &Float(f2)) => f1.approx_eq(&f2),
            (&Int(i1), &Int(i2)) => i1 == i2,
            (&Var(ref v1), &Var(ref v2)) => v1 == v2,
            _ => false
        }
    }
}
