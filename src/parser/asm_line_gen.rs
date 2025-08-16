use crate::{definitions::{op_defn::Opcode}, parser::parser_defns::{AsmLine, RegOrImmOperand}, tokenizer::tokens::Token};



pub fn generate_parse_tree(tokens: &Vec<Vec<Token>>) -> Vec<AsmLine> {
    let mut parsed_lines:  Vec<AsmLine> = Vec::new();

    for line in tokens {
        if let Some(first_token) = line.get(0) {
            match first_token {
                Token::Opcode(_) => {
                    if let Some(asm_line) = parse_opcodes(line) {
                        parsed_lines.push(asm_line)
                    }
                }
                Token::Label(lbl) => {

                }
                _ => eprintln!("Tokenization failed on: {:?}", line)
            }
        }
    }

    parsed_lines
}

fn parse_opcodes(tokens: &Vec<Token>) -> Option<AsmLine> {
    match tokens.get(0) {
        Some(Token::Opcode(Opcode::Add)) | Some(Token::Opcode(Opcode::Sub)) | Some(Token::Opcode(Opcode::Mul)) | Some(Token::Opcode(Opcode::Div)) => parse_arithmetic(tokens),
        Some(Token::Opcode(Opcode::Dout)) | Some(Token::Opcode(Opcode::Nl)) | Some(Token::Opcode(Opcode::Din))  => parse_io(tokens),
        //Ld  => (),
        //St  => (),
        _            => None
    }
}

fn parse_arithmetic(tokens: &Vec<Token>) -> Option<AsmLine> {
    // grab the dr and sr 1
    match (&tokens.get(0)?, &tokens.get(1)?, &tokens.get(2), &tokens.get(3)) {
        (
            Token::Opcode(opc @ (Opcode::Add | Opcode::Sub)),
            Token::Register(dr),
            Some(Token::Register(sr1)),
            Some(Token::Register(sr2))
        ) => {
            let instruction_type = match opc {
                Opcode::Add => AsmLine::Add {
                    dr: *dr,
                    sr1: *sr1,
                    sr2: RegOrImmOperand::Register(*sr2),
                },
                Opcode::Sub => AsmLine::Sub {
                    dr: *dr,
                    sr1: *sr1,
                    sr2: RegOrImmOperand::Register(*sr2),
                },
                _ => unreachable!()                
            };
            Some(instruction_type)
        }
        (
            Token::Opcode(opc @ (Opcode::Add | Opcode::Sub)),
            Token::Register(dr),
            Some(Token::Register(sr1)),
            Some(Token::Numeric(imm5))    
        ) => {
            if (*imm5 < 32 && *imm5 >=0) {
                let instruction_type = match opc {
                    Opcode::Add => AsmLine::Add {
                        dr: *dr,
                        sr1: *sr1,
                        sr2: RegOrImmOperand::Immediate(*imm5),
                    },
                    Opcode::Sub => AsmLine::Sub {
                        dr: *dr,
                        sr1: *sr1,
                        sr2: RegOrImmOperand::Immediate(*imm5),
                    },
                    _ => unreachable!()
                };
                Some(instruction_type)
            } else {
                eprintln!("Immediate 5 for tokens: {:?} not within 5 bit range", tokens);
                return None;
            }
        }
        (
            Token::Opcode(opc @ (Opcode::Mul | Opcode::Div)),
            Token::Register(dr),
            Some(Token::Register(sr)),
            None
        ) => {
            let instruction_type = match opc {
                Opcode::Mul => AsmLine::Mul { dr: *dr, sr: *sr },
                Opcode::Div => AsmLine::Div { dr: *dr, sr: *sr },
                _           => unreachable!()
            };
            Some(instruction_type)
        }

        _ => None
    }
}

fn parse_io(tokens: &Vec<Token>) -> Option<AsmLine> {
    match tokens.as_slice() {
        [Token::Opcode(Opcode::Dout), Token::Register(reg)] => {
            Some(AsmLine::Dout { dr: *reg })
        }
        [Token::Opcode(Opcode::Nl)] => {
            Some(AsmLine::Nl {  })
        }
        [Token::Opcode(Opcode::Din), Token::Register(reg)] => {
            Some(AsmLine::Din { dr: *reg })
        }
        _ => None
    }
}