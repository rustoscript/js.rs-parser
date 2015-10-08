extern crate lalrpop;

fn main() {
    lalrpop::process_root().ok().expect("Unable to process .lalrpop files");
}
