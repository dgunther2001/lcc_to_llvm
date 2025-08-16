use inkwell::module::Linkage;

use crate::llvm_ir_gen::ir_gen::IRGenUtil;



pub fn dout(ir: &mut IRGenUtil) {
    let dout_fn_type = ir.context.void_type().fn_type(&[ir.i16p_type.into()], false);
    let dout_fn = ir.module.add_function("dout", dout_fn_type, Some(Linkage::Internal));
    let dout_entry_blk = ir.context.append_basic_block(dout_fn, "dout");
    ir.builder.position_at_end(dout_entry_blk);

    let dr = dout_fn.get_nth_param(0).unwrap().into_pointer_value();

    let fmt_str = ir.builder.build_global_string_ptr("%d", "dout_fmt").expect("Failed to build global string").as_pointer_value();
    let loaded_val = ir.builder.build_load(ir.context.i16_type(), dr, "loaded_val").expect("Failed to load from register");
    //let zext_val = ir.builder.build_int_z_extend(loaded_val.into_int_value(), ir.context.i32_type(), "zext_val").expect("Failed to build zero extended i32 for DOUT");

    ir.builder.build_call(ir.extract_microcode_function("printf").unwrap(), &[fmt_str.into(), loaded_val.into()], "printf_call").unwrap();

    ir.cache_microcode("dout".to_string(), dout_fn);

    let _ = ir.builder.build_return(None);
}

pub fn nl(ir: &mut IRGenUtil) {
    let nl_fn_type = ir.context.void_type().fn_type(&[ir.i16p_type.into()], false);
    let nl_fn = ir.module.add_function("nl", nl_fn_type, Some(Linkage::Internal));
    let nl_entry_blk = ir.context.append_basic_block(nl_fn, "nl");
    ir.builder.position_at_end(nl_entry_blk);
    let fmt_str = ir.builder.build_global_string_ptr("\n", "nl_fmt").expect("Failed to build global string").as_pointer_value();
    ir.builder.build_call(ir.extract_microcode_function("printf").unwrap(), &[fmt_str.into()], "printf_call").unwrap();
    ir.cache_microcode("nl".to_string(), nl_fn);
    let _ = ir.builder.build_return(None);
}