use crate::definitions;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Opcode(definitions::op_defn::Opcode),
    Register(definitions::reg_defn::Registers),
    Label(String),
    Numeric(i16),
    Directive(String),
    Identifier(String),
    String(String),
}