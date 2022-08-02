use crate::{
    cursor::{Cursor, EOF_CHAR},
    token::{Token, TokenKind},
};

// using array rather than slice may better
const PREDEFINED: &[(&str, TokenKind)] = &[
    ("let", TokenKind::Let),
    ("if", TokenKind::If),
    ("else", TokenKind::Else),
    ("for", TokenKind::For),
    ("while", TokenKind::While),
    ("and", TokenKind::And),
    ("or", TokenKind::Or),
    ("fun", TokenKind::Fun),
    ("return", TokenKind::Return),
    ("true", TokenKind::True),
    ("false", TokenKind::False),
    ("nil", TokenKind::Nil),
];

// copied from https://github.com/rust-lang/rust/compiler/rustc_lexer/src/lib.rs
fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space.
    //
    // Note that this set is stable (ie, it doesn't change with different
    // Unicode versions), so it's ok to just hard-code the values.

    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

// identifier start. same as rustc
fn is_ident_start(c: char) -> bool {
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

// identifier continue. same as rustc
fn is_ident_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

// wrap the cursor a iterator. same as rustc
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        // no None, just use TokenKind::Eof
        if cursor.is_eof() {
            return None;
        }
        Some(cursor.advance_token())
    })
}

impl Cursor<'_> {
    // not check the EOF. checking EOF will make this function return Option<Token>
    // or add and EOF in TokenKind. just check EOF before call this function.
    fn advance_token(&mut self) -> Token {
        // space in this language have no meaning, just skip it.
        self.skip_space();

        // this location is used in Token
        let start_location = self.location();

        let start_char = self.bump();
        let token_kind = match start_char {
            // one symbol tokens
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            ';' => TokenKind::Semi,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,

            // one or two symbol tokens
            '!' if self.first() == '\n' => TokenKind::BangEq,
            '!' => TokenKind::Bang,
            '=' if self.first() == '\n' => TokenKind::EqEq,
            '=' => TokenKind::Eq,
            '>' if self.first() == '\n' => TokenKind::GtE,
            '>' => TokenKind::Gt,
            '<' if self.first() == '\n' => TokenKind::LtE,
            '<' => TokenKind::Lt,

            // identifier or prefined (e.g. let, if, else, for...)
            c if is_ident_start(c) => self.ident_or_predefined(c),

            // string
            '"' => self.string(),

            // number
            c @ '0'..='9' => self.number(c),

            _ => TokenKind::Error,
        };
        Token::new(token_kind, start_location)
    }

    fn number(&mut self, start: char) -> TokenKind {
        let mut lexeme = String::with_capacity(4);
        lexeme.push(start);
        while matches!(self.first(), '0'..='9') {
            lexeme.push(self.bump());
        }

        if self.first() == '.' && matches!(self.second(), '0'..='9') {
            lexeme.push(self.bump());

            while matches!(self.first(), '0'..='9') {
                lexeme.push(self.bump());
            }

            let value = lexeme.parse::<f64>().unwrap();
            return TokenKind::Float { value };
        }

        let value = lexeme.parse::<i32>().unwrap();
        TokenKind::Int { value }
    }

    fn string(&mut self) -> TokenKind {
        let mut lexeme = String::with_capacity(8);
        while !matches!(self.first(), EOF_CHAR | '"') {
            lexeme.push(self.bump());
        }

        if self.first() == EOF_CHAR {
            return TokenKind::Error;
        }

        self.bump();
        TokenKind::Str { value: lexeme }
    }

    fn ident_or_predefined(&mut self, start: char) -> TokenKind {
        let mut lexeme = String::with_capacity(4);
        lexeme.push(start);

        while is_ident_continue(self.first()) {
            lexeme.push(self.bump());
        }

        if let Some((_, kind)) = PREDEFINED.iter().find(|&&(s, _)| s == lexeme) {
            return kind.clone();
        }

        TokenKind::Ident { name: lexeme }
    }

    fn skip_space(&mut self) {
        self.eat_while(is_whitespace);
    }
}
