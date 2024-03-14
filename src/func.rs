pub struct Func {}

pub enum Args {
    Int32(i32),
    Bool(bool),
    Str(String),
    Arr(Vec<Args>),
}
