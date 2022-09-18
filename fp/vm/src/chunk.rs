use fp_utils::location::Location;

use crate::object::Objects;

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Objects>,
    locations: Option<Vec<Location>>,
}
