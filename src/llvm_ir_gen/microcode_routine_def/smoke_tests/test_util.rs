

#[cfg(test)]
pub mod test_util_helpers {
    use crate::{llvm_ir_gen::{ir_gen::{populate_module, set_triple}, ir_gen_structures::IRGenUtil}, parser::{asm_line_gen::generate_parse_tree, parser_defns::AsmLine}, tokenizer::lexer::tokenize_text};


    fn test_main_and_ret_r0(ir: &mut IRGenUtil) -> () {
        use crate::llvm_ir_gen::microcode_routine_def::load_i16;

        let test_fn = ir.module.add_function("test", ir.context.i16_type().fn_type(&[], false), None);
        let test_entry = ir.context.append_basic_block(test_fn, "entry");
        ir.builder.position_at_end(test_entry);

        let main_fn = ir.module.get_function("main").expect("main not emitted");
        ir.builder.build_call(main_fn, &[], "call_main").unwrap();

        let r0_as_i16 = load_i16(ir, ir.get_register_ptr_const(0), "r1_val");
        ir.builder.build_return(Some(&r0_as_i16)).unwrap();
    }

    fn run_test_exec_engine(ir: &mut IRGenUtil) -> i16 {
        let exec_engine = ir.module.create_jit_execution_engine(inkwell::OptimizationLevel::None).unwrap();
        type ReturnI16FuncSig = unsafe extern "C" fn() -> i16;
        let test_func = exec_engine.get_function_address("test").unwrap();
        let run: ReturnI16FuncSig = unsafe { std::mem::transmute(test_func) };
        return unsafe { run() };
    }

    pub fn create_main_run_test_ret_r0(input_text: &str) -> i16 {
        let tokens = tokenize_text(input_text);
        let parse_tree: Vec<AsmLine> = generate_parse_tree(&tokens);

        let context = inkwell::context::Context::create();
        let mut ir = IRGenUtil::new(&context, "lcc_module");
        set_triple(&mut ir);
        populate_module(parse_tree, &mut ir);


        test_main_and_ret_r0(&mut ir);

        //let ir_txt = ir.module.print_to_string().to_string();
        run_test_exec_engine(&mut ir)
    }

    #[macro_export]
    macro_rules! smoke_test_case {
        ($smoke_test_name:ident, $input_lcc:expr, $expected_r0_val:expr) => {
            #[test]
            fn $smoke_test_name() {
                let r0_val = $crate::llvm_ir_gen::microcode_routine_def::smoke_tests::test_util::test_util_helpers::create_main_run_test_ret_r0($input_lcc);
                assert_eq!(r0_val, $expected_r0_val)
            }
        }
    }
}


