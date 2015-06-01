use std::char;
use std::iter;

use super::tokenize;


pub struct State {
    pub memory: Vec<u8>,
    pub code_loc: usize,
    pub data_loc: usize,
    pub input_loc: usize,
    pub loop_depth: usize,
    pub loop_loc: Vec<usize>,
    pub exit: Option<String>,
}


pub fn interpret(tokens: Vec<tokenize::Operator>, input: String) -> State {
    let mut state = State {
        memory: iter::repeat(0).take(30000).collect::<Vec<u8>>(),
        code_loc: 0,
        data_loc: 0,
        input_loc: 0,
        loop_depth: 0,
        loop_loc: Vec::new(),
        exit: None,
    };
    let instructions = tokens.len();
    let input_bytes = input.as_bytes();
    while (state.code_loc < instructions) & (state.exit == None) {
        let ref operator = tokens[state.code_loc];
        match *operator {
            tokenize::Operator::IncCell  => inc_cell(&mut state),
            tokenize::Operator::DecCell  => dec_cell(&mut state),
            tokenize::Operator::IncPtr   => inc_ptr(&mut state),
            tokenize::Operator::DecPtr   => dec_ptr(&mut state),
            tokenize::Operator::Print    => print_cell(&mut state),
            tokenize::Operator::Read     => read_input(&mut state, input_bytes),
            tokenize::Operator::JumpZero => jump_zero(&mut state, &tokens),
            tokenize::Operator::Loop     => loop_jump(&mut state),
        }
    }
    state
}

fn inc_cell(state: &mut State) {
    let x = state.memory[state.data_loc].checked_add(1);
    match x {
        Some(val) => {
            state.memory[state.data_loc] = val;
        }
        None => {
            state.exit = Some("Integer Overflow".to_string());
        }
    };
    state.code_loc += 1
}

fn dec_cell(state: &mut State) {
    let x = state.memory[state.data_loc].checked_sub(1);
    match x {
        Some(val) => {
            state.memory[state.data_loc] = val;
        }
        None => {
            state.exit = Some("Integer Underflow".to_string());
        }
    };
    state.code_loc += 1;
}

fn inc_ptr(state: &mut State) {
    if state.data_loc == state.memory.len() - 1 {
        state.exit = Some("Out of bounds (overflow)".to_string());
    } else {
        state.data_loc += 1;
        state.code_loc += 1;
    }
}

fn dec_ptr(state: &mut State) {
    if state.data_loc == 0 {
        state.exit = Some("Out of bounds (underflow)".to_string());
    } else {
        state.data_loc -= 1;
        state.code_loc += 1;
    }
}

fn print_cell(state: &mut State) {
    let character = char::from_u32(state.memory[state.data_loc] as u32);
    match character {
        Some(c) => print!("{}", c),
        None => print!("<??>"),
    }
    state.code_loc += 1;
}

fn read_input(state: &mut State, input_bytes: &[u8]) {
    if state.input_loc < input_bytes.len() {
        state.memory[state.data_loc] = input_bytes[state.input_loc];
    } else {
        state.memory[state.data_loc] = 0;
    }
    state.input_loc += 1;
    state.code_loc += 1;
}

fn jump_zero(state: &mut State, tokens: &Vec<tokenize::Operator>) {
    state.loop_depth = 1;
    let memory_value = state.memory[state.data_loc];
    if memory_value == 0 {
        // Jump
        while (tokens[state.code_loc] != tokenize::Operator::Loop) | (state.loop_depth != 0) {
            state.code_loc += 1;
            if tokens[state.code_loc] == tokenize::Operator::JumpZero {
                state.loop_depth += 1;
            } else if tokens[state.code_loc] == tokenize::Operator::Loop {
                state.loop_depth -= 1;
            }
        }
    } else {
        // Don't jump
        state.loop_loc.push(state.code_loc);
        state.code_loc += 1;
    }
}

fn loop_jump(state: &mut State) {
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
