use super::{MoonArgs, MoonObject, MoonResult};

pub fn echo(args: MoonArgs) -> MoonResult {
    for arg in args {
        if let MoonObject::String(value) = arg {
            print!("{} ", value);
        }
    }
    println!();

    MoonResult {
        res: None,
        exit_code: 0,
    }
}
