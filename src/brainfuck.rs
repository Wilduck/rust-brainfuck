extern crate getopts;
use getopts::Options;
use std::env;
use std::char;

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

enum Status {
    Success,
    Failure(String),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = create_options();
    let (source, input) = parse_args(opts, args);
    println!("Running program: \n{}\n", source);
    println!("With input: \n{}\n", input);
    let tokens = tokenize(source);
    println!("Output:");
    let blah = parse(tokens, input);
    println!("");
    println!("{}", blah.len());
}

fn parse_args(opts: Options, args: Vec<String>) -> (String, String) {
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let source = match matches.opt_str("s") {
        Some(s) => s,
        None => "".to_string(),
    };
    let input = match matches.opt_str("i") {
        Some(i) => i,
        None => "".to_string(),
    };
    (source, input)
}

fn create_options() -> (Options) {
    let mut opts = Options::new();
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

fn printable_op(op: Operator) -> char {
    match op {
        Operator::IncCell => '+',
        Operator::DecCell => '-',
        Operator::IncPtr => '>',
        Operator::DecPtr => '<',
        Operator::Print => '.',
        Operator::Read => ',',
        Operator::JumpZero => '[',
        Operator::Loop => ']',
    }
}