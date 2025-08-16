use crate::llvm_ir_gen::{ir_gen_structures::IRGenUtil, microcode_routine_def::{create_void_return, declare_microcode, load_i16, store_i16, MicrocodeCallSignature}};

pub fn add_sr2(ir: &mut IRGenUtil) -> () {
    let (_, args) = declare_microcode(ir, "add_sr2", MicrocodeCallSignature::DrSrSr);
    let sr1 = load_i16(ir, args.sr1.unwrap(), "sr1_val");
    let sr2 = load_i16(ir, args.sr2.unwrap(), "sr2_val");
    let res = ir.builder.build_int_add(sr1, sr2, "res").unwrap();
    store_i16(ir, args.dr.unwrap(), res);
    create_void_return(ir);
}

pub fn add_imm5(ir: &mut IRGenUtil) -> () {
    let (_, args) = declare_microcode(ir, "add_imm5", MicrocodeCallSignature::DrSrImm5);
    let sr = load_i16(ir, args.sr.unwrap(), "sr_val");
    let imm5 = args.imm_or_offset.unwrap();
    let res = ir.builder.build_int_add(sr, imm5, "res").unwrap();
    store_i16(ir, args.dr.unwrap(), res);
    create_void_return(ir);
}

pub fn sub_sr2(ir: &mut IRGenUtil) -> () {
    let (_, args) = declare_microcode(ir, "sub_sr2", MicrocodeCallSignature::DrSrSr);
    let sr1 = load_i16(ir, args.sr1.unwrap(), "sr1_val");
    let sr2 = load_i16(ir, args.sr2.unwrap(), "sr2_val");
    let res = ir.builder.build_int_sub(sr1, sr2, "res").unwrap();
    store_i16(ir, args.dr.unwrap(), res);
    create_void_return(ir);
}

pub fn sub_imm5(ir: &mut IRGenUtil) -> () {
    let (_, args) = declare_microcode(ir, "sub_imm5", MicrocodeCallSignature::DrSrImm5);
    let sr = load_i16(ir, args.sr.unwrap(), "sr_val");
    let imm5 = args.imm_or_offset.unwrap();
    let res = ir.builder.build_int_sub(sr, imm5, "res").unwrap();
    store_i16(ir, args.dr.unwrap(), res);
    create_void_return(ir);
}

pub fn mul(ir: &mut IRGenUtil) -> () {
    let (_, args) = declare_microcode(ir, "mul", MicrocodeCallSignature::DrSr);
    let dr = load_i16(ir, args.dr.unwrap(), "dr_val");
    let sr = load_i16(ir, args.sr.unwrap(), "sr_val");
    let res = ir.builder.build_int_mul(dr, sr, "res").unwrap();
    store_i16(ir, args.dr.unwrap(), res);
    create_void_return(ir);
}

pub fn div(ir: &mut IRGenUtil) -> () {
    let (_, args) = declare_microcode(ir, "div", MicrocodeCallSignature::DrSr);
    let dr = load_i16(ir, args.dr.unwrap(), "dr_val");
    let sr = load_i16(ir, args.sr.unwrap(), "sr_val");
    let res = ir.builder.build_int_unsigned_div(dr, sr, "res").unwrap();
    store_i16(ir, args.dr.unwrap(), res);
    create_void_return(ir);
}