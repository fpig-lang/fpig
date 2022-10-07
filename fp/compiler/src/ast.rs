use core::f64;

use crate::location::Location;

#[derive(Debug)]
pub(crate) enum ParseObj {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f64),
    Str(String),
    Ident(String),
}

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct Located<T> {
    pub node: T,
    pub location: Location,
}

impl<T> Located<T> {
    pub(crate) fn new(node: T, location: Location) -> Self {
        Located { node, location }
    }
}

pub(crate) type Stmt = Located<StmtKind>;

pub(crate) enum StmtKind {
    ExprStmt {
        expr: Box<Expr>,
    },
    VarDec {
        name: String,
        value: Box<Expr>,
    },
}

pub(crate) type Expr = Located<ExprKind>;

#[derive(Debug)]
pub(crate) enum ExprKind {
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
pub(crate) enum Unaryop {
    Not,
    Minus,
}

#[derive(Debug)]
pub(crate) enum Binaryop {
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
