extern crate jsrs_parser;
extern crate rustyline;

use jsrs_parser::lalr::parse_Stmt;
use rustyline::Editor;
use rustyline::error::ReadlineError::Eof;
use std::fs::metadata;
use std::io::{self, Write};

fn main() {
    let mut err = io::stderr();
    let mut rl = Editor::new();

    if metadata(".history").is_ok() && rl.load_history(".history").is_err() {
        writeln!(err, "Error: unable to load history on startup").unwrap();
    }

    loop {
        let input_result = rl.readline(">> ");

        let input = match input_result {
            Ok(t) => t,
            Err(Eof) => String::from(".exit"),
            Err(e) => panic!("Error: {:?}", e),
        };

        if input.eq(".exit") {
            if let Err(_) = rl.save_history(".history") {
                writeln!(err, "Error: unable to save history on exit").unwrap();
            }

            break;
        }

        rl.add_history_entry(&input);
        let stmt = match parse_Stmt(&input) {
            Ok(s) => s,
            _ => {
                writeln!(err, "Error: unable to parse statement").unwrap();
                continue;
            }
        };

        println!("{:?}", stmt);
    }
}
