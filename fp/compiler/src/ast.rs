use core::f64;

#[derive(Debug)]
pub(crate) enum ParseObj {
    Nil,
    Bool(bool),
    Int(i32),
    Float(f64),
    Str(String),
    Ident(String), // TODO: use a u16 for idx instead use String directly
}

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct Located<T> {
    pub node: T,
}

impl<T> Located<T> {
    pub(crate) fn new(node: T) -> Self {
        Located { node }
    }
}

pub(crate) type Stmt = Located<StmtKind>;

#[derive(Debug)]
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
