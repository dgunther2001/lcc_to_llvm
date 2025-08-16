use std::collections::HashMap;
use std::fs::File;

use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::types::{IntType, PointerType};
use inkwell::values::{FunctionValue, GlobalValue, PointerValue};
use inkwell::{context::Context, module::Linkage};
use inkwell::module::Module;
use inkwell::{AddressSpace, OptimizationLevel};
use crate::llvm_ir_gen::microcode_routine_def::microcode_lib::initialize_microcode_wrappers;
use crate::parser::parser_defns::{AsmLine, RegOrImmOperand};

pub struct IRGenUtil<'ctx> {
    pub context: &'ctx Context,
    pub module:  Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub i16_type:  IntType<'ctx>, 
    pub i16p_type: PointerType<'ctx>,
    pub i32_type:  IntType<'ctx>,
    pub register_table: GlobalValue<'ctx>,
    pub microcode_function_map: HashMap<String, FunctionValue<'ctx>>
}

impl<'ctx> IRGenUtil<'ctx> {
    pub fn new(context: &'ctx Context, name: &str) -> Self {
        let module = context.create_module(name);
        let builder = context.create_builder();
        let i16_type = context.i16_type();
        let i16p_type = i16_type.ptr_type(AddressSpace::default());
        let i32_type = context.i32_type();
        let microcode_function_map = HashMap::new();

        let array_type = i16_type.array_type(8);
        let zero_initializer = i16_type.const_int(0, false);
        let initializer = i16_type.const_array(&vec![zero_initializer; 8]);
        let register_table = module.add_global(array_type, None, "register_table");
        register_table.set_initializer(&initializer);
        register_table.set_linkage(Linkage::Internal);

        Self { context, module, builder, i16_type, i16p_type, i32_type, microcode_function_map , register_table }
    }
pub fn get_register_ptr_const(&self, index: u32) -> PointerValue<'ctx> {
    let zero = self.context.i32_type().const_zero();
    let idx = self.context.i32_type().const_int(index as u64, false);

    unsafe {
        self.builder
            .build_gep(
                self.i16_type,
                self.builder.build_gep(
                    self.i16_type.array_type(8),
                    self.register_table.as_pointer_value(),
                    &[zero],
                    "reg_base",
                ).unwrap(),
                &[idx],
                "reg"
            )
            .unwrap()
    }
}
    pub fn cache_microcode(&mut self, name: String, function: FunctionValue<'ctx>) {
        self.microcode_function_map.insert(name, function);
    }

    pub fn extract_microcode_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        match self.microcode_function_map.get(name) {
            Some(function) => Some(*function),
            None => None
        }
    }
}

pub fn populate_module(lines: Vec<AsmLine>, ir: &mut IRGenUtil) -> () {
    // add printf functionality, etc....
    add_c_std_lib_funcs(ir);

    let entry_function_type = ir.i32_type.fn_type(&[], false);
    let entry_function = ir.module.add_function("main", entry_function_type, None);
    let entry_block = ir.context.append_basic_block(entry_function, "entry");
    ir.builder.position_at_end(entry_block);
    /* 
    let registers: Vec<inkwell::values::PointerValue<'_>> = (0..8)
        .map(|i| {
            let ptr = ir.builder.build_alloca(ir.i16_type, &format!("reg{i}")).unwrap();
            ir.builder.build_store(ptr, ir.i16_type.const_zero()).unwrap();
            ptr
        })
        .collect();
     */
        /* 
        .map(|_| ir.builder.build_alloca(ir.i16_type, "reg").unwrap())
        .collect();
        */

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
            AsmLine::Dout { dr } => {
                ir.builder.build_call(ir.extract_microcode_function("dout").unwrap(), &[ir.get_register_ptr_const(dr as u32).into()],"").unwrap();
            }
            AsmLine::Nl { } => {
                ir.builder.build_call(ir.extract_microcode_function("nl").unwrap(), &[],"").unwrap();
            }
            _ => todo!()
        }
    }
}




fn add_c_std_lib_funcs(ir: &mut IRGenUtil) -> () {
    let i8p_type = ir.context.i8_type().ptr_type(AddressSpace::default());
    let printf_type = ir.context.i32_type().fn_type(&[i8p_type.into()], true);
    let printf_fn = ir.module.add_function("printf", printf_type, None);
    printf_fn.get_nth_param(0).unwrap().into_pointer_value().set_name("format");
    ir.cache_microcode("printf".to_string(), printf_fn);
}

pub fn write_module(ir: &IRGenUtil, output_path: &str) -> () {
    let _ = ir.module.print_to_file(output_path);
}

pub fn set_triple(ir: &mut IRGenUtil) -> () {
    let target_triple = inkwell::targets::TargetMachine::get_default_triple();
    ir.module.set_triple(&target_triple);
}