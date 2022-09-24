// this file is on dev, only use panic() instead error handler!

use utils::location::Location;

use crate::ast::{ExprKind, ParseObj};

use crate::{
    ast::{Binaryop, Expr, Unaryop},
    cursor::Cursor,
    token::{Token, TokenKind},
};

pub struct Parser<'a> {
    cursor: Cursor<'a>,
    now: Token,
    next: Token,
}

impl Parser<'_> {
    pub fn new(cursor: Cursor) -> Parser {
        Parser {
            cursor,
            now: Token::default(),
            next: Token::default(),
        }
    }

    pub fn parse(&mut self) -> Box<Expr> {
        self.eat();
        self.expression()
    }

    fn expression(&mut self) -> Box<Expr> {
        self.expr_and()
    }

    fn expr_and(&mut self) -> Box<Expr> {
        let l = self.get_location();
        let mut left = self.expr_or();

        while self.check(&[TokenKind::And]) {
            let right = self.expr_or();
            left = Box::new(Expr::new(
                ExprKind::Binary {
                    left,
                    op: Binaryop::And,
                    right,
                },
                l,
            ));
        }

        left
    }

    fn expr_or(&mut self) -> Box<Expr> {
        let l = self.get_location();
        let mut left = self.expr_equal();

        while self.check(&[TokenKind::Or]) {
            let right = self.expr_equal();
            left = Box::new(Expr::new(
                ExprKind::Binary {
                    left,
                    op: Binaryop::Or,
                    right,
                },
                l,
            ));
        }

        left
    }

    fn expr_equal(&mut self) -> Box<Expr> {
        let l = self.get_location();
        let mut left = self.expr_comparison();

        use TokenKind::*;
        while self.check(&[EqEq, BangEq]) {
            let op = match self.now.kind() {
                EqEq => Binaryop::Eq,
                BangEq => Binaryop::NotEq,
                _ => panic!(),
            };
            let right = self.expr_comparison();
            left = Box::new(Expr::new(ExprKind::Binary { left, op, right }, l));
        }

        left
    }

    fn expr_comparison(&mut self) -> Box<Expr> {
        let l = self.get_location();
        let mut left = self.term();

        use TokenKind::*;
        while self.check(&[Gt, GtE, Lt, LtE]) {
            let op = match self.now.kind() {
                Gt => Binaryop::Gt,
                GtE => Binaryop::GtE,
                Lt => Binaryop::Lt,
                LtE => Binaryop::LtE,
                _ => panic!(),
            };
            let right = self.term();
            left = Box::new(Expr::new(ExprKind::Binary { left, op, right }, l));
        }

        left
    }

    fn term(&mut self) -> Box<Expr> {
        let l = self.get_location();
        let mut left = self.factor();

        use TokenKind::*;
        while self.check(&[Plus, Minus]) {
            let op = match self.now.kind() {
                Plus => Binaryop::Add,
                Minus => Binaryop::Sub,
                _ => panic!(),
            };
            let right = self.factor();
            left = Box::new(Expr::new(ExprKind::Binary { left, op, right }, l));
        }

        left
    }

    fn factor(&mut self) -> Box<Expr> {
        let l = self.get_location();
        let mut left = self.unary();

        use TokenKind::*;
        while self.check(&[Star, Slash]) {
            let op = match self.now.kind() {
                Star => Binaryop::Mult,
                Slash => Binaryop::Div,
                _ => panic!(),
            };
            let right = self.unary();
            left = Box::new(Expr::new(ExprKind::Binary { left, op, right }, l));
        }

        left
    }

    fn unary(&mut self) -> Box<Expr> {
        let l = self.get_location();
        use TokenKind::*;
        if self.check(&[Bang, Minus]) {
            let op = match self.now.kind() {
                Bang => Unaryop::Not,
                Minus => Unaryop::Sub,
                _ => panic!(),
            };
            let operand = self.unary();
            return Box::new(Expr::new(ExprKind::Unary { op, operand }, l));
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<Expr> {
        let l = self.get_location();
        use TokenKind::*;

        let expr = match self.peek().kind() {
            True => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Bool(true),
                },
                l,
            )),
            False => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Bool(false),
                },
                l,
            )),
            Nil => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Nil,
                },
                l,
            )),
            Int { value } => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Int(*value),
                },
                l,
            )),
            Float { value } => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Float(*value),
                },
                l,
            )),
            Str { value } => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Str(value.clone()),
                },
                l,
            )),
            Ident { name } => Box::new(Expr::new(
                ExprKind::Literal {
                    value: ParseObj::Ident(name.clone()),
                },
                l,
            )),
            OpenParen => {
                self.eat();
                let expr_inner = self.expression();
                if !self.check(&[OpenParen]) {
                    panic!()
                }
                // already eat
                return Box::new(Expr::new(ExprKind::Group { body: expr_inner }, l));
            }
            _ => panic!(),
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

    fn get_location(&self) -> Location {
        *self.now.location()
    }
}
