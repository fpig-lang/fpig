use core::fmt;
use std::default;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    line: usize,
    column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Location { line, column }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn right(&mut self) {
        self.column += 1;
    }

    pub fn new_line(&mut self) {
        self.line += 1;
    }

    pub fn reset(&mut self) {
        self.line = 1;
        self.column = 1;
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:4}, {:4}]", self.line, self.column)
    }
}

impl default::Default for Location {
    fn default() -> Self {
        Location { line: 1, column: 1 }
    }
}
