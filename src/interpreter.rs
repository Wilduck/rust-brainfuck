use std::char;
use std::iter;

use super::tokenize;


struct State {
    memory: Vec<u8>,
    code_loc: usize,
    data_loc: usize,
    input_loc: usize,
    loop_depth: usize,
    loop_loc: Vec<usize>,
}


pub fn interpret(tokens: Vec<tokenize::Operator>, input: String) -> Vec<u8> {
    let mut state = State {
        memory: iter::repeat(0).take(30000).collect::<Vec<u8>>(),
        code_loc: 0,
        data_loc: 0,
        input_loc: 0,
        loop_depth: 0,
        loop_loc: Vec::new()
    };
    let instructions = tokens.len();
    let input_bytes = input.as_bytes();
    while state.code_loc < instructions {
        let ref operator = tokens[state.code_loc];
        match *operator {
            tokenize::Operator::IncCell => {
                inc_cell(&mut state);
            }
            tokenize::Operator::DecCell => {
                state.memory[state.data_loc] -= 1;
                state.code_loc += 1;
            }
            tokenize::Operator::IncPtr => {
                state.data_loc += 1;
                state.code_loc += 1;
            }
            tokenize::Operator::DecPtr => {
                state.data_loc -= 1;
                state.code_loc += 1;
            }
            tokenize::Operator::Print => {
                let character = char::from_u32(state.memory[state.data_loc] as u32);
                match character {
                    Some(c) => print!("{}", c),
                    None => print!("<??>"),
                }
                state.code_loc += 1;
            }
            tokenize::Operator::Read => {
                state.memory[state.data_loc] = input_bytes[state.input_loc];
                state.input_loc += 1;
                state.code_loc += 1;
            }
            tokenize::Operator::JumpZero => {
                state.loop_depth = 0;
                if state.memory[state.data_loc] == 0 {
                    while (tokens[state.code_loc] != tokenize::Operator::Loop)
                        | (state.loop_depth != 0) {
                        if tokens[state.code_loc] == tokenize::Operator::JumpZero {
                            state.loop_depth += 1;
                        } else if tokens[state.code_loc] == tokenize::Operator::Loop {
                            state.loop_depth -= 1;
                        }
                        state.code_loc += 1;
                    }
                } else {
                    state.loop_loc.push(state.code_loc);
                    state.code_loc += 1;
                }
            }
            tokenize::Operator::Loop => {
                if state.memory[state.data_loc] == 0 {
                    state.loop_loc.pop();
                    state.code_loc += 1;
                } else {
                    state.code_loc = match state.loop_loc.pop() {
                        Some(loc) => loc,
                        None => state.code_loc + 1,
                    }
                }
            }
        }
    }
    state.memory
}


fn inc_cell(state: &mut State) {
    state.memory[state.data_loc] += 1;
    state.code_loc += 1;
}
