#[allow(unused, non_snake_case)]
pub mod LDA {
    use crate::Byte;

    pub const IMM: Byte = 0xA9;
    pub const ZP: Byte = 0xA5;
    pub const ZPX: Byte = 0xB5;
    pub const ABS: Byte = 0xAD;
    pub const ABSX: Byte = 0xBD;
    pub const ABSY: Byte = 0xB9;
    pub const INDX: Byte = 0xA1;
    pub const INDY: Byte = 0xB1;
}
#[allow(unused, non_snake_case)]
pub mod LDX {
    use crate::Byte;

    pub const IMM: Byte = 0xA2;
    pub const ZP: Byte = 0xA6;
    pub const ZPY: Byte = 0xB6;
    pub const ABS: Byte = 0xAE;
    pub const ABSY: Byte = 0xBE;
}
#[allow(unused, non_snake_case)]
pub mod LDY {
    use crate::Byte;

    pub const IMM: Byte = 0xA0;
    pub const ZP: Byte = 0xA4;
    pub const ZPX: Byte = 0xB4;
    pub const ABS: Byte = 0xAC;
    pub const ABSX: Byte = 0xBC;
}
#[allow(unused, non_snake_case)]
pub mod JMP {
    use crate::Byte;

    pub const ABS: Byte = 0x4C;
    pub const IND: Byte = 0x6C;
}

#[allow(unused, non_snake_case)]
pub mod INX {
    use crate::Byte;

    pub const IMP: Byte = 0xE8;
}

#[allow(unused, non_snake_case)]
pub mod INY {
    use crate::Byte;

    pub const IMP: Byte = 0xC8;
}
