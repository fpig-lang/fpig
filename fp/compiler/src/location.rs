use core::fmt;
use std::default;

// dont use this file at the early dev in fpig,
// it makes thing become really complex.
// just add location support when fpig's compiler is finished.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Location {
    line: usize,
    column: usize,
}

#[allow(unused)]
impl Location {
    pub(crate) fn new(line: usize, column: usize) -> Self {
        Location { line, column }
    }

    pub(crate) fn line(&self) -> usize {
        self.line
    }

    pub(crate) fn column(&self) -> usize {
        self.column
    }

    pub(crate) fn right(&mut self) {
        self.column += 1;
    }

    pub(crate) fn new_line(&mut self) {
        self.line += 1;
    }

    pub(crate) fn reset(&mut self) {
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
