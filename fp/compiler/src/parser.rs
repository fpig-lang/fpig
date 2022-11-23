use crate::ast::{ExprKind, ParseObj, Stmt, StmtKind};

use crate::{
    ast::{Binaryop, Expr, Unaryop},
    lexer::Cursor,
    token::{Token, TokenKind},
};

pub(crate) struct Parser<'a> {
    cursor: Cursor<'a>,
    now: Token,
    next: Token,
}

impl Parser<'_> {
    // TODO: make location really correct not just start at Stmt or Expr

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
            _ => Box::new(Stmt::new(StmtKind::ExprStmt { expr: self.expression() })),
        }
    }

    fn var_declaration(&mut self) -> Box<Stmt> {
        // TODO: remove clone()
        match self.peek().kind().clone() {
            TokenKind::Ident { name } => {
                self.eat();
                if self.check(&[TokenKind::Eq]) {
                    let expr = self.expression();
                    return Box::new(Stmt::new(StmtKind::VarDec { name: name.to_owned(), value: expr }))
                }

                let expr = Box::new(Expr::new(ExprKind::Literal { value: ParseObj::Nil }));
                Box::new(Stmt::new(StmtKind::VarDec { name: name.to_owned(), value: expr }))
            },
            _ => todo!()
        }
    }

    fn expression(&mut self) -> Box<Expr> {
        self.expr_and()
    }

    fn expr_and(&mut self) -> Box<Expr> {
        let mut left = self.expr_or();

        while self.check(&[TokenKind::And]) {
            let right = self.expr_or();
            left = Box::new(Expr::new(
                ExprKind::Binary {
                    left,
                    op: Binaryop::And,
                    right,
                },
            ));
        }

        left
    }

    fn expr_or(&mut self) -> Box<Expr> {
        let mut left = self.expr_equal();

        while self.check(&[TokenKind::Or]) {
            let right = self.expr_equal();
            left = Box::new(Expr::new(
                ExprKind::Binary {
                    left,
                    op: Binaryop::Or,
                    right,
                },
            ));
        }

        left
    }

    fn expr_equal(&mut self) -> Box<Expr> {
        let mut left = self.expr_comparison();

        use TokenKind::*;
        while self.check(&[EqEq, BangEq]) {
            let op = match self.now.kind() {
                EqEq => Binaryop::Eq,
                BangEq => Binaryop::NotEq,
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
        while self.check(&[Gt, GtE, Lt, LtE]) {
            let op = match self.now.kind() {
                Gt => Binaryop::Gt,
                GtE => Binaryop::GtE,
                Lt => Binaryop::Lt,
                LtE => Binaryop::LtE,
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
        while self.check(&[Plus, Minus]) {
            let op = match self.now.kind() {
                Plus => Binaryop::Add,
                Minus => Binaryop::Sub,
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
        while self.check(&[Star, Slash]) {
            let op = match self.now.kind() {
                Star => Binaryop::Mult,
                Slash => Binaryop::Div,
                _ => todo!(),
            };
            let right = self.unary();
            left = Box::new(Expr::new(ExprKind::Binary { left, op, right }));
        }

        left
    }

    fn unary(&mut self) -> Box<Expr> {
        use TokenKind::*;
        if self.check(&[Bang, Minus]) {
            let op = match self.now.kind() {
                Bang => Unaryop::Not,
                Minus => Unaryop::Minus,
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
            True => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Bool(true),
                },
            )),
            False => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Bool(false),
                },
            )),
            Nil => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Nil,
                },
            )),
            Int { value } => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Int(*value),
                },
            )),
            Float { value } => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Float(*value),
                },
            )),
            Str { value } => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Str(value.clone()),
                },
            )),
            Ident { name } => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Ident(name.clone()),
                },
            )),
            OpenParen => {
                self.eat();
                let expr_inner = self.expression();
                if !self.check(&[OpenParen]) {
                    todo!()
                }
                // already eat
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

    fn check(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.peek().kind() == kind {
                self.eat();
                return true;
            }
        }

        false
    }

    fn peek(&self) -> &Token {
        &self.next
    }
}
