pub mod interpreter;
use interpreter::Interpreter;
use std::env;

pub fn main() {
    let filename: &String = &env::args().collect::<Vec<String>>()[1];

    let interpreter = Interpreter::new();

}
