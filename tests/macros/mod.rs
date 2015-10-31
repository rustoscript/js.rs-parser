macro_rules! exp {
    ($e1:expr, $o:expr, $e2:expr) => { BinExp(Box::new($e1), $o, Box::new($e2)) }
}

macro_rules! post_dec {
    ($e:expr) => { PostDec(Box::new($e)) }
}

macro_rules! post_inc {
    ($e:expr) => { PostInc(Box::new($e)) }
}

macro_rules! pre_dec {
    ($e:expr) => { PreDec(Box::new($e)) }
}

macro_rules! pre_inc {
    ($e:expr) => { PreInc(Box::new($e)) }
}

macro_rules! assign {
    ($v:expr, $e:expr) => { Assign(String::from($v), $e) }
}

macro_rules! decl {
    ($v:expr, $e:expr) => { Decl(String::from($v), $e) }
}

macro_rules! var {
    ($s:expr) => { Var(String::from($s)) }
}

macro_rules! parse_exp {
    ($s:expr) => { parse_Exp($s).unwrap() }
}

macro_rules! parse_stmt {
    ($s:expr) => { parse_Stmt($s).unwrap() }
}
