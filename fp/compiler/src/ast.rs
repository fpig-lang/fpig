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
pub(crate) type Expr = Located<ExprKind>;

#[derive(Debug)]
pub(crate) enum StmtKind {
    ExprStmt { expr: Box<Expr> },
    VarDec { name: String, value: Box<Expr> },
}

#[derive(Debug)]
pub(crate) enum ExprKind {
    Literal {
        value: ParseObj,
    },
    Group {
        body: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Block {
        inner: Vec<Stmt>,
    },
}

#[derive(Debug)]
pub(crate) enum UnaryOp {
    Not,
    Minus,
}

#[derive(Debug)]
pub(crate) enum BinaryOp {
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
