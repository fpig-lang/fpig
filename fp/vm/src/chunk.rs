use crate::location::Location;

use crate::value::Value;

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    locations: Vec<Location>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::with_capacity(8),
            constants: Vec::with_capacity(8),
            locations: Vec::new(),
        }
    }

    pub fn get_byte(&self, i: usize) -> Option<u8> {
        self.code.get(i).copied()
    }

    pub fn get_long_bytes(&self, start: usize) -> Option<u16> {
        if start + 2 > self.code.len() {
            return None;
        }

        let high = self.get_byte(start)?;
        let low = self.get_byte(start + 1)?;

        let bytes = [high, low];

        Some(u16::from_be_bytes(bytes))
    }

    pub fn get_constant(&self, i: usize) -> Option<&Value> {
        self.constants.get(i)
    }

    pub fn get_location(&self, i: usize) -> Option<&Location> {
        self.locations.get(i)
    }

    pub fn write_code(&mut self, code: u8) {
        self.code.push(code);
    }

    pub fn write_constant(&mut self, v: Value) -> usize {
        self.constants.push(v);
        self.constants.len() - 1
    }

    pub fn write_location(&mut self, l: Location) {
        self.locations.push(l)
    }
}
