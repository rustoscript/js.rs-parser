

use nalgebra::ApproxEq;
use jsrs_parser::arith::parse_Exp;
use jsrs_parser::ast::Exp::{self, Int, Float, BinExp};
use jsrs_parser::ast::BinOp::{Star, Plus, Minus};

// Checks for equality between two values where an exact equality cannot be found (i.e. with
// floats)
trait MostlyEq {
    fn mostly_eq(&self, other: &Self) -> bool;
}

impl MostlyEq for Exp {
    fn mostly_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Int(i1), &Int(i2)) => i1 == i2,
            (&Float(f1), &Float(f2)) => f1.approx_eq(&f2),
            (&BinExp(ref a1, ref o1, ref b1), &BinExp(ref a2, ref o2, ref b2)) =>
                (&*a1).mostly_eq(a2) && o1 == o2 && (&*b1).mostly_eq(&*b2),
            _ => false
        }
    }
}

macro_rules! assert_mostly_eq {
    ($e1:expr, $e2:expr) => { $e1.mostly_eq($e2) }
}

#[test]
fn bare_int() {
    assert_mostly_eq!(Int(-4), parse_exp!("-4"));
    assert_mostly_eq!(Int(0), parse_exp!("-0"));
    assert_mostly_eq!(Int(0), parse_exp!("0"));
    assert_mostly_eq!(Int(74), parse_exp!("74"));
}

#[test]
fn bare_float() {
    assert_mostly_eq!(Float(-11.01), parse_exp!("-4.01"));
    assert_mostly_eq!(Float(-4.0), parse_exp!("-4.0"));
    assert_mostly_eq!(Float(-4.0221), parse_exp!("-4.0221"));
    assert_mostly_eq!(Float(-0.56), parse_exp!("-0.56"));
    assert_mostly_eq!(Float(0.0), parse_exp!("-0.0"));
    assert_mostly_eq!(Float(0.0), parse_exp!("0.0"));
    assert_mostly_eq!(Float(0.1), parse_exp!("0.1"));
    assert_mostly_eq!(Float(0.733), parse_exp!("0.0733"));
    assert_mostly_eq!(Float(8.927), parse_exp!("8.927"));
    assert_mostly_eq!(Float(34.68), parse_exp!("34.68"));
}

#[test]
fn single_binop_exprs() {
    let with_plus = BinExp(Box::new(Float(3.7)), Plus, Box::new(Int(-4)));
    let with_star = BinExp(Box::new(Float(3.7)), Star, Box::new(Int(-4)));
    let with_minus = BinExp(Box::new(Float(3.7)), Minus, Box::new(Int(-4)));

    assert_mostly_eq!(with_plus, parse_exp!("3.7 + -4"));
    assert_mostly_eq!(with_star, parse_exp!("3.7 * -4"));
    assert_mostly_eq!(with_minus, parse_exp!("3.7 - -4"));
    assert_mostly_eq!(with_plus, parse_exp!("(3.7 + -4)"));
    assert_mostly_eq!(with_star, parse_exp!("(3.7) * -4"));
    assert_mostly_eq!(with_minus, parse_exp!("(3.7) - (-4)"));
    assert_mostly_eq!(with_star, parse_exp!("3.7 * (-4)"));
    assert_mostly_eq!(with_minus, parse_exp!("((3.7) - ((4)))"));
}

#[test]
fn multi_binop_exprs() {
    assert_mostly_eq!(exp!(exp!(Float(3.7), Star, Int(-4)), Minus, Int(2)),
                      parse_exp!("3.7 * -4 - 2"));
    assert_mostly_eq!(exp!(Float(3.7), Star, exp!(Int(-4), Minus, Int(-2))),
                      parse_exp!("3.7 * (-4 - 2)"));
    assert_mostly_eq!(exp!(Float(3.7), Plus, exp!(Int(-4), Star, Int(2))),
                      parse_exp!("3.7 + -4 * 2"));
    assert_mostly_eq!(exp!(exp!(Float(3.7), Plus, Int(-4)), Star, Int(2)),
                      parse_exp!("(3.7 + -4) * 2"));
    assert_mostly_eq!(exp!(exp!(Float(3.7), Plus, Int(-4)), Plus, Int(2)),
                      parse_exp!("3.7 + -4 + 2"));
    assert_mostly_eq!(exp!(Float(3.7), Plus, exp!(Int(-4), Plus, Int(2))),
                      parse_exp!("3.7 + (-4 + 2)"));
    assert_mostly_eq!(exp!(exp!(Float(3.7), Star, Int(4)), Plus, exp!(Int(2), Star, Int(11))),
                      parse_exp!("3.7 * 4 + 2 * 11"));
    assert_mostly_eq!(exp!(exp!(Float(3.7), Star, exp!(Int(4), Plus, Int(2))), Star, Int(11)),
                      parse_exp!("(3.7 * (4 + 2) * 11)"));
    assert_mostly_eq!(exp!(Float(3.7), Star, exp!(Int(4), Plus, exp!(Int(2), Star, Int(11)))),
                      parse_exp!("3.7 * (4 + 2 * 11)"));
    assert_mostly_eq!(exp!(Float(3.7), Star, exp!(exp!(Int(4), Plus, Int(2)), Star, Int(11))),
                      parse_exp!("3.7 * ((4 + 2) * 11)"));
}
