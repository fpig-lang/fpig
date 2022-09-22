use fp_utils::location::Location;

use crate::value::Value;

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    locations: Option<Vec<Location>>,
}

impl Chunk {
    pub fn get_byte(&self, i: usize) -> Option<u8> {
        if i >= self.code.len() {
            return None;
        }
        Some(self.code[i])
    }

    pub fn get_bytes(&self, start: usize, end: usize) -> Option<&[u8]> {
        if end >= self.code.len() || start > end {
            return None;
        }

        Some(&self.code[start..end])
    }

    pub fn get_constant(&self, i: usize) -> Option<Value> {
        if i >= self.constants.len() {
            return None;
        }
        Some(self.constants[i])
    }

    pub fn get_location(&self, i: usize) -> Option<&Location> {
        if let Some(l) = &self.locations {
            if i >= l.len() {
                return None;
            }
            return Some(&l[i]);
        }

        None
    }

    pub fn write_code(&mut self, code: u8) {
        self.code.push(code);
    }

    pub fn write_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    pub fn write_location(&mut self, location: Location) {
        if let Some(l) = &mut self.locations {
            l.push(location);
        }
    }
}
