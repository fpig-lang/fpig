use core::f64;

use utils::location::Location;

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
pub struct Located<T> {
    node: T,
    location: Location,
}

impl<T> Located<T> {
    pub fn new(node: T, location: Location) -> Self {
        Located { node, location }
    }
}

#[derive(Debug)]
pub enum ExprKind {
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

pub type Expr = Located<ExprKind>;

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
