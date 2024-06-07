use crate::{Byte, Cycle, instructions, Word};
use crate::memory::Memory;

#[allow(non_snake_case, unused)]
pub struct Flags {
    pub Carry: bool,
    pub Zero: bool,
    pub InterruptDisable: bool,
    pub DecimalMode: bool,
    pub Break: bool,
    pub Overflow: bool,
    pub Negative: bool,
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

    pub fn execute(&mut self, mem: &mut Memory, cycle: &mut Cycle) {
        let opcode = self.fetch_byte(mem);
        match opcode {
            instructions::LDA::IMM => {
                self.handle_LDA_IMM(mem, cycle);
            }
            instructions::LDA::ZP => {
                self.handle_LDA_ZP(mem, cycle);
            }
            instructions::LDA::ZPX => {
                self.handle_LDA_ZPX(mem, cycle)
            }
            instructions::LDA::ABS => {
                self.handle_LDA_ABS(mem, cycle)
            }
            instructions::LDA::ABSX => {
                self.handle_LDA_ABSX(mem, cycle)
            }
            instructions::LDA::ABSY => {
                self.handle_LDA_ABSY(mem, cycle)
            }
            instructions::LDA::INDX => {
                self.handle_LDA_INDX(mem, cycle)
            }
            instructions::LDA::INDY => {
                self.handle_LDA_INDY(mem, cycle)
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

    fn handle_LDA_ABS(&mut self, mem: &mut Memory, cycle: &mut Cycle) {
        let address: Word = self.fetch_word(mem);
        *cycle -= 1;
        let mut value = self.read_byte(mem, address);
        *cycle -= 1;
        self.A = value;
        *cycle -= 1;
        self.set_flags_LDA()
    }

    fn handle_LDA_ABSX(&mut self, mem: &mut Memory, cycle: &mut Cycle) {
        let base_address: Word = self.fetch_word(mem);
        *cycle -= 1;
        let address = base_address + self.X as Word;
        *cycle -= 1;
        if (base_address >> 8) & 0xFF != (address>>8)&0xFF{
            *cycle -= 1
        }
        let value = self.read_byte(mem, address);
        *cycle -= 1;
        self.A = value;
        *cycle -= 1;
        self.set_flags_LDA()
    }

    fn handle_LDA_ABSY(&mut self, mem: &mut Memory, cycle: &mut Cycle) {
        let base_address: Word = self.fetch_word(mem);
        *cycle -= 1;
        let address = base_address + self.Y as Word;
        *cycle -= 1;
        if (base_address >> 8) & 0xFF != (address>>8)&0xFF{
            *cycle -= 1
        }
        let value = self.read_byte(mem, address);
        *cycle -= 1;
        self.A = value;
        *cycle -= 1;
        self.set_flags_LDA()
    }

    fn handle_LDA_INDX(&mut self, mem: &mut Memory, cycle: &mut Cycle) {
        let mut address: Word = (0x0000 | self.fetch_byte(mem)) as Word;
        *cycle -= 1;
        address = address + self.X as Word;
        *cycle -= 1;
        let lo = self.read_byte(mem, address);
        let hi = self.read_byte(mem, address+0x01);
        *cycle -= 1;
        let effective_address = ((hi as u16) << 8) | lo as u16;
        *cycle -= 1;
        let value = self.read_byte(mem, effective_address);
        *cycle -= 1;
        self.A = value;
        *cycle -= 1;
        self.set_flags_LDA()
    }

    fn handle_LDA_INDY(&mut self, mem: &mut Memory, cycle: &mut Cycle) {
        let zp_address: Word = (0x0000 | self.fetch_byte(mem)) as Word;
        *cycle -= 1;
        let lo = self.read_byte(mem, zp_address);
        let hi = self.read_byte(mem, zp_address+0x01);
        *cycle -= 1;
        let base_address = (((hi as u16) << 8) | lo as u16) ;
        let effective_address = base_address + self.Y as u16;
        *cycle -= 1;
        if (base_address >> 8) & 0xFF != (effective_address>>8)&0xFF{
            *cycle -= 1
        }
        let value = self.read_byte(mem, effective_address as Word);
        *cycle -= 1;
        self.A = value;
        *cycle -= 1;
        self.set_flags_LDA()
    }
}