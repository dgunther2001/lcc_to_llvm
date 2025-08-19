use std::io::BufRead;

use crate::{definitions::{op_defn::match_opcode, reg_defn::match_string_to_reg}, tokenizer::tokens::Token};


pub fn tokenize_file(file_reader: impl BufRead) -> Vec<Vec<Token>> {
    let mut tokens: Vec<Vec<Token>> = Vec::new();

    for reader_line in file_reader.lines() {
        if let Ok(line) = reader_line {
            tokenize_line(&line, &mut tokens);
        }
    }
    tokens
}

pub fn tokenize_text(text: &str) -> Vec<Vec<Token>> {
    let mut tokens: Vec<Vec<Token>> = Vec::new();
    for line in text.split("\n") {
        tokenize_line(line, &mut tokens);
    }
    tokens
}

pub fn tokenize_line(line:  &str, tokens: &mut Vec<Vec<Token>>) -> () {
    let mut new_line_tokens: Vec<Token> = Vec::new();


    // need to verify the ordering of things here??? probably not....
    // maybe just want to enforce that we declare labels at the beginning of the line
    // gotta figure out strings....
    for word in line.trim().split_whitespace() {
        if word.starts_with(';') {
            break;
        } else if let Some(opcode) = match_opcode(word) {
            new_line_tokens.push(Token::Opcode(opcode));
        } else if let Some(reg) = match_string_to_reg(word) {
            new_line_tokens.push(Token::Register(reg));
        } else if word.starts_with('.') {
            new_line_tokens.push(Token::Directive(word.to_string()));
        } else if word.ends_with(':') {
            new_line_tokens.push(Token::Label(word.to_string()));
        } else if let Ok(immediate) = word.parse::<i16>() {
            new_line_tokens.push(Token::Numeric(immediate));
        } else {
            new_line_tokens.push(Token::Identifier(word.to_string()));
        }
    }

    if new_line_tokens.len() > 0 {
        tokens.push(new_line_tokens);
    }
}