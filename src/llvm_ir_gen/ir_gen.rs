use inkwell::{AddressSpace};
use crate::llvm_ir_gen::ir_gen_structures::IRGenUtil;
use crate::llvm_ir_gen::microcode_routine_def::microcode_lib::initialize_microcode_wrappers;
use crate::parser::parser_defns::{AsmLine, RegOrImmOperand};

pub fn populate_module(lines: Vec<AsmLine>, ir: &mut IRGenUtil) -> () {
    // add printf functionality, etc....
    add_c_std_lib_funcs(ir);

    let entry_function_type = ir.i32_type.fn_type(&[], false);
    let entry_function = ir.module.add_function("main", entry_function_type, None);
    let entry_block = ir.context.append_basic_block(entry_function, "entry");
    ir.builder.position_at_end(entry_block);

    ir.format_string_registry.initialize_registry(&ir.builder);

    // generate all of the microcode (lcc function) wrappers and add them to the cached function table
    initialize_microcode_wrappers(ir, &entry_block);

    // generate llvm ir in the primary module from the lcc assembly
    generate_ir_from_asm_lines(ir, lines);

    // TEMORARY
    let _ = ir.builder.build_return(Some(&ir.i32_type.const_int(0, false)));


    //ir.module.print_to_stderr();

}









fn generate_ir_from_asm_lines(ir: &mut IRGenUtil, lines: Vec<AsmLine>) -> () {

    for line in lines {
        match line {
            AsmLine::Add { dr, sr1, sr2 } => {
                match sr2 {
                    RegOrImmOperand::Register(sr2_reg) => {
                        ir.builder.build_call(ir.extract_microcode_function("add_sr2").unwrap(), &[ir.get_register_ptr_const(dr as u32).into(), ir.get_register_ptr_const(sr1 as u32).into(), ir.get_register_ptr_const(sr2_reg as u32).into()], "").unwrap();
                    }
                    RegOrImmOperand::Immediate(imm5) => {
                        ir.builder.build_call(ir.extract_microcode_function("add_imm5").unwrap(), &[ir.get_register_ptr_const(dr as u32).into(), ir.get_register_ptr_const(sr1 as u32).into(), ir.i16_type.const_int(imm5 as u64, true).into()], "").unwrap();
                    }
                }
            }
            AsmLine::Sub { dr, sr1, sr2 } => {
                match sr2 {
                    RegOrImmOperand::Register(sr2_reg) => {
                        ir.builder.build_call(ir.extract_microcode_function("sub_sr2").unwrap(), &[ir.get_register_ptr_const(dr as u32).into(), ir.get_register_ptr_const(sr1 as u32).into(), ir.get_register_ptr_const(sr2_reg as u32).into()], "").unwrap();
                    }
                    RegOrImmOperand::Immediate(imm5) => {
                        ir.builder.build_call(ir.extract_microcode_function("sub_imm5").unwrap(), &[ir.get_register_ptr_const(dr as u32).into(), ir.get_register_ptr_const(sr1 as u32).into(), ir.i16_type.const_int(imm5 as u64, true).into()], "").unwrap();
                    }
                }
            }
            AsmLine::Mul { dr, sr } => {
                ir.builder.build_call(ir.extract_microcode_function("mul").unwrap(), &[ir.get_register_ptr_const(dr as u32).into(), ir.get_register_ptr_const(sr as u32).into()],"").unwrap();
            }
            AsmLine::Div { dr, sr } => {
                ir.builder.build_call(ir.extract_microcode_function("div").unwrap(), &[ir.get_register_ptr_const(dr as u32).into(), ir.get_register_ptr_const(sr as u32).into()],"").unwrap();
            }
            

            AsmLine::And { dr, sr1, sr2 } => {
                match sr2 {
                    RegOrImmOperand::Register(sr2_reg) => {
                        ir.builder.build_call(ir.extract_microcode_function("and_sr2").unwrap(), &[ir.get_register_ptr_const(dr as u32).into(), ir.get_register_ptr_const(sr1 as u32).into(), ir.get_register_ptr_const(sr2_reg as u32).into()], "").unwrap();
                    }
                    RegOrImmOperand::Immediate(imm5) => {
                        ir.builder.build_call(ir.extract_microcode_function("and_imm5").unwrap(), &[ir.get_register_ptr_const(dr as u32).into(), ir.get_register_ptr_const(sr1 as u32).into(), ir.i16_type.const_int(imm5 as u64, true).into()], "").unwrap();
                    }
                }
            }
            AsmLine::Or { dr, sr } => {
                ir.builder.build_call(ir.extract_microcode_function("or").unwrap(), &[ir.get_register_ptr_const(dr as u32).into(), ir.get_register_ptr_const(sr as u32).into()],"").unwrap();
            }
            AsmLine::Xor { dr, sr } => {
                ir.builder.build_call(ir.extract_microcode_function("xor").unwrap(), &[ir.get_register_ptr_const(dr as u32).into(), ir.get_register_ptr_const(sr as u32).into()],"").unwrap();
            }
            AsmLine::Not { dr, sr } => {
                ir.builder.build_call(ir.extract_microcode_function("not").unwrap(), &[ir.get_register_ptr_const(dr as u32).into(), ir.get_register_ptr_const(sr as u32).into()],"").unwrap();
            }

            AsmLine::Dout { dr } => {
                ir.builder.build_call(ir.extract_microcode_function("dout").unwrap(), &[ir.get_register_ptr_const(dr as u32).into()],"").unwrap();
            }
            AsmLine::Nl { } => {
                ir.builder.build_call(ir.extract_microcode_function("nl").unwrap(), &[],"").unwrap();
            }
            AsmLine::Din { dr } => {
                ir.builder.build_call(ir.extract_microcode_function("din").unwrap(), &[ir.get_register_ptr_const(dr as u32).into()],"").unwrap();
            }
            _ => todo!()
        }
    }
}




fn add_c_std_lib_funcs(ir: &mut IRGenUtil) -> () {
    let i8p_type = ir.context.i8_type().ptr_type(AddressSpace::default());
    let io_func_type = ir.context.i32_type().fn_type(&[i8p_type.into()], true);


    let printf_fn = ir.module.add_function("printf", io_func_type, None);
    printf_fn.get_nth_param(0).unwrap().into_pointer_value().set_name("format");
    ir.cache_microcode("printf".to_string(), printf_fn);


    let scanf_fn =  ir.module.add_function("scanf", io_func_type, None);
    scanf_fn.get_nth_param(0).unwrap().into_pointer_value().set_name("format");
    ir.cache_microcode("scanf".to_string(), scanf_fn);
}

pub fn write_module(ir: &IRGenUtil, output_path: &str) -> () {
    let _ = ir.module.print_to_file(output_path);
}

pub fn set_triple(ir: &mut IRGenUtil) -> () {
    let target_triple = inkwell::targets::TargetMachine::get_default_triple();
    ir.module.set_triple(&target_triple);
}