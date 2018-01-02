extern crate diff_json;

use diff_json::diff::Diff;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    Diff::new("foo".to_string(), "bar".to_string()).exec();
    if args.len() != 2 {
        panic!("Usage: foo bar.")
    }
}
