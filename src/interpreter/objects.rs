use std::fmt;

pub type MoonArgs<'a> = Vec<&'a MoonObject<'a>>;

pub struct MoonResult<'a> {
    pub res: Option<MoonObject<'a>>,
    pub exit_code: i64,
}

pub enum MoonObject<'a> {
    Function(&'a dyn Fn(MoonArgs) -> MoonResult),
    String(String),
}

impl fmt::Debug for MoonObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoonObject::Function(_) => write!(f, "Unknown Function"),
            MoonObject::String(s) => write!(f, "{:?}", s),


        }
    }
}

