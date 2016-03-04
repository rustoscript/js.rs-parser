extern crate lalrpop;

// Generate parsing functions from all *.lalrpop files in src/
fn main() {
    lalrpop::process_root().unwrap();
}
