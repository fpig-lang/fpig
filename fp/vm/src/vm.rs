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

    #[allow(unused)]
    fn run(&mut self) -> IntResult {
        loop {
            let byte = self.read_byte().ok_or(())?;

            match byte {
                0x00 => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = (a + b)?;
                    self.stack.push(result);
                }
                0x01 => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = (a - b)?;
                    self.stack.push(result);
                }
                0x02 => {
                    let value = self.get_val()?;
                    // self.stack.push(-value);
                }
                0x03 => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = (a * b)?;
                    self.stack.push(result);
                }
                0x04 => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = (a / b)?;
                    self.stack.push(result);
                }
                0x05 => self.stack.push(Value::Bool(true)),
                0x06 => self.stack.push(Value::Bool(false)),
                0x07 => self.stack.push(Value::Nil),
                0x08 => todo!(),
                0x09 => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = a == b;
                    self.stack.push(Value::Bool(result));
                },
                0x0A => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = a > b;
                    self.stack.push(Value::Bool(result));
                },
                0x0B => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = a < b;
                    self.stack.push(Value::Bool(result));
                },
                0x0C => {
                    return Ok(());
                }
                0x0D => {
                    let constant = self.read_byte().ok_or(())?;
                    let value = self.chunk.get_constant(constant as usize).ok_or(())?;
                    self.stack.push(value.clone());
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

#[cfg(test)]
mod tests {
    use utils::op::OpCode;

    use crate::{chunk::Chunk, value::Value};

    use super::Vm;

    fn vm_with_chunk(codes: &[u8], constants: Vec<Value>) -> Vm {
        let mut chunk = Chunk::new();
        for v in constants {
            chunk.write_constant(v);
        }

        for code in codes {
            chunk.write_code(*code as u8);
        }
        Vm::new(chunk)
    }

    #[test]
    fn test_return() {
        let constants = vec![];
        let codes = vec![OpCode::Return as u8];
        let mut vm = vm_with_chunk(&codes, constants);
        assert_eq!(vm.run(), Ok(()))
    }
}
