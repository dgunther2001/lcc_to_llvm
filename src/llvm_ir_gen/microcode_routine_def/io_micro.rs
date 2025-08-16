use crate::llvm_ir_gen::{ir_gen_structures::IRGenUtil, ir_gen_structures::FormatStringKey, microcode_routine_def::{create_void_return, declare_microcode, MicrocodeCallSignature}};

pub fn dout(ir: &mut IRGenUtil) {
    let (_, args) = declare_microcode(ir, "dout", MicrocodeCallSignature::Sr);
    let sr_val = ir.builder.build_load(ir.context.i16_type(), args.sr.unwrap(), "sr_val").expect("Failed to load from register");
    ir.builder.build_call(ir.extract_microcode_function("printf").unwrap(), &[ir.format_string_registry.get_fmt_string(FormatStringKey::SignedInt).into(), sr_val.into()], "printf_call").unwrap();
    create_void_return(ir);
}

pub fn nl(ir: &mut IRGenUtil) {
    let _ = declare_microcode(ir, "nl", MicrocodeCallSignature::Void);
    ir.builder.build_call(ir.extract_microcode_function("printf").unwrap(), &[ir.format_string_registry.get_fmt_string(FormatStringKey::Newline).into()], "printf_call").unwrap();
    create_void_return(ir);
}

pub fn din(ir: &mut IRGenUtil) {
    let (_, args) = declare_microcode(ir, "din", MicrocodeCallSignature::Dr);
    ir.builder.build_call(ir.extract_microcode_function("scanf").unwrap(), &[ir.format_string_registry.get_fmt_string(FormatStringKey::SignedInt).into(), inkwell::values::BasicMetadataValueEnum::PointerValue(args.dr.unwrap())], "scanf_call").unwrap();
    create_void_return(ir);
}