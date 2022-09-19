use core::f64;

#[derive(Debug)]
pub enum ParseObj {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f64),
    Str(String),
    Ident(String),
}

#[derive(Debug)]
pub enum Expr {
    Literal {
        value: ParseObj,
    },
    Group {
        body: Box<Expr>,
    },
    Unary {
        op: Unaryop,
        operand: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Binaryop,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Unaryop {
    Sub,
    Not,
}

#[derive(Debug)]
pub enum Binaryop {
    Add,
    Sub,
    Mult,
    Div,
    Eq,
    NotEq,
    Gt,
    GtE,
    Lt,
    LtE,
    And,
    Or,
}
