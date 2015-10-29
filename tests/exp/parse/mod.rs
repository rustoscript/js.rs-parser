use ::mostlyeq::MostlyEq;

use jsrs_parser::lalr::parse_Exp;
use jsrs_parser::ast::Exp::*;
use jsrs_parser::ast::BinOp::*;


#[test]
fn bare_float() {
    assert_mostly_eq!(Float(-11.01), &parse_exp!("-11.01"));
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
    let with_plus = BinExp(Box::new(Float(3.7)), Plus, Box::new(Float(-4.0)));
    let with_star = BinExp(Box::new(Float(3.7)), Star, Box::new(Float(-4.0)));
    let with_minus = BinExp(Box::new(Float(3.7)), Minus, Box::new(Float(-4.0)));
    let with_slash = BinExp(Box::new(Float(3.7)), Slash, Box::new(Float(-4.0)));

    assert_mostly_eq!(with_plus, &parse_exp!("3.7 + -4"));
    assert_mostly_eq!(with_star, &parse_exp!("3.7 * -4"));
    assert_mostly_eq!(with_slash, &parse_exp!("3.7 / -4"));
    assert_mostly_eq!(with_plus, &parse_exp!("(3.7 + -4)"));
    assert_mostly_eq!(with_star, &parse_exp!("(3.7) * -4"));
    assert_mostly_eq!(with_minus, &parse_exp!("(3.7) - (-4)"));
    assert_mostly_eq!(with_slash, &parse_exp!("3.7 / (-4)"));
    assert_mostly_eq!(with_plus, &parse_exp!("((3.7) + ((4)))"));
}

#[test]
fn multi_binop_exprs() {
    assert_mostly_eq!(exp!(exp!(Float(3.7), Star, Float(-4.0)), Minus, Float(2.0)),
                      &parse_exp!("3.7 * -4 - 2"));
    assert_mostly_eq!(exp!(Float(3.7), Slash, exp!(Float(-4.0), Minus, Float(-2.0))),
                      &parse_exp!("3.7 / (-4 - 2)"));
    assert_mostly_eq!(exp!(Float(3.7), Plus, exp!(Float(-4.0), Star, Float(2.0))),
                      &parse_exp!("3.7 + -4 * 2"));
    assert_mostly_eq!(exp!(exp!(Float(3.7), Plus, Float(-4.0)), Slash, Float(2.0)),
                      &parse_exp!("(3.7 + -4) / 2"));
    assert_mostly_eq!(exp!(exp!(Float(3.7), Plus, Float(-4.0)), Plus, Float(2.0)),
                      &parse_exp!("3.7 + -4 + 2"));
    assert_mostly_eq!(exp!(Float(3.7), Plus, exp!(Float(-4.0), Plus, Float(2.0))),
                      &parse_exp!("3.7 + (-4 + 2)"));
    assert_mostly_eq!(exp!(exp!(Float(3.7), Star, Float(4.0)), Plus, exp!(Float(2.0), Star, Float(11.0))),
                      &parse_exp!("3.7 * 4 + 2 / 11"));
    assert_mostly_eq!(exp!(exp!(Float(3.7), Star, exp!(Float(4.0), Plus, Float(2.0))), Slash, Float(11.0)),
                      &parse_exp!("(3.7 / (4 + 2) * 11)"));
    assert_mostly_eq!(exp!(Float(3.7), Slash, exp!(Float(4.0), Plus, exp!(Float(2.0), Star, Float(11.0)))),
                      &parse_exp!("3.7 * (4 + 2 * 11)"));
    assert_mostly_eq!(exp!(Float(3.7), Star, exp!(exp!(Float(4.0), Plus, Float(2.0)), Slash, Float(11.0))),
                      &parse_exp!("3.7 * ((4 + 2) / 11)"));
}

#[test]
fn undefined() {
    assert_mostly_eq!(Undefined, &parse_exp!("undefined"));
}
