use core::f64;

#[derive(Debug)]
pub enum FpObjects {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f64),
    Str(String),
    Ident(String),
}
