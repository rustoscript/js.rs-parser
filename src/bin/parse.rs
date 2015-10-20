extern crate jsrs_parser;

use jsrs_parser::arith::parse_Stmt;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("{:?}", parse_Stmt(&input).unwrap());
}
