// #[derive(Debug)]

pub type MoonArgs<'a> = Vec<&'a MoonObject<'a>>;

pub struct MoonResult<'a> {
    pub res: Option<MoonObject<'a>>,
    pub exit_code: i64,
}

pub enum MoonObject<'a> {
    Function(&'a dyn Fn(MoonArgs) -> MoonResult),
    String(String),
}
