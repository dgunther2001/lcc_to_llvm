use crate::{definitions::{reg_defn::Registers}};


#[derive(Debug)]
pub enum AsmLine {
    Add  { dr: Registers, sr1: Registers, sr2: RegOrImmOperand },
    Sub  { dr: Registers, sr1: Registers, sr2: RegOrImmOperand },
    Mul  { dr: Registers, sr: Registers},
    Div  { dr: Registers, sr: Registers},
    //Ld   { dr: Registers, loc: LabelOrOffset },
    //St   { dr: Registers, loc: LabelOrOffset },
    Dout { dr: Registers },
    Din  { dr: Registers },
    Nl   { },
    //Invalid
}

#[derive(Debug)]
pub enum RegOrImmOperand {
    Register(Registers),
    Immediate(i16)
}

#[derive(Debug)]
pub enum LabelOrOffset {
    Label(String),
    Offset(i16)
}