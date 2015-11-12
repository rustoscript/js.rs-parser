extern crate jsrs_parser;

use jsrs_parser::lalr::parse_Stmt;
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    loop {
        print!(">> ");
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        if &input == ".exit\n" {
            return;
        }

        let stmt = match parse_Stmt(&input) {
            Ok(s) => s,
            _ => {
                writeln!(stderr, "Error: failed to parse").unwrap();
                continue
            }
        };

        println!("=> {:?}", stmt);
    }
}
