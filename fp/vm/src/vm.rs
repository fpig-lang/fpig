use crate::chunk::Chunk;

type IntResult = Result<(), ()>;

pub struct Vm {
    chunk: Chunk,
    ip: usize,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Vm { chunk, ip: 0 }
    }

    pub fn interpret(&mut self) -> IntResult {
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> IntResult {
        loop {
            let byte = self.read_byte().ok_or(())?;

            match byte {
                0x00 => todo!(),
                0x01 => todo!(),
                _ => return Err(()),
            }
        }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        let byte = self.chunk.get_byte(self.ip);
        self.ip += 1;
        byte
    }
}
