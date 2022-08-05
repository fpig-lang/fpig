use core::fmt;

use fp_utils::location::Location;

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    location: Location,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "location: {}, token kind: {}", self.location, self.kind)
    }
}

impl Token {
    pub fn new(kind: TokenKind, location: Location) -> Token {
        Token { kind, location }
    }

    #[cfg(test)]
    pub fn reset_location(&mut self) {
        self.location.reset();
    }
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
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
        }
    }
}
