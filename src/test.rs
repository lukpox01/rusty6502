
use crate::memory::Memory;
use crate::cpu::CPU;
use crate::instructions;

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

    assert_eq!(cpu.A, 0x16)
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

    assert_eq!(cpu.A, 0x11)
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

    assert_eq!(cpu.A, 0x10)
}