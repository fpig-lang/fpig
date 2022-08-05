use std::str::Chars;

use fp_utils::location::Location;

pub const EOF_CHAR: char = '\0';

pub struct Cursor<'a> {
    chars: Chars<'a>,
    location: Location,
}

// similar to lexer in rustc
impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            chars: input.chars(),
            location: Location::default(),
        }
    }

    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    // bump a char and not checked
    // must be called after checked one of first(), second(), is_eof()
    pub fn bump(&mut self) -> char {
        let c = self.chars.next().unwrap_or(EOF_CHAR);
        if c == '\n' {
            self.location.new_line();
        } else {
            self.location.right();
        }
        c
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    // copied from rustc
    pub fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        // It was tried making optimized version of this for eg. line comments, but
        // LLVM can inline all of this and compile it down to fast iteration over bytes.
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }
}
