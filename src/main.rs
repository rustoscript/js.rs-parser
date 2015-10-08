mod arith;
mod ast;

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("{:?}", arith::parse_Exp(&input).unwrap());
}
