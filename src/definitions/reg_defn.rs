
#[derive(Debug, Clone, Copy)]
pub enum Registers {
    R0,
    R1,
    R2,
    R3,
    R4,
    Fp, // r5
    Sp, // r6
    Lr, // r7
}

pub fn match_string_to_reg(reg: &str) -> Option<Registers> {
    match reg {
        "r0"    => Some(Registers::R0),
        "r1"    => Some(Registers::R1),
        "r2"    => Some(Registers::R2),
        "r3"    => Some(Registers::R3),
        "r4"    => Some(Registers::R4),
        "r5"    => Some(Registers::Fp),
        "r6"    => Some(Registers::Sp),
        "r7"    => Some(Registers::Lr),
        "fp"    => Some(Registers::Fp),
        "sp"    => Some(Registers::Sp),
        "lr"    => Some(Registers::Lr),
        _       => None
    }
}