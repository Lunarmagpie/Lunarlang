pub mod lex;
pub mod parse;

pub fn main(content: String) -> Vec<parse::Node> {
    parse::main(lex::main(content))
}