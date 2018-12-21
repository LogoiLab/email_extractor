extern crate tempdir;
extern crate zip;

pub mod parser;
pub mod extractor;

use self::parser::parse;
use self::extractor::extract;

fn main() {
    println!("Hello, world!");
}
