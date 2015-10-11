use jsrs_parser::ast::BinOp::*;

#[test]
fn star() {
    assert!(Star.precedence() > Plus.precedence());
    assert!(Star.precedence() > Minus.precedence());
}


#[test]
fn plus() {
    assert!(Plus.precedence() < Star.precedence());
    assert!(Plus.precedence() == Minus.precedence());
}

#[test]
fn minus() {
    assert!(Minus.precedence() < Star.precedence());
    assert!(Minus.precedence() == Plus.precedence());
}
