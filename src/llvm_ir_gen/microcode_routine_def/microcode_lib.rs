use inkwell::basic_block::BasicBlock;

use crate::llvm_ir_gen::{ir_gen::IRGenUtil, microcode_routine_def::{add_imm5, add_sr2, div, dout, mul, nl, sub_imm5, sub_sr2}};



pub fn initialize_microcode_wrappers(ir: &mut IRGenUtil, entry_block: &BasicBlock) -> () {
    add_sr2(ir);
    reset_to_entry_block(ir, entry_block);
    add_imm5(ir);
    reset_to_entry_block(ir, entry_block);
    sub_sr2(ir);
    reset_to_entry_block(ir, entry_block);
    sub_imm5(ir);
    reset_to_entry_block(ir, entry_block);
    mul(ir);
    reset_to_entry_block(ir, entry_block);
    div(ir);
    reset_to_entry_block(ir, entry_block);


    dout(ir);
    reset_to_entry_block(ir, entry_block);
    nl(ir);
    reset_to_entry_block(ir, entry_block);

}

pub fn reset_to_entry_block(ir: &IRGenUtil, entry_block: &BasicBlock) -> () {
    ir.builder.position_at_end(*entry_block);
}