#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Int(i64),
    Float(f64),
    Str(String),
}
