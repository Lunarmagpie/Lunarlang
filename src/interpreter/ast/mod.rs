use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub mod lex;
pub mod parse;

pub fn main(content: String) -> Vec<parse::Node> {

    parse::main(lex::main(content))

}