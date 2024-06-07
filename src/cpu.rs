use crate::{Byte, Cycle, instructions, Word};
use crate::memory::Memory;

#[allow(non_snake_case, unused)]
pub struct Flags {
    Carry: bool,
    Zero: bool,
    InterruptDisable: bool,
    DecimalMode: bool,
    Break: bool,
    Overflow: bool,
    Negative: bool,
}

#[allow(non_snake_case)]
pub struct CPU {
    pub PC: Word, // Program Counter
    pub SP: Byte, // Stack Pointer

    // Registers
    pub A: Byte, // Accumulator
    pub X: Byte,
    pub Y: Byte,

    // Status Register
    pub Status: Flags,

}


#[allow(non_snake_case, unused)]
impl CPU {
    pub fn new() -> CPU {
        CPU {
            PC: 0,
            SP: 0,
            A: 0,
            X: 0,
            Y: 0,
            Status: Flags {
                Carry: false,
                Zero: false,
                InterruptDisable: false,
                DecimalMode: false,
                Break: false,
                Overflow: false,
                Negative: false,
            }
        }
    }

    pub fn reset(&mut self) {
        self.PC = 0xFFFC;
        self.SP = 0xFF;
        self.Status = Flags {
            Carry: false,
            Zero: false,
            InterruptDisable: false,
            DecimalMode: false,
            Break: false,
            Overflow: false,
            Negative: false,
        };
    }

    fn fetch_byte(&mut self, mem: &Memory) -> Byte {
        let data = mem.data[self.PC as usize];
        self.PC += 1;
        data
    }

    fn read_byte(&mut self, mem: &Memory, address: Word) -> Byte {
        let data = mem.data[address as usize];
        data
    }

    fn fetch_word(&mut self, mem: &Memory) -> Word {
        // little endian
        let lo = mem.data[self.PC as usize];
        let hi = mem.data[(self.PC + 1) as usize];
        let data = ((hi as u16) << 8) | lo as u16;
        self.PC += 2;
        data
    }

    pub fn execute(&mut self, mem: &mut Memory) {
        let opcode = self.fetch_byte(mem);
        match opcode {
            instructions::LDA::IMM => {
                self.handle_LDA_IMM(mem, &mut 2);
            }
            instructions::LDA::ZP => {
                self.handle_LDA_ZP(mem, &mut 3);
            }
            instructions::LDA::ZPX => {
                self.handle_LDA_ZPX(mem, &mut 4)
            }
            _ => panic!("Unknown opcode: {:X}", opcode),
        }
    }

    fn set_flags_LDA(&mut self) {
        self.Status.Zero = self.A == 0;
        self.Status.Negative = (self.A & 0b1000_0000) > 0;
    }

    fn handle_LDA_IMM(&mut self, mem: &mut Memory, cycle: &mut Cycle) {
        let value = self.fetch_byte(mem);
        *cycle -= 1;
        self.A = value;
        *cycle -= 1;
        self.set_flags_LDA()
    }

    fn handle_LDA_ZP(&mut self, mem: &mut Memory, cycle: &mut Cycle) {
        let address: Word = (0x0000 | self.fetch_byte(mem)) as Word;
        *cycle -= 1;
        let value = self.read_byte(mem, address);
        *cycle -= 1;
        self.A = value;
        *cycle -= 1;
        self.set_flags_LDA()
    }

    fn handle_LDA_ZPX(&mut self, mem: &mut Memory, cycle: &mut Cycle) {
        let address: Word = (0x0000 | self.fetch_byte(mem)) as Word;
        *cycle -= 1;
        let mut value = self.read_byte(mem, address);
        *cycle -= 1;
        value += self.X;
        *cycle -= 1;
        self.A = value;
        *cycle -= 1;
        self.set_flags_LDA()
    }
}