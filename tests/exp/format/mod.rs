use jsrs_parser::ast::BinOp::*;
use jsrs_parser::ast::Exp::*;

macro_rules! format_exp {
    ($e1:expr, $o:expr, $e2:expr) => { &format!("{}", exp!($e1, $o, $e2)) }
}

#[test]
fn constants() {
    assert_eq!("-14", format!("{}", Int(-14)));
    assert_eq!("8", format!("{}", Int(8)));

    // Floating point values specifically chosen so fractional part can be perfectly represented
    // by floating-point numbers.
    assert_eq!("12.25", format!("{}", Float(12.25)));
    assert_eq!("-3.5", format!("{}", Float(-3.5)));
}

#[test]
fn single_binop_exprs() {
    assert_eq!("-14 * 7", format_exp!(Int(-14), Star, Int(7)));
    assert_eq!("8 - -10", format_exp!(Int(8), Minus, Int(-10)));
    assert_eq!("12.25 + 72", format_exp!(Float(12.25), Plus, Int(72)));
    assert_eq!("-3 * 42.5", format_exp!(Int(-3), Star, Float(42.5)));
}

#[test]
fn multi_binop_exprs_no_grouping() {
    assert_eq!("-10 + 18.5 - 17", format_exp!(exp!(Int(-10), Plus, Float(18.5)), Minus, Int(17)));
    assert_eq!("-10 + 18.5 - 17", format_exp!(Int(-10), Plus, exp!(Float(18.5), Minus, Int(17))));
    assert_eq!("-10 * 18.5 + 17", format_exp!(exp!(Int(-10), Star, Float(18.5)), Plus, Int(17)));
    assert_eq!("-10 - 18.5 * 17", format_exp!(Int(-10), Minus, exp!(Float(18.5), Star, Int(17))));

    assert_eq!("-10 - 18.5 + 17 * -3.25",
               format_exp!(exp!(Int(-10), Minus, Float(18.5)), Plus, exp!(Int(17), Star, Float(-3.25))));


    assert_eq!("-10 * 18.5 * 17 + -3.25",
               format_exp!(exp!(exp!(Int(-10), Star, Float(18.5)), Star, Int(17)), Plus, Float(-3.25)));
}

#[test]
fn multi_binop_exprs_with_grouping() {
    assert_eq!("-10 * (18.5 + 17)", format_exp!(Int(-10), Star, exp!(Float(18.5), Plus, Int(17))));
    assert_eq!("(-10 - 18.5) * 17", format_exp!(exp!(Int(-10), Minus, Float(18.5)), Star, Int(17)));
    assert_eq!("-10 - (18.5 + 17)", format_exp!(Int(-10), Minus, exp!(Float(18.5), Plus, Int(17))));

    assert_eq!("(-10 - 18.5) * 17 * -3.25",
               format_exp!(exp!(Int(-10), Minus, Float(18.5)), Star, exp!(Int(17), Star, Float(-3.25))));

    assert_eq!("(-10 + 18.5 - 17) * -3.25",
               format_exp!(exp!(exp!(Int(-10), Plus, Float(18.5)), Minus, Int(17)), Star, Float(-3.25)));

    assert_eq!("(-10 - (18.5 - 17)) * -3.25",
                format_exp!(exp!(Int(-10), Minus, exp!(Float(18.5), Minus, Int(17))), Star, Float(-3.25)));
}
