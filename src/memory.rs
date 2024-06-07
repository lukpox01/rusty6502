use crate::Byte;

pub struct Memory {
    pub data: [Byte; 64*1024],
}

#[allow(unused)]
impl Memory {
    pub fn new() -> Memory {
        Memory{data: [0; 64*1024]}
    }
}