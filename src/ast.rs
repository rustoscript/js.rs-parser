#[derive(Debug)]
pub enum BinOp {
    Star,
    Plus,
    Minus,
}

#[derive(Debug)]
pub enum Exp {
    Int(i64),
    Float(f64),
    BinExp(Box<Exp>, BinOp, Box<Exp>),
}
