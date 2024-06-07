mod instructions;


// http://www.6502.org/users/obelisk/6502/index.html
type Byte = u8;
type Word = u16;
// type DoubleWord = u32;
type Cycle = u8;


struct Memory {
    data: [Byte; 64*1024],
}

impl Memory {
    fn new() -> Memory {
        Memory{data: [0; 64*1024]}
    }
}

#[allow(non_snake_case)]
struct Flags {
    Carry: bool,
    Zero: bool,
    InterruptDisable: bool,
    DecimalMode: bool,
    Break: bool,
    Overflow: bool,
    Negative: bool,
}

#[allow(non_snake_case)]
struct CPU {
    PC: Word, // Program Counter
    SP: Byte, // Stack Pointer

    // Registers
    A: Byte, // Accumulator
    X: Byte,
    Y: Byte,

    // Status Register
    Status: Flags,

}


#[allow(non_snake_case)]
impl CPU {
    fn new() -> CPU {
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

    fn reset(&mut self) {
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

    fn execute(&mut self, mem: &mut Memory) {
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


fn main() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDA::ZPX;
    mem.data[0xFFFD] = 0x10;
    mem.data[0x0010] = 0x11;
    cpu.X = 0x5;

    cpu.execute(&mut mem);

    println!("A - {}", cpu.A);
}
