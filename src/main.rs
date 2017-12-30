extern crate diff_json;

use diff_json::formatter;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: foo bar.")
    }
}
