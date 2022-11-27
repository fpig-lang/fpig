use crate::ast::{ExprKind, ParseObj, Stmt, StmtKind};

use crate::{
    ast::{BinaryOp, Expr, UnaryOp},
    lexer::Cursor,
    token::{Token, TokenKind},
};

pub(crate) struct Parser<'a> {
    cursor: Cursor<'a>,
    now: Token,
    next: Token,
}

impl Parser<'_> {
    pub(crate) fn new(cursor: Cursor) -> Parser {
        Parser {
            cursor,
            now: Token::default(),
            next: Token::default(),
        }
    }

    pub(crate) fn parse(&mut self) -> Box<Stmt> {
        self.eat();
        self.declaration()
    }

    fn declaration(&mut self) -> Box<Stmt> {
        match self.peek().kind() {
            TokenKind::Let => {
                self.eat(); // eat the Token Let
                self.var_declaration()
            }
            _ => self.statement(),
        }
    }

    fn statement(&mut self) -> Box<Stmt> {
        match self.peek().kind() {
            _ => Box::new(Stmt::new(StmtKind::ExprStmt {
                expr: self.expression(),
            })),
        }
    }

    fn var_declaration(&mut self) -> Box<Stmt> {
        // TODO: remove clone()
        match self.peek().kind().clone() {
            TokenKind::Ident { name } => {
                self.eat();
                if self.check_eat(&[TokenKind::Eq]) {
                    let expr = self.expression();
                    return Box::new(Stmt::new(StmtKind::VarDec { name, value: expr }));
                }

                let expr = Box::new(Expr::new(ExprKind::Literal {
                    value: ParseObj::Nil,
                }));
                Box::new(Stmt::new(StmtKind::VarDec { name, value: expr }))
            }
            _ => todo!(),
        }
    }

    fn expression(&mut self) -> Box<Expr> {
        match self.peek().kind().clone() {
            TokenKind::OpenBrace => {
                self.eat();
                let mut inner = Vec::new();
                while !self.check(&[TokenKind::CloseBrace, TokenKind::Eof]) {
                    inner.push(*self.declaration());
                }

                if !self.check_eat(&[TokenKind::CloseBrace]) {
                    todo!()
                }

                // already eated }

                Box::new(Expr::new(ExprKind::Block { inner }))
            }
            _ => self.expr_and(),
        }
    }

    fn expr_and(&mut self) -> Box<Expr> {
        let mut left = self.expr_or();

        while self.check_eat(&[TokenKind::And]) {
            let right = self.expr_or();
            left = Box::new(Expr::new(ExprKind::Binary {
                left,
                op: BinaryOp::And,
                right,
            }));
        }

        left
    }

    fn expr_or(&mut self) -> Box<Expr> {
        let mut left = self.expr_equal();

        while self.check_eat(&[TokenKind::Or]) {
            let right = self.expr_equal();
            left = Box::new(Expr::new(ExprKind::Binary {
                left,
                op: BinaryOp::Or,
                right,
            }));
        }

        left
    }

    fn expr_equal(&mut self) -> Box<Expr> {
        let mut left = self.expr_comparison();

        use TokenKind::*;
        while self.check_eat(&[EqEq, BangEq]) {
            let op = match self.now.kind() {
                EqEq => BinaryOp::Eq,
                BangEq => BinaryOp::NotEq,
                _ => todo!(),
            };
            let right = self.expr_comparison();
            left = Box::new(Expr::new(ExprKind::Binary { left, op, right }));
        }

        left
    }

    fn expr_comparison(&mut self) -> Box<Expr> {
        let mut left = self.term();

        use TokenKind::*;
        while self.check_eat(&[Gt, GtE, Lt, LtE]) {
            let op = match self.now.kind() {
                Gt => BinaryOp::Gt,
                GtE => BinaryOp::GtE,
                Lt => BinaryOp::Lt,
                LtE => BinaryOp::LtE,
                _ => todo!(),
            };
            let right = self.term();
            left = Box::new(Expr::new(ExprKind::Binary { left, op, right }));
        }

        left
    }

    fn term(&mut self) -> Box<Expr> {
        let mut left = self.factor();

        use TokenKind::*;
        while self.check_eat(&[Plus, Minus]) {
            let op = match self.now.kind() {
                Plus => BinaryOp::Add,
                Minus => BinaryOp::Sub,
                _ => todo!(),
            };
            let right = self.factor();
            left = Box::new(Expr::new(ExprKind::Binary { left, op, right }));
        }

        left
    }

    fn factor(&mut self) -> Box<Expr> {
        let mut left = self.unary();

        use TokenKind::*;
        while self.check_eat(&[Star, Slash]) {
            let op = match self.now.kind() {
                Star => BinaryOp::Mult,
                Slash => BinaryOp::Div,
                _ => todo!(),
            };
            let right = self.unary();
            left = Box::new(Expr::new(ExprKind::Binary { left, op, right }));
        }

        left
    }

    fn unary(&mut self) -> Box<Expr> {
        use TokenKind::*;
        if self.check_eat(&[Bang, Minus]) {
            let op = match self.now.kind() {
                Bang => UnaryOp::Not,
                Minus => UnaryOp::Minus,
                _ => todo!(),
            };
            let operand = self.unary();
            return Box::new(Expr::new(ExprKind::Unary { op, operand }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<Expr> {
        use TokenKind::*;

        let expr = match self.peek().kind() {
            True => Box::new(Expr::new(ExprKind::Literal {
                value: ParseObj::Bool(true),
            })),
            False => Box::new(Expr::new(ExprKind::Literal {
                value: ParseObj::Bool(false),
            })),
            Nil => Box::new(Expr::new(ExprKind::Literal {
                value: ParseObj::Nil,
            })),
            Int { value } => Box::new(Expr::new(ExprKind::Literal {
                value: ParseObj::Int(*value),
            })),
            Float { value } => Box::new(Expr::new(ExprKind::Literal {
                value: ParseObj::Float(*value),
            })),
            Str { value } => Box::new(Expr::new(ExprKind::Literal {
                value: ParseObj::Str(value.clone()),
            })),
            Ident { name } => Box::new(Expr::new(ExprKind::Literal {
                value: ParseObj::Ident(name.clone()),
            })),
            OpenParen => {
                self.eat();
                let expr_inner = self.expression();
                if !self.check_eat(&[CloseParen]) {
                    todo!()
                }
                // already eated )
                return Box::new(Expr::new(ExprKind::Group { body: expr_inner }));
            }
            _ => todo!(),
        };
        self.eat();

        expr
    }

    fn eat(&mut self) {
        let next = self.cursor.advance_token();
        self.now = std::mem::replace(&mut self.next, next);
    }

    fn check_eat(&mut self, kinds: &[TokenKind]) -> bool {
        if self.check(kinds) {
            self.eat();
            return true;
        }
        false
    }

    fn check(&self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.peek().kind() == kind {
                return true;
            }
        }

        false
    }

    fn peek(&self) -> &Token {
        &self.next
    }
}
