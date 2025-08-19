#[cfg(test)]
mod logic_micro_tests {
    use crate::smoke_test_case;

    smoke_test_case!(single_and_sr2,
                     r#"add r0 r0 6
                        add r1 r1 9
                        and r0 r0 r1"#, 0);
    
    smoke_test_case!(single_and_imm5,
                     r#"add r0 r0 3
                        and r0 r0 14"#, 2);

    smoke_test_case!(single_or, 
                     r#"add r0 r0 6
                        add r1 r1 9 
                        or r0 r1"#, 15);

    smoke_test_case!(single_xor, 
                     r#"add r1 r1 -1
                        xor r0 r1"#, -1);
        
    smoke_test_case!(single_not, 
                     r#"add r1 r1 -5
                        not r0 r1"#, 4);

    
    // slightly more edge casey
    smoke_test_case!(and_negative_one, 
                     r#"add r0 r0 5
                        and r0 r0 -1"#, 5);

    smoke_test_case!(and_with_zero, 
                     r#"add r0 r0 19
                        and r0 r0 0"#, 0);

    smoke_test_case!(self_xor, 
                     r#"add r0 r0 15
                        xor r0 r0"#, 0);    

    smoke_test_case!(double_not, 
                     r#"add r0 r0 12
                        not r0 r0
                        not r0 r0"#, 12);                                       

}