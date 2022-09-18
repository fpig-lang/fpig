use std::collections::HashMap;

pub enum Objects {
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
    value: Vec<Objects>,
}
