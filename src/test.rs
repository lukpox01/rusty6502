use crate::cpu::CPU;
use crate::instructions;
use crate::memory::Memory;

#[allow(non_snake_case)]
#[test]
fn LDA_ZPX_CAN_LOAD_AND_OFFSET() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDA::ZPX;
    mem.data[0xFFFD] = 0x10;
    mem.data[0x0010] = 0x11;
    cpu.X = 0x5;

    cpu.execute(&mut mem);

    assert_eq!(cpu.A, 0x16);

    // Flags
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDA_ZP_CAN_LOAD() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDA::ZP;
    mem.data[0xFFFD] = 0x10;
    mem.data[0x0010] = 0x11;

    cpu.execute(&mut mem);

    assert_eq!(cpu.A, 0x11);

    // Flags
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDA_IMM_CAN_LOAD() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDA::IMM;
    mem.data[0xFFFD] = 0x10;

    cpu.execute(&mut mem);

    assert_eq!(cpu.A, 0x10);

    // Flags
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDA_FLAG_NEGATIVE() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDA::IMM;
    mem.data[0xFFFD] = 0x80;

    cpu.execute(&mut mem);

    assert_eq!(cpu.A, 0x80);

    // Flags
    assert_eq!(cpu.Status.Negative, true);
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDA_FLAG_ZERO() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDA::IMM;
    mem.data[0xFFFD] = 0x00;

    cpu.execute(&mut mem);

    assert_eq!(cpu.A, 0x00);

    // Flags
    assert_eq!(cpu.Status.Zero, true);
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDA_ABS_CAN_LOAD() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDA::ABS;
    mem.data[0xFFFD] = 0x80;
    mem.data[0xFFFE] = 0x80;
    mem.data[0x8080] = 0x15;

    cpu.execute(&mut mem);

    assert_eq!(cpu.A, 0x15);

    // Flags
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDA_ABSX_CAN_LOAD() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDA::ABSX;
    mem.data[0xFFFD] = 0x80;
    mem.data[0xFFFE] = 0x80;
    mem.data[0x8081] = 0x15;
    cpu.X = 0x01;

    cpu.execute(&mut mem);

    assert_eq!(cpu.A, 0x15);

    // Flags
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDA_ABSY_CAN_LOAD() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDA::ABSY;
    mem.data[0xFFFD] = 0x81;
    mem.data[0xFFFE] = 0x81;
    mem.data[0x8183] = 0x0B;
    cpu.Y = 0x02;

    cpu.execute(&mut mem);

    assert_eq!(cpu.A, 0x0B);

    // Flags
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDA_INDX_CAN_LOAD() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDA::INDX;
    mem.data[0xFFFD] = 0x1E;
    mem.data[0x0020] = 0x05;
    mem.data[0x0021] = 0x10;
    mem.data[0x1005] = 0x0F;
    cpu.X = 0x02;

    cpu.execute(&mut mem);

    assert_eq!(cpu.A, 0x0F);

    // Flags
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDA_INDY_CAN_LOAD() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDA::INDY;
    mem.data[0xFFFD] = 0x20;
    mem.data[0x0020] = 0x30;
    mem.data[0x0021] = 0x40;
    mem.data[0x4035] = 0x22;
    cpu.Y = 0x05;

    cpu.execute(&mut mem);

    assert_eq!(cpu.A, 0x22);

    // Flags
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDA_INDY_CROSSED_PAGE() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDA::INDY;
    mem.data[0xFFFD] = 0x20;
    mem.data[0x0020] = 0xFF;
    mem.data[0x0021] = 0x30;
    mem.data[0x3101] = 0x80;
    cpu.Y = 0x02;

    cpu.execute(&mut mem);

    assert_eq!(cpu.A, 0x80);

    // Flags
    assert_eq!(cpu.Status.Negative, true);
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

// LDX
#[allow(non_snake_case)]
#[test]
fn LDX_ZPY_CAN_LOAD_AND_OFFSET() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDX::ZPY;
    mem.data[0xFFFD] = 0x10;
    mem.data[0x0010] = 0x11;
    cpu.Y = 0x5;

    cpu.execute(&mut mem);

    assert_eq!(cpu.X, 0x16);

    // Flags
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDX_ZP_CAN_LOAD() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDX::ZP;
    mem.data[0xFFFD] = 0x10;
    mem.data[0x0010] = 0x11;

    cpu.execute(&mut mem);

    assert_eq!(cpu.X, 0x11);

    // Flags
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDX_IMM_CAN_LOAD() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDX::IMM;
    mem.data[0xFFFD] = 0x10;

    cpu.execute(&mut mem);

    assert_eq!(cpu.X, 0x10);

    // Flags
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

// LDY
#[allow(non_snake_case)]
#[test]
fn LDY_ZPX_CAN_LOAD_AND_OFFSET() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDY::ZPX;
    mem.data[0xFFFD] = 0x10;
    mem.data[0x0010] = 0x11;
    cpu.X = 0x5;

    cpu.execute(&mut mem);

    assert_eq!(cpu.Y, 0x16);

    // Flags
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDY_ZP_CAN_LOAD() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDY::ZP;
    mem.data[0xFFFD] = 0x10;
    mem.data[0x0010] = 0x11;

    cpu.execute(&mut mem);

    assert_eq!(cpu.Y, 0x11);

    // Flags
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

#[allow(non_snake_case)]
#[test]
fn LDY_ABS_CAN_LOAD() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::LDY::ABS;
    mem.data[0xFFFD] = 0x80;
    mem.data[0xFFFE] = 0x80;
    mem.data[0x8080] = 0x15;

    cpu.execute(&mut mem);

    assert_eq!(cpu.Y, 0x15);

    // Flags
    assert_eq!(cpu.Status.Negative, false);
    assert_eq!(cpu.Status.Zero, false);
    assert_eq!(cpu.Status.Overflow, false);
    assert_eq!(cpu.Status.Carry, false);
    assert_eq!(cpu.Status.DecimalMode, false);
    assert_eq!(cpu.Status.InterruptDisable, false);
    assert_eq!(cpu.Status.Break, false);
}

// JMP

#[allow(non_snake_case)]
#[test]
fn JMP_ABS_CAN_JUMP() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::JMP::ABS;
    mem.data[0xFFFD] = 0x80;
    mem.data[0xFFFE] = 0x80;

    cpu.execute(&mut mem);

    assert_eq!(cpu.PC, 0x8080);
}

#[allow(non_snake_case)]
#[test]
fn JMP_IND_CAN_JUMP() {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();
    cpu.reset();

    mem.data[0xFFFC] = instructions::JMP::IND;
    mem.data[0xFFFD] = 0x80;
    mem.data[0xFFFE] = 0x80;
    mem.data[0x8080] = 0x90;
    mem.data[0x8081] = 0x80;

    cpu.execute(&mut mem);

    assert_eq!(cpu.PC, 0x8090);
}
