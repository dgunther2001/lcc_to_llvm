use inkwell::{module::Linkage, types::{BasicMetadataTypeEnum}, values::{FunctionValue, IntValue, PointerValue}};

use ir_gen_structures::IRGenUtil;
use crate::llvm_ir_gen::ir_gen_structures;

pub enum MicrocodeCallSignature {
    DrSrSr,
    DrSrImm5,
    DrSr,
    Dr,
    Sr,
    Void
    // TBD ADD MORE!!!
}

pub struct MicrodeArguments<'ctx> {
    pub(crate) dr:              Option<PointerValue<'ctx>>,
    pub(crate) sr:              Option<PointerValue<'ctx>>,
    pub(crate) sr1:             Option<PointerValue<'ctx>>,
    pub(crate) sr2:             Option<PointerValue<'ctx>>,
    pub(crate) imm_or_offset:   Option<IntValue<'ctx>>,
}


pub fn declare_microcode<'ctx>(ir: &mut IRGenUtil<'ctx>, name: &str, call_sig: MicrocodeCallSignature) -> (FunctionValue<'ctx>, MicrodeArguments<'ctx>) {
    let i16  = ir.i16_type;
    let i16p = ir.i16p_type;

    let arg_parameters: Vec<BasicMetadataTypeEnum> = match call_sig {
        MicrocodeCallSignature::DrSrSr      => vec![i16p.into(), i16p.into(), i16p.into()],
        MicrocodeCallSignature::DrSrImm5    => vec![i16p.into(), i16p.into(), i16.into()],
        MicrocodeCallSignature::DrSr        => vec![i16p.into(), i16p.into()],
        MicrocodeCallSignature::Dr          => vec![i16p.into()],
        MicrocodeCallSignature::Sr          => vec![i16p.into()],
        MicrocodeCallSignature::Void        => vec![],
    };

    let fn_type = ir.context.void_type().fn_type(&arg_parameters, false);
    let function = ir.module.add_function(name, fn_type, Some(Linkage::Internal));
    let function_entry_block = ir.context.append_basic_block(function, name);
    ir.builder.position_at_end(function_entry_block);

    let get_parameter = |n| function.get_nth_param(n).unwrap();

    let args = match call_sig {
        MicrocodeCallSignature::DrSrSr   => {
                    let dr      = get_parameter(0).into_pointer_value(); dr.set_name("dr");
                    let sr1     = get_parameter(1).into_pointer_value(); sr1.set_name("sr1");
                    let sr2     = get_parameter(2).into_pointer_value(); sr2.set_name("sr2");
                    MicrodeArguments { dr: Some(dr), sr: None, sr1: Some(sr1), sr2: Some(sr2), imm_or_offset: None }

                },
        MicrocodeCallSignature::DrSrImm5 => {
                    let dr      = get_parameter(0).into_pointer_value(); dr.set_name("dr");
                    let sr      = get_parameter(1).into_pointer_value(); sr.set_name("sr");
                    let imm5        = get_parameter(2).into_int_value();     imm5.set_name("imm5");
                    MicrodeArguments { dr: Some(dr), sr: Some(sr), sr1: None, sr2: None, imm_or_offset: Some(imm5) }         
                },
        MicrocodeCallSignature::DrSr     => {
                let dr      = get_parameter(0).into_pointer_value(); dr.set_name("dr");
                let sr      = get_parameter(1).into_pointer_value(); sr.set_name("sr");
                MicrodeArguments { dr: Some(dr), sr: Some(sr), sr1: None, sr2: None, imm_or_offset: None } 
            },
        MicrocodeCallSignature::Dr       => {
                let dr      = get_parameter(0).into_pointer_value(); dr.set_name("dr");
                MicrodeArguments { dr: Some(dr), sr: None, sr1: None, sr2: None, imm_or_offset: None }
        },
        MicrocodeCallSignature::Sr       => {
                let sr      = get_parameter(0).into_pointer_value(); sr.set_name("sr");
                MicrodeArguments { dr: None, sr: Some(sr), sr1: None, sr2: None, imm_or_offset: None }
            },
        MicrocodeCallSignature::Void     => {
                MicrodeArguments { dr: None, sr: None, sr1: None, sr2: None, imm_or_offset: None }
            },
    };


    ir.cache_microcode(name.to_string(), function);
    (function, args)
}

pub fn load_i16<'ctx>(ir: &IRGenUtil<'ctx>, loc_to_load: PointerValue<'ctx>, name: &str) ->  IntValue<'ctx> {
    return ir.builder.build_load(ir.i16_type, loc_to_load, name).unwrap().into_int_value();
}

//ir.builder.build_store(dr, result).unwrap();
pub fn store_i16<'ctx>(ir: &IRGenUtil<'ctx>, loc_to_store: PointerValue<'ctx>, value_to_store: IntValue<'ctx>) -> () {
    let _ = ir.builder.build_store(loc_to_store, value_to_store);
}

pub fn create_void_return<'ctx>(ir: &IRGenUtil<'ctx>) -> () {
    let _ = ir.builder.build_return(None);
}