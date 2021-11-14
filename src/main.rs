mod ast;

use std::env;

pub fn main() {
    let filename: &String = &env::args().collect::<Vec<String>>()[1];
    ast::main(filename);
}
