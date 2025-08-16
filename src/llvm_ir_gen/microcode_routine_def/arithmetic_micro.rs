use inkwell::module::Linkage;

use crate::llvm_ir_gen::ir_gen::IRGenUtil;

pub fn add_sr2(ir: &mut IRGenUtil) -> () {
    let add_fn_type = ir.context.void_type().fn_type(&[ir.i16p_type.into(), ir.i16p_type.into(), ir.i16p_type.into()], false);
    let add_fn = ir.module.add_function("add_sr2", add_fn_type, Some(Linkage::Internal));
    let add_entry_blk = ir.context.append_basic_block(add_fn, "add_sr2");
    ir.builder.position_at_end(add_entry_blk);

    let dr = add_fn.get_nth_param(0).unwrap().into_pointer_value();
    let sr1 = add_fn.get_nth_param(1).unwrap().into_pointer_value();
    let sr2 = add_fn.get_nth_param(2).unwrap().into_pointer_value();

    dr.set_name("dr");
    sr1.set_name("sr1");
    sr2.set_name("sr2");

    
    let sr1_val = ir.builder.build_load(ir.i16_type, sr1, "sr1_val").unwrap().into_int_value();
    let sr2_val = ir.builder.build_load(ir.i16_type, sr2, "sr2_val").unwrap().into_int_value();
    let result = ir.builder.build_int_add(sr1_val, sr2_val, "res").unwrap();

    ir.builder.build_store(dr, result).unwrap();

    ir.cache_microcode("add_sr2".to_string(), add_fn);

    let _ = ir.builder.build_return(None);
}

pub fn add_imm5(ir: &mut IRGenUtil) -> () {
    let add_fn_type = ir.context.void_type().fn_type(&[ir.i16p_type.into(), ir.i16p_type.into(), ir.i16_type.into()], false);
    let add_fn = ir.module.add_function("add_imm5", add_fn_type, Some(Linkage::Internal));
    let add_entry_blk = ir.context.append_basic_block(add_fn, "add_imm5");
    ir.builder.position_at_end(add_entry_blk);

    let dr = add_fn.get_nth_param(0).unwrap().into_pointer_value();
    let sr1 = add_fn.get_nth_param(1).unwrap().into_pointer_value();
    let imm5 = add_fn.get_nth_param(2).unwrap().into_int_value();

    dr.set_name("dr");
    sr1.set_name("sr1");
    imm5.set_name("imm5");

    let sr1_val = ir.builder.build_load(ir.i16_type, sr1, "sr1_val").unwrap().into_int_value();
    let result = ir.builder.build_int_add(sr1_val, imm5, "res").unwrap();

    ir.builder.build_store(dr, result).unwrap();

    ir.cache_microcode("add_imm5".to_string(), add_fn);

    let _ = ir.builder.build_return(None);
}