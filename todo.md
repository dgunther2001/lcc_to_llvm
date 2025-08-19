[] Add global format string table...
[] AND, OR, NOT, XOR
[] Make sure that 0 extension is working correctly on negatives wrt the immediate 5
[] nczv flags for ADD; AND; NOT; MUL; DIV; OR; XOR; SUB

[] =>     smoke_test_case!(and_negative_one, 
                     r#"add r0 r0 5
                        and r0 -1"#, 5); => WHY DID THIS WORK?????

[] Ongoing immediate 5 bugs (add sometimes allows me to input massive numbers???)                        