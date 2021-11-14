pub mod ast;
pub mod interpreter;

use std::env;

pub fn main() {
    let filename: &String = &env::args().collect::<Vec<String>>()[1];
    let tree = ast::main(filename);
    interpreter::main(tree);
}
