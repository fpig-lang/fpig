use core::fmt;
use std::default;

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    kind: TokenKind,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "token kind: {}", self.kind())
    }
}

impl Token {
    pub(crate) fn new(kind: TokenKind) -> Token {
        Token { kind }
    }

    pub(crate) fn kind(&self) -> &TokenKind {
        &self.kind
    }
}

// must drop!!! just used for placehold.
impl default::Default for Token {
    fn default() -> Self {
        Token {
            kind: TokenKind::Eof,
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenKind {
    // single character
    Plus, Minus, Star, Slash, // + - * /
    Comma, Dot, Semi,         // , . ;
    OpenParen, CloseParen,    // ( )
    OpenBrace, CloseBrace,    // { }

    // one or two character
    Bang, BangEq, // ! !=
    Eq, EqEq,     // = ==
    Gt, GtE,      // > >=
    Lt, LtE,      // < <=

    // ident
    Ident { name: String },

    // literals
    Str { value: String },
    Int { value: i32 },
    Float { value: f64 },
    True,
    False,
    Nil,

    // keywords
    Let,              // let
    If, Else,         // if else
    For, While,       // for while
    And, Or,          // and or
    Fun,              // fn
    Return,           // return

    // other
    Error,
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;

        match self {
            OpenParen => write!(f, "("),
            CloseParen => write!(f, ")"),
            OpenBrace => write!(f, "{{"),
            CloseBrace => write!(f, "}}"),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Star => write!(f, "*"),
            Slash => write!(f, "/"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            Semi => write!(f, ";"),
            Bang => write!(f, "!"),
            BangEq => write!(f, "!="),
            Eq => write!(f, "="),
            EqEq => write!(f, "=="),
            Gt => write!(f, ">"),
            GtE => write!(f, ">="),
            Lt => write!(f, "<"),
            LtE => write!(f, "<="),
            Ident { name } => write!(f, "(ident) {}", name),
            Str { value } => write!(f, "(str) {}", value),
            Int { value } => write!(f, "(int) {}", value),
            Float { value } => write!(f, "(float) {}", value),
            Let => write!(f, "let"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            Nil => write!(f, "nil"),
            If => write!(f, "if"),
            Else => write!(f, "else"),
            For => write!(f, "for"),
            While => write!(f, "while"),
            And => write!(f, "and"),
            Or => write!(f, "or"),
            Fun => write!(f, "fn"),
            Return => write!(f, "return"),
            Error => write!(f, "error"),
            Eof => write!(f, "eof"),
        }
    }
}
