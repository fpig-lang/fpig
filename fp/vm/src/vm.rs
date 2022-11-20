use std::collections::HashMap;

use crate::{chunk::Chunk, value::Value};

type IntResult = Result<(), ()>;

pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    global: HashMap<u16, Value>,
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::with_capacity(8),
            global: HashMap::new(),
        }
    }

    pub fn set_chunk(&mut self, chunk: Chunk) {
        self.ip = 0;
        self.chunk = chunk;
    }

    pub fn interpret(&mut self, chunk: Chunk) -> IntResult {
        self.set_chunk(chunk);
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
                }
                0x0A => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = a > b;
                    self.stack.push(Value::Bool(result));
                }
                0x0B => {
                    let b = self.get_val()?;
                    let a = self.get_val()?;
                    let result = a < b;
                    self.stack.push(Value::Bool(result));
                }
                0x0C => {
                    #[cfg(feature = "vm_dev")]
                    {
                        println!("{:?}", self.stack);
                        println!("{:?}", self.global);
                    }

                    return Ok(());
                }
                0x0D => {
                    let constant = self.read_byte().ok_or(())?;
                    let value = self.chunk.get_constant(constant as usize).ok_or(())?;
                    self.stack.push(value.clone());
                }
                0x0E => {
                    let constant = self.read_long_byte().ok_or(())?;
                    let value = self.chunk.get_constant(constant as usize).ok_or(())?;
                    self.stack.push(value.clone())
                }
                0x0F => {
                    let a = self.stack.pop().unwrap_or(Value::Nil);

                    #[cfg(feature = "vm_dev")]
                    println!("{:?}", a);
                }
                0x10 => {
                    let i = self.read_byte().ok_or(())? as u16;
                    let value = self.stack.pop().ok_or(())?;
                    self.global.insert(i, value);

                    #[cfg(feature = "vm_dev")]
                    println!("{:?}", self.global)
                }
                0x11 => {
                    todo!()
                }
                0x12 => {
                    let i = self.read_byte().ok_or(())?;
                    let value = self.global.get(&(i as u16)).ok_or(())?;
                    self.stack.push(value.clone());
                }
                0x13 => todo!(),
                _ => return Err(()),
            }
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        let byte = self.chunk.get_byte(self.ip);
        self.ip += 1;
        byte
    }

    fn read_long_byte(&mut self) -> Option<u16> {
        let long_byte = self.chunk.get_long_bytes(self.ip);
        self.ip += 2;
        long_byte
    }

    fn get_val(&mut self) -> Result<Value, ()> {
        self.stack.pop().ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use crate::op::OpCode;

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
        let mut vm = Vm::new();
        vm.set_chunk(chunk);
        vm
    }

    #[test]
    fn test_return() {
        let constants = vec![];
        let codes = vec![OpCode::Return as u8];
        let mut vm = vm_with_chunk(&codes, constants);
        assert_eq!(vm.run(), Ok(()))
    }
}
