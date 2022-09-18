// this file is on dev, only use panic() instead error handler!

use fp_utils::objects::FpObjects;

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
        let mut left = self.expr_or();

        while self.check(&[TokenKind::And]) {
            let right = self.expr_or();
            left = Box::new(Expr::Binary {
                left,
                op: Binaryop::And,
                right,
            })
        }

        left
    }

    fn expr_or(&mut self) -> Box<Expr> {
        let mut left = self.expr_equal();

        while self.check(&[TokenKind::Or]) {
            let right = self.expr_equal();
            left = Box::new(Expr::Binary {
                left,
                op: Binaryop::Or,
                right,
            });
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
                _ => panic!(),
            };
            let right = self.expr_comparison();
            left = Box::new(Expr::Binary { left, op, right });
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
                _ => panic!(),
            };
            let right = self.term();
            left = Box::new(Expr::Binary { left, op, right });
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
                _ => panic!(),
            };
            let right = self.factor();
            left = Box::new(Expr::Binary { left, op, right });
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
                _ => panic!(),
            };
            let right = self.unary();
            left = Box::new(Expr::Binary { left, op, right });
        }

        left
    }

    fn unary(&mut self) -> Box<Expr> {
        use TokenKind::*;
        if self.check(&[Bang, Minus]) {
            let op = match self.now.kind() {
                Bang => Unaryop::Not,
                Minus => Unaryop::Sub,
                _ => panic!(),
            };
            let operand = self.unary();
            return Box::new(Expr::Unary { op, operand });
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<Expr> {
        use TokenKind::*;

        let expr = match self.peek().kind() {
            True => Box::new(Expr::Literal {
                value: FpObjects::Bool(true),
            }),
            False => Box::new(Expr::Literal {
                value: FpObjects::Bool(false),
            }),
            Nil => Box::new(Expr::Literal {
                value: FpObjects::Nil,
            }),
            Int { value } => Box::new(Expr::Literal {
                value: FpObjects::Int(*value),
            }),
            Float { value } => Box::new(Expr::Literal {
                value: FpObjects::Float(*value),
            }),
            Str { value } => Box::new(Expr::Literal {
                value: FpObjects::Str(value.clone()),
            }),
            Ident { name } => Box::new(Expr::Literal {
                value: FpObjects::Ident(name.clone()),
            }),
            OpenParen => {
                self.eat();
                let expr_inner = self.expression();
                if !self.check(&[OpenParen]) {
                    panic!()
                }
                // already eat
                return expr_inner;
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
}
