pub mod lex;
pub mod parse;

pub fn main(content: String) -> Vec<parse::Node<'static>> {
    parse::main(lex::main(content))
}