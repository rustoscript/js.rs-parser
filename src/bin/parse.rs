extern crate jsrs_parser;
extern crate rustyline;

use jsrs_parser::lalr::parse_Stmt;
use rustyline::Editor;
use std::fs::metadata;
use std::io::{self, Write};

fn main() {
    let mut err = io::stderr();
    let mut rl = Editor::new();

    if let Ok(_) = metadata(".history") {
        if let Err(_) = rl.load_history(".history") {
            writeln!(err, "Error: unable to load history on startup").unwrap();
        }
    }

    loop {
        let input = rl.readline(">> ").unwrap();

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
