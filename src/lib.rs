mod instructions;
mod cpu;
mod memory;
#[cfg(test)]
mod test;



// http://www.6502.org/users/obelisk/6502/index.html
type Byte = u8;
type Word = u16;
// type DoubleWord = u32;
type Cycle = u8;

