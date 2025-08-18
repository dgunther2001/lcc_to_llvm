use inkwell::basic_block::BasicBlock;

use crate::llvm_ir_gen::{ir_gen_structures::IRGenUtil, microcode_routine_def::{add_imm5, add_sr2, and_imm5, and_sr2, din, div, dout, mul, nl, not, or, sub_imm5, sub_sr2, xor}};

type MicrocodeInitSignature = fn(&mut IRGenUtil);

const MICROCODE_INITIALIZATION_TABLE: &[MicrocodeInitSignature] = &[
    add_sr2,
    add_imm5,
    sub_sr2,
    sub_imm5,
    mul,
    div,
    and_sr2,
    and_imm5,
    or,
    not,
    xor,
    dout, 
    din, 
    nl,
];

pub fn initialize_microcode_wrappers(ir: &mut IRGenUtil, entry_block: &BasicBlock) -> () {
    for microcode_init_function in MICROCODE_INITIALIZATION_TABLE {
        microcode_init_function(ir);
        reset_to_entry_block(ir, entry_block);
    }
}

pub fn reset_to_entry_block(ir: &IRGenUtil, entry_block: &BasicBlock) -> () {
    ir.builder.position_at_end(*entry_block);
}