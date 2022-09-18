use crate::chunk::Chunk;

type IntResult = std::result::Result<(), ()>;

pub struct Vm {
    chunk: Option<Chunk>,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Vm { chunk: None }
    }

    pub fn interpret(chunk: Chunk) -> IntResult {
        todo!()
    }
}
