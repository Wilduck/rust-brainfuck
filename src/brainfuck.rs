use std::char;
use std::env;

extern crate brainfuck;
use brainfuck::argparse;

#[derive(PartialEq)]
enum Operator {
    IncCell,     // +
    DecCell,     // -
    IncPtr,      // >
    DecPtr,      // <
    Print,       // .
    Read,        // ,
    JumpZero,    // [
    Loop,        // ]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = argparse::create_options();
    let parsed_args_result = argparse::parse_args(opts, args);
    match parsed_args_result {
        Err(e) => {
            println!("Invalid Arguments: {}", e);
        }
        Ok(parsed_args) => {
            if parsed_args.verbose {
                println!("Running program: \n{}\n", parsed_args.source);
                println!("With input: \n{}\n", parsed_args.input);
                println!("Output:");
            }
            let tokens = tokenize(parsed_args.source);
            parse(tokens, parsed_args.input);
            println!("");
        }
    }
}


fn tokenize(source: String) -> Vec<Operator> {
    let mut tokens: Vec<Operator> = Vec::new();
    for c in source.chars() {
        let o: Option<Operator> = match c {
            '+' => Some(Operator::IncCell),
            '-' => Some(Operator::DecCell),
            '>' => Some(Operator::IncPtr),
            '<' => Some(Operator::DecPtr),
            '.' => Some(Operator::Print),
            ',' => Some(Operator::Read),
            '[' => Some(Operator::JumpZero),
            ']' => Some(Operator::Loop),
            _ => None
        };
        match o {
            Some(operator) => tokens.push(operator),
            None => {}
        };
    };
    tokens
}

fn parse(tokens: Vec<Operator>, input: String) -> [u8; 30000] {
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
            Operator::IncCell => {
                memory[data_loc] += 1;
                code_loc += 1;
            }
            Operator::DecCell => {
                memory[data_loc] -= 1;
                code_loc += 1;
            }
            Operator::IncPtr => {
                data_loc += 1;
                code_loc += 1;
            }
            Operator::DecPtr => {
                data_loc -= 1;
                code_loc += 1;
            }
            Operator::Print => {
                let character = char::from_u32(memory[data_loc] as u32);
                match character {
                    Some(c) => print!("{}", c),
                    None => print!("<??>"),
                }
                code_loc += 1;
            }
            Operator::Read => {
                memory[data_loc] = input_bytes[input_loc];
                input_loc += 1;
                code_loc += 1;
            }
            Operator::JumpZero => {
                let mut depth_tracker = 0;
                if memory[data_loc] == 0 {
                    while (tokens[code_loc] != Operator::Loop) | (depth_tracker != 0) {
                        if tokens[code_loc] == Operator::JumpZero {
                            depth_tracker += 1;
                        } else if tokens[code_loc] == Operator::Loop {
                            depth_tracker -= 1;
                        }
                        code_loc += 1;
                    }
                } else {
                    loop_loc.push(code_loc);
                    code_loc += 1;
                }
            }
            Operator::Loop => {
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
