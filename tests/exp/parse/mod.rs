use ::mostlyeq::MostlyEq;

use jsrs_parser::lalr::parse_Exp;
use jsrs_parser::ast::Exp::*;
use jsrs_parser::ast::BinOp::*;


#[test]
fn bare_int() {
    assert_mostly_eq!(Int(-4), &parse_exp!("-4"));
    assert_mostly_eq!(Int(0), &parse_exp!("-0"));
    assert_mostly_eq!(Int(0), &parse_exp!("0"));
    assert_mostly_eq!(Int(74), &parse_exp!("74"));
}

#[test]
fn bare_float() {
    assert_mostly_eq!(Float(-11.01), &parse_exp!("-4.01"));
    assert_mostly_eq!(Float(-4.0), &parse_exp!("-4.0"));
    assert_mostly_eq!(Float(-4.0221), &parse_exp!("-4.0221"));
    assert_mostly_eq!(Float(-0.56), &parse_exp!("-0.56"));
    assert_mostly_eq!(Float(0.0), &parse_exp!("-0.0"));
    assert_mostly_eq!(Float(0.0), &parse_exp!("0.0"));
    assert_mostly_eq!(Float(0.1), &parse_exp!("0.1"));
    assert_mostly_eq!(Float(0.733), &parse_exp!("0.0733"));
    assert_mostly_eq!(Float(8.927), &parse_exp!("8.927"));
    assert_mostly_eq!(Float(34.68), &parse_exp!("34.68"));
}

#[test]
fn single_binop_exprs() {
    let with_plus = BinExp(Box::new(Float(3.7)), Plus, Box::new(Int(-4)));
    let with_star = BinExp(Box::new(Float(3.7)), Star, Box::new(Int(-4)));
    let with_minus = BinExp(Box::new(Float(3.7)), Minus, Box::new(Int(-4)));

    assert_mostly_eq!(with_plus, &parse_exp!("3.7 + -4"));
    assert_mostly_eq!(with_star, &parse_exp!("3.7 * -4"));
    assert_mostly_eq!(with_minus, &parse_exp!("3.7 - -4"));
    assert_mostly_eq!(with_plus, &parse_exp!("(3.7 + -4)"));
    assert_mostly_eq!(with_star, &parse_exp!("(3.7) * -4"));
    assert_mostly_eq!(with_minus, &parse_exp!("(3.7) - (-4)"));
    assert_mostly_eq!(with_star, &parse_exp!("3.7 * (-4)"));
    assert_mostly_eq!(with_minus, &parse_exp!("((3.7) - ((4)))"));
}

#[test]
fn multi_binop_exprs() {
    assert_mostly_eq!(exp!(exp!(Float(3.7), Star, Int(-4)), Minus, Int(2)),
                      &parse_exp!("3.7 * -4 - 2"));
    assert_mostly_eq!(exp!(Float(3.7), Star, exp!(Int(-4), Minus, Int(-2))),
                      &parse_exp!("3.7 * (-4 - 2)"));
    assert_mostly_eq!(exp!(Float(3.7), Plus, exp!(Int(-4), Star, Int(2))),
                      &parse_exp!("3.7 + -4 * 2"));
    assert_mostly_eq!(exp!(exp!(Float(3.7), Plus, Int(-4)), Star, Int(2)),
                      &parse_exp!("(3.7 + -4) * 2"));
    assert_mostly_eq!(exp!(exp!(Float(3.7), Plus, Int(-4)), Plus, Int(2)),
                      &parse_exp!("3.7 + -4 + 2"));
    assert_mostly_eq!(exp!(Float(3.7), Plus, exp!(Int(-4), Plus, Int(2))),
                      &parse_exp!("3.7 + (-4 + 2)"));
    assert_mostly_eq!(exp!(exp!(Float(3.7), Star, Int(4)), Plus, exp!(Int(2), Star, Int(11))),
                      &parse_exp!("3.7 * 4 + 2 * 11"));
    assert_mostly_eq!(exp!(exp!(Float(3.7), Star, exp!(Int(4), Plus, Int(2))), Star, Int(11)),
                      &parse_exp!("(3.7 * (4 + 2) * 11)"));
    assert_mostly_eq!(exp!(Float(3.7), Star, exp!(Int(4), Plus, exp!(Int(2), Star, Int(11)))),
                      &parse_exp!("3.7 * (4 + 2 * 11)"));
    assert_mostly_eq!(exp!(Float(3.7), Star, exp!(exp!(Int(4), Plus, Int(2)), Star, Int(11))),
                      &parse_exp!("3.7 * ((4 + 2) * 11)"));
}
