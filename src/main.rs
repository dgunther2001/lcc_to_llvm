use std::{env, fs::File, io::BufReader};

use inkwell::context::Context;

use crate::{llvm_ir_gen::ir_gen::{populate_module, set_triple, write_module, IRGenUtil}, parser::{asm_line_gen::generate_parse_tree, parser_defns::AsmLine}, tokenizer::tokens::Token};


mod definitions;
mod tokenizer;
mod parser;
mod llvm_ir_gen;



fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(index) = args.iter().position(|arg| arg == "--file") {
        if let Some(file_path) = args.get(index + 1) {
            let file = File::open(file_path).expect("Failed to open input file");
            let tokens: Vec<Vec<Token>> = tokenizer::lexer::tokenize_file(BufReader::new(file));
            println!("Tokens: {:?}", tokens);
            let parse_tree: Vec<AsmLine> = generate_parse_tree(&tokens);
            println!("Parse Tree: {:?}", parse_tree);
            let context = Context::create();
            let mut ir_gen_utility = IRGenUtil::new(&context, "lcc_module");
            set_triple(&mut ir_gen_utility);
            populate_module(parse_tree, &mut ir_gen_utility);
            write_module(&ir_gen_utility, "lcc_module.ll");

        } else {
            eprintln!("Expected input value after --file arg")
        }
    } else {
        eprintln!("--file is a required argument")
    }

    //make_basic_ir();
}

#[test]
fn basic_test() {
    println!("Opcode: {:?}", definitions::op_defn::match_opcode("add"));
    println!("Opcode: {:?}", definitions::op_defn::match_opcode("ad"));
    println!("Register: {:?}", definitions::reg_defn::match_string_to_reg("r0"));
    println!("Register: {:?}", definitions::reg_defn::match_string_to_reg("lr"));
    println!("Register: {:?}", definitions::reg_defn::match_string_to_reg("t"));
}

#[test]
fn tokenize_some_stuff() {
    //let tokens1 = tokenizer::lexer::tokenize_line("ld r1 86745");
    //let tokens2 = tokenizer::lexer::tokenize_line("add r0 r1 r5");
    //println!("Tokenized Output: {:?}", tokens1);
    //println!("Tokenized Output: {:?}", tokens2);
}
