use crate::memory::Memory;
use crate::{instructions, Byte, Word};

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
            },
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
                self.handle_LDA_IMM(mem);
            }
            instructions::LDA::ZP => {
                self.handle_LDA_ZP(mem);
            }
            instructions::LDA::ZPX => self.handle_LDA_ZPX(mem),
            instructions::LDA::ABS => self.handle_LDA_ABS(mem),
            instructions::LDA::ABSX => self.handle_LDA_ABSX(mem),
            instructions::LDA::ABSY => self.handle_LDA_ABSY(mem),
            instructions::LDA::INDX => self.handle_LDA_INDX(mem),
            instructions::LDA::INDY => self.handle_LDA_INDY(mem),
            instructions::LDX::IMM => self.handle_LDX_IMM(mem),
            instructions::LDX::ZP => self.handle_LDX_ZP(mem),
            instructions::LDX::ZPY => self.handle_LDX_ZPY(mem),
            instructions::LDX::ABS => self.handle_LDX_ABS(mem),
            instructions::LDX::ABSY => self.handle_LDX_ABSY(mem),
            instructions::LDY::IMM => self.handle_LDY_IMM(mem),
            instructions::LDY::ZP => self.handle_LDY_ZP(mem),
            instructions::LDY::ZPX => self.handle_LDY_ZPX(mem),
            instructions::LDY::ABS => self.handle_LDY_ABS(mem),
            instructions::LDY::ABSX => self.handle_LDY_ABSX(mem),
            instructions::JMP::ABS => self.handle_JMP_ABS(mem),
            instructions::JMP::IND => self.handle_JMP_IND(mem),
            instructions::INX::IMP => self.handle_INX_IMP(),
            instructions::INY::IMP => self.handle_INY_IMP(),

            _ => panic!("Unknown opcode: {:X}", opcode),
        }
    }

    fn set_flags_LDA(&mut self) {
        self.Status.Zero = self.A == 0;
        self.Status.Negative = (self.A & 0b1000_0000) > 0;
    }

    fn set_flags_LDX(&mut self) {
        self.Status.Zero = self.X == 0;
        self.Status.Negative = (self.X & 0b1000_0000) > 0;
    }

    fn set_flags_LDY(&mut self) {
        self.Status.Zero = self.Y == 0;
        self.Status.Negative = (self.Y & 0b1000_0000) > 0;
    }

    fn ZP_ADDRESSING(&mut self, mem: &mut Memory) -> Word {
        let address: Word = (0x0000 | self.fetch_byte(mem)) as Word;
        address
    }

    fn ABSX_ADDRESSING(&mut self, mem: &mut Memory) -> Word {
        let base_address: Word = self.fetch_word(mem);
        let address = base_address + self.X as Word;
        address
    }

    fn ABSY_ADDRESSING(&mut self, mem: &mut Memory) -> Word {
        let base_address: Word = self.fetch_word(mem);
        let address = base_address + self.Y as Word;
        address
    }

    fn ABS_ADDRESSING(&mut self, mem: &mut Memory) -> Word {
        let address: Word = self.fetch_word(mem);
        address
    }

    fn INDX_ADDRESSING(&mut self, mem: &mut Memory) -> Word {
        let mut address: Word = (0x0000 | self.fetch_byte(mem)) as Word;
        address = address + self.X as Word;
        let lo = self.read_byte(mem, address);
        let hi = self.read_byte(mem, address + 0x01);
        let effective_address = ((hi as u16) << 8) | lo as u16;
        effective_address
    }

    fn INDY_ADDRESSING(&mut self, mem: &mut Memory) -> Word {
        let zp_address: Word = (0x0000 | self.fetch_byte(mem)) as Word;
        let lo = self.read_byte(mem, zp_address);
        let hi = self.read_byte(mem, zp_address + 0x01);
        let base_address = (((hi as u16) << 8) | lo as u16);
        let effective_address = base_address + self.Y as u16;
        effective_address
    }

    fn handle_LDA_IMM(&mut self, mem: &mut Memory) {
        let value = self.fetch_byte(mem);
        self.A = value;
        self.set_flags_LDA()
    }

    fn handle_LDA_ZP(&mut self, mem: &mut Memory) {
        let address = self.ZP_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.A = value;
        self.set_flags_LDA()
    }

    fn handle_LDA_ZPX(&mut self, mem: &mut Memory) {
        let address = self.ZP_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.A = value + self.X;
        self.set_flags_LDA()
    }

    fn handle_LDA_ABS(&mut self, mem: &mut Memory) {
        let address = self.ABS_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.A = value + self.Y;
        self.set_flags_LDA()
    }

    fn handle_LDA_ABSX(&mut self, mem: &mut Memory) {
        let address = self.ABSX_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.A = value;
        self.set_flags_LDA()
    }

    fn handle_LDA_ABSY(&mut self, mem: &mut Memory) {
        let address = self.ABSY_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.A = value;
        self.set_flags_LDA()
    }

    fn handle_LDA_INDX(&mut self, mem: &mut Memory) {
        let address = self.INDX_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.A = value;
        self.set_flags_LDA()
    }

    fn handle_LDA_INDY(&mut self, mem: &mut Memory) {
        let address = self.INDY_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.A = value;
        self.set_flags_LDA()
    }

    fn handle_LDX_IMM(&mut self, mem: &mut Memory) {
        let value = self.fetch_byte(mem);
        self.X = value;
        self.set_flags_LDX()
    }

    fn handle_LDX_ZP(&mut self, mem: &mut Memory) {
        let address = self.ZP_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.X = value;
        self.set_flags_LDX()
    }

    fn handle_LDX_ZPY(&mut self, mem: &mut Memory) {
        let address = self.ZP_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.X = value + self.Y;
        self.set_flags_LDX()
    }

    fn handle_LDX_ABS(&mut self, mem: &mut Memory) {
        let address = self.ABS_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.X = value;
        self.set_flags_LDX()
    }

    fn handle_LDX_ABSY(&mut self, mem: &mut Memory) {
        let address = self.ABSY_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.X = value;
        self.set_flags_LDX()
    }

    fn handle_LDY_IMM(&mut self, mem: &mut Memory) {
        let value = self.fetch_byte(mem);
        self.Y = value;
        self.set_flags_LDY()
    }

    fn handle_LDY_ZP(&mut self, mem: &mut Memory) {
        let address = self.ZP_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.Y = value;
        self.set_flags_LDY()
    }

    fn handle_LDY_ZPX(&mut self, mem: &mut Memory) {
        let address = self.ZP_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.Y = value + self.X;
        self.set_flags_LDY()
    }

    fn handle_LDY_ABS(&mut self, mem: &mut Memory) {
        let address = self.ABS_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.Y = value;
        self.set_flags_LDY()
    }

    fn handle_LDY_ABSX(&mut self, mem: &mut Memory) {
        let address = self.ABSX_ADDRESSING(mem);
        let value = self.read_byte(mem, address);
        self.Y = value;
        self.set_flags_LDY()
    }

    fn handle_JMP_ABS(&mut self, mem: &mut Memory) {
        let address = self.fetch_word(mem);
        self.PC = address;
    }

    fn handle_JMP_IND(&mut self, mem: &mut Memory) {
        let address = self.fetch_word(mem);
        let lo = self.read_byte(mem, address);
        let hi = self.read_byte(mem, address + 0x01);
        let address = ((hi as u16) << 8) | lo as u16;
        self.PC = address;
    }

    fn handle_INX_IMP(&mut self) {
        self.X = self.X.wrapping_add(1);
        self.set_flags_LDX()
    }

    fn handle_INY_IMP(&mut self) {
        self.Y = self.Y.wrapping_add(1);
        self.set_flags_LDY()
    }
}
