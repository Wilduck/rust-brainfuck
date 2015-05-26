use std::char;

use super::tokenize;


pub fn interpret(tokens: Vec<tokenize::Operator>, input: String) -> [u8; 30000] {
    let instructions = tokens.len();
    let mut memory: [u8; 30000] = [0; 30000];
    let input_bytes = input.as_bytes();
    let mut code_loc: usize = 0;
    let mut input_loc: usize = 0;
    let mut loop_loc: Vec<usize> = Vec::new();
    let mut data_loc: usize = 0;
    while code_loc < instructions {
        let ref operator = tokens[code_loc];
        match *operator {
            tokenize::Operator::IncCell => {
                memory[data_loc] += 1;
                code_loc += 1;
            }
            tokenize::Operator::DecCell => {
                memory[data_loc] -= 1;
                code_loc += 1;
            }
            tokenize::Operator::IncPtr => {
                data_loc += 1;
                code_loc += 1;
            }
            tokenize::Operator::DecPtr => {
                data_loc -= 1;
                code_loc += 1;
            }
            tokenize::Operator::Print => {
                let character = char::from_u32(memory[data_loc] as u32);
                match character {
                    Some(c) => print!("{}", c),
                    None => print!("<??>"),
                }
                code_loc += 1;
            }
            tokenize::Operator::Read => {
                memory[data_loc] = input_bytes[input_loc];
                input_loc += 1;
                code_loc += 1;
            }
            tokenize::Operator::JumpZero => {
                let mut depth_tracker = 0;
                if memory[data_loc] == 0 {
                    while (tokens[code_loc] != tokenize::Operator::Loop) 
                        | (depth_tracker != 0) {
                        if tokens[code_loc] == tokenize::Operator::JumpZero {
                            depth_tracker += 1;
                        } else if tokens[code_loc] == tokenize::Operator::Loop {
                            depth_tracker -= 1;
                        }
                        code_loc += 1;
                    }
                } else {
                    loop_loc.push(code_loc);
                    code_loc += 1;
                }
            }
            tokenize::Operator::Loop => {
                if memory[data_loc] == 0 {
                    loop_loc.pop();
                    code_loc += 1;
                } else {
                    code_loc = match loop_loc.pop() {
                        Some(loc) => loc,
                        None => code_loc + 1,
                    }
                }
            }
        }
    }
    memory
}
