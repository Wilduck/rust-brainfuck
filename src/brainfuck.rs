extern crate getopts;
use getopts::Options;

use std::env;
use std::char;
use std::io;
use std::io::prelude::*;
use std::str;

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

struct ParsedArgs {
    source: String,
    input: String,
    verbose: bool,
    failure: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = create_options();
    let parsed_args = parse_args(opts, args);
    match parsed_args.failure {
        Some(failure_string) => {
            println!("Invalid Arguments: {}", failure_string);
        }
        None => {
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

fn parse_args(opts: Options, args: Vec<String>) -> ParsedArgs {
    // parse out everything we're interested in handling
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let verbose = matches.opt_present("v");
    let input_flag = matches.opt_str("i");

    // Get the source code string
    let source = match matches.opt_str("s") {
        Some(s) => s,
        None => "".to_string(),
    };

    // Get the input string, either from an arguement or stdin.
    let input = match input_flag {
        Some(i) => i,
        None => {
            let stdin = io::stdin();
            let mut buf: Vec<u8> = Vec::new();
            let result = stdin.lock().read_to_end(&mut buf);
            match result {
                Ok(_) => str::from_utf8(&buf).unwrap().to_string(),
                Err(_) => "".to_string(),
            }
        }
    };

    ParsedArgs {
        source: source,
        input: input,
        verbose: verbose,
        failure: None,
    }
}

fn create_options() -> (Options) {
    let mut opts = Options::new();
    opts.optflag("v", "verbose", "Print More Info");
    opts.optopt("s", "source", "Source string", "SOURCE");
    opts.optopt("i", "input", "Input string", "INPUT");
    opts
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
                if memory[data_loc] == 0 {
                    while tokens[code_loc] != Operator::Loop {
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
