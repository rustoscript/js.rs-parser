use ::mostlyeq::MostlyEq;
use jsrs_parser::ast::BinOp::*;
use jsrs_parser::ast::Exp::*;
use jsrs_parser::ast::Stmt::*;
use jsrs_parser::lalr::parse_Stmt;

#[test]
fn assign_num() {
    assert_mostly_eq!(assign!("x", Int(0)), &parse_stmt!("x = 0;"));
    assert_mostly_eq!(assign!("num", Int(3)), &parse_stmt!("num = 3;"));
    assert_mostly_eq!(assign!("some_num", Int(62)), &parse_stmt!("some_num = 62;"));
    assert_mostly_eq!(assign!("anotherNumber", Int(7798)), &parse_stmt!("anotherNumber = 7798;"));
    assert_mostly_eq!(assign!("_x", Int(-5)), &parse_stmt!("_x = -5;"));
    assert_mostly_eq!(assign!("_", Int(-1234)), &parse_stmt!("_ = -1234;"));
    assert_mostly_eq!(assign!("x2", Float(0.5)), &parse_stmt!("x1 = 0.5;"));
    assert_mostly_eq!(assign!("NUMBER", Float(-4.25)), &parse_stmt!("NUMBER = -4.25;"));
    assert_mostly_eq!(assign!("N_4", Float(4182.125)), &parse_stmt!("N_4 = -4182.125;"));
}

#[test]
fn assign_single_binop() {
    assert_mostly_eq!(assign!("x",
        exp!(var!("y"), Plus, Int(7))), &parse_stmt!("x = y + 7;"));
    assert_mostly_eq!(assign!("num",
        exp!(Int(72), Minus, Float(3.5))), &parse_stmt!("num = 72 * 3.5;"));
    assert_mostly_eq!(assign!("some_num",
        exp!(Float(0.125), Star, var!("otherNum"))), &parse_stmt!("some_num = 0.125 * otherNum;"));
    assert_mostly_eq!(assign!("anotherNumber",
        exp!(var!("_7"), Plus, var!("thing22"))), &parse_stmt!("anotherNumber = _7 + thing22;"));
}

#[test]
fn assign_multi_binop() {
    assert_mostly_eq!(assign!("_x", exp!(exp!(var!("_22"), Star, Int(-4)), Minus, Int(2))),
        &parse_stmt!("_x = _22 * -4 - 2;"));
    assert_mostly_eq!(assign!("_", exp!(Float(3.7), Star, exp!(var!("e_4_4"), Minus, Int(-2)))),
        &parse_stmt!("_ = 3.7 * (e_4_4 - 2);"));
    assert_mostly_eq!(assign!("x2", exp!(Float(3.7), Plus, exp!(Int(-4), Star, var!("numb3r9")))),
        &parse_stmt!("x2 = 3.7 + -4 * numb3r9;"));
    assert_mostly_eq!(assign!("NUMBER", exp!(exp!(var!("O_k"), Plus, var!("_Ok")), Star, Int(2))),
        &parse_stmt!("NUMBER = (3.7 + O_k) * _Ok;"));
    assert_mostly_eq!(assign!("N_4", exp!(exp!(var!("OkDk"), Plus, Int(-4)), Plus, var!("h2o"))),
        &parse_stmt!("N_4 = OkDk + -4 + h2o;"));
    assert_mostly_eq!(assign!("x", exp!(Float(3.7), Plus, exp!(Int(-4), Plus, Int(2)))),
        &parse_stmt!("x = 3.7 + (-4 + 2);"));
    assert_mostly_eq!(assign!("num", exp!(exp!(Float(3.7), Star, Int(4)), Plus, exp!(Int(2), Star, Int(11)))),
        &parse_stmt!("num = 3.7 * 4 + 2 * 11;"));
    assert_mostly_eq!(assign!("some_num", exp!(exp!(Float(3.7), Star, exp!(Int(4), Plus, Int(2))), Star, Int(11))),
        &parse_stmt!("some_num = (3.7 * (4 + 2) * 11);"));
    assert_mostly_eq!(assign!("anotherNumber", exp!(Float(3.7), Star, exp!(Int(4), Plus, exp!(Int(2), Star, Int(11))))),
        &parse_stmt!("anotherNumber = 3.7 * (4 + 2 * 11);"));
    assert_mostly_eq!(assign!("e_4_4", exp!(Float(3.7), Star, exp!(exp!(Int(4), Plus, Int(2)), Star, Int(11)))),
        &parse_stmt!("e_4_4 = 3.7 * ((4 + 2) * 11);"));
}
