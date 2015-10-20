macro_rules! exp {
    ($e1:expr, $o:expr, $e2:expr) => { BinExp(Box::new($e1), $o, Box::new($e2)) }
}

macro_rules! parse_exp {
    ($s:expr) => { parse_Exp($s).unwrap() }
}
