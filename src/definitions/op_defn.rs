#[derive(Debug)]
pub enum Opcode {
    Add,
    Ld,
    St, /*
    Bl,
    Blr,
    And,
    Ldr,
    Str,
    Cmp,
    Not,
    Push, 
    Pop,
    Srl,
    Sra,
    Sll,
    Rol,
    Ror, 
    */
    Mul,
    Div,
    /* 
    Rem,
    Or,
    Xor,
    Mvr,
    Sext,
    */
    Sub,
    /*
    Jmp,
    Ret,
    Mvi,
    Lea,
    Halt,
    */
    Nl,
    Dout, /* 
    Uout,
    Hout,
    Aout,
    Sout, */
    Din,
    /*
    Hin,
    Ain,
    Sin,
    Brz,
    Bre,
    Brnz,
    Brne,
    Brn,
    Brp,
    Brlt,
    Brgt,
    Brc,
    Brb,
    Br,
    Bral */
}

pub fn match_opcode(opcode: &str) -> Option<Opcode> {
    match opcode {
        "add"   => Some(Opcode::Add),
        "ld"    => Some(Opcode::Ld),
        "st"    => Some(Opcode::St),
        "sub"   => Some(Opcode::Sub),
        "mul"   => Some(Opcode::Mul),
        "div"   => Some(Opcode::Div),
        "dout"  => Some(Opcode::Dout),
        "din"   => Some(Opcode::Din),
        "nl"    => Some(Opcode::Nl),
        _       => None
    }
}

/*
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
    }
}
    */