use fp_utils::objects::FpObjects;

#[derive(Debug)]
pub enum Expr {
    Literal {
        value: FpObjects,
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
