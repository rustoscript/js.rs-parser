use jsrs_parser::ast::BinOp::*;
use jsrs_parser::ast::Exp::*;

macro_rules! format_exp {
    ($e1:expr, $o:expr, $e2:expr) => { &format!("{}", exp!($e1, $o, $e2)) }
}

#[test]
fn constants() {
    // Floating point values specifically chosen so fractional part can be perfectly represented
    // by floating-point numbers.
    assert_eq!("12.25", format!("{}", Float(12.25)));
    assert_eq!("-3.5", format!("{}", Float(-3.5)));
}

#[test]
fn single_binop_exprs() {
    assert_eq!("-14 * 7", format_exp!(Float(-14.0), Star, Float(7.0)));
    assert_eq!("8 - -10", format_exp!(Float(8.0), Minus, Float(-10.0)));
    assert_eq!("12.25 + 72", format_exp!(Float(12.25), Plus, Float(72.0)));
    assert_eq!("-3 * 42.5", format_exp!(Float(-3.0), Star, Float(42.5)));
}

#[test]
fn multi_binop_exprs_no_grouping() {
    assert_eq!("x + 18.5 - 17", format_exp!(exp!(var!("x"), Plus, Float(18.5)), Minus, Float(17.0)));
    assert_eq!("-10 + num - 17", format_exp!(Float(-10.0), Plus, exp!(var!("num"), Minus, Float(17.0))));
    assert_eq!("-10 * 18.5 + some_num", format_exp!(exp!(Float(-10.0), Star, Float(18.5)), Plus, var!("some_num")));
    assert_eq!("anotherNumber - _x * 17", format_exp!(var!("anotherNumber"), Minus, exp!(var!("_x"), Star, Float(17.0))));

    assert_eq!("_ - 18.5 + x2 * -3.25",
               format_exp!(exp!(var!("_"), Minus, Float(18.5)), Plus, exp!(var!("x2"), Star, Float(-3.25))));


    assert_eq!("NUMBER * 18.5 * 17 + N_4",
               format_exp!(exp!(exp!(var!("NUMBER"), Star, Float(18.5)), Star, Float(17.0)), Plus, var!("N_4")));
}

#[test]
fn multi_binop_exprs_with_grouping() {
    assert_eq!("_7 * (e_4_4 + OkDk)", format_exp!(var!("_7"), Star, exp!(var!("e_4_4"), Plus, var!("OkDk"))));
    assert_eq!("(-10 - 18.5) * 17", format_exp!(exp!(Float(-10.0), Minus, Float(18.5)), Star, Float(17.0)));
    assert_eq!("-10 - (O_k + 17)", format_exp!(Float(-10.0), Minus, exp!(var!("O_k"), Plus, Float(17.0))));

    assert_eq!("(_Ok - h2o) * 17 * -3.25",
               format_exp!(exp!(var!("_Ok"), Minus, var!("h2o")), Star, exp!(Float(17.0), Star, Float(-3.25))));

    assert_eq!("(-10 + 18.5 - 17) * -3.25",
               format_exp!(exp!(exp!(Float(-10.0), Plus, Float(18.5)), Minus, Float(17.0)), Star, Float(-3.25)));

    assert_eq!("(-10 - (18.5 - 17)) * -3.25",
                format_exp!(exp!(Float(-10.0), Minus, exp!(Float(18.5), Minus, Float(17.0))), Star, Float(-3.25)));
}
