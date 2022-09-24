use crate::{chunk::Chunk, value::Value};

type IntResult = Result<(), ()>;

pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Vm {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(8),
        }
    }

    pub fn interpret(&mut self) -> IntResult {
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> IntResult {
        loop {
            let byte = self.read_byte().ok_or(())?;

            self.ip += 1;
            match byte {
                0x00 => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = a + b;
                    self.stack.push(result);
                }
                0x01 => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = a - b;
                    self.stack.push(result);
                }
                0x02 => {
                    let value = self.get_val()?;
                    self.stack.push(-value);
                }
                0x03 => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = a * b;
                    self.stack.push(result);
                }
                0x04 => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = a / b;
                    self.stack.push(result);
                }
                0x05 => todo!(),
                0x06 => todo!(),
                0x07 => todo!(),
                0x08 => todo!(),
                0x09 => todo!(),
                0x0A => todo!(),
                0x0B => todo!(),
                0x0C => {
                    return Ok(());
                }
                0x0D => {
                    let constant = self.read_byte().ok_or(())?;
                    let value = self.chunk.get_constant(constant as usize).ok_or(())?;
                    self.stack.push(value);
                }
                _ => return Err(()),
            }
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        let byte = self.chunk.get_byte(self.ip);
        self.ip += 1;
        byte
    }

    fn get_val(&mut self) -> Result<Value, ()> {
        self.stack.pop().ok_or(())
    }
}
