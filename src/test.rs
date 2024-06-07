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

    cpu.execute(&mut mem, &mut 4);

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

    cpu.execute(&mut mem, &mut 3);

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

    cpu.execute(&mut mem, &mut 2);

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

    cpu.execute(&mut mem, &mut 2);

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

    cpu.execute(&mut mem, &mut 2);

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

    cpu.execute(&mut mem, &mut 4);

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

    cpu.execute(&mut mem, &mut 4);

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

    cpu.execute(&mut mem, &mut 4);

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

    cpu.execute(&mut mem, &mut 6);

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

    cpu.execute(&mut mem, &mut 5);

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

    cpu.execute(&mut mem, &mut 6);

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