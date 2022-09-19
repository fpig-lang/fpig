use std::collections::HashMap;

pub enum Value {
    Nil,
    Number(FpNumber),
    Str(FpStr),
    List(FpList),
}

pub struct FpNumber {
    value: f64,
}

pub struct FpStr {
    value: String,
}

pub struct FpList {
    value: Vec<Value>,
}
