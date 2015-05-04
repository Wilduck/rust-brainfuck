extern crate getopts;
use getopts::Options;
use std::env;

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
    let opts = create_options();
    let (source, input) = parse_args(opts, args);
    println!("Running program: \n{}\n", source);
    println!("With input: \n{}\n", input);
    let tokens = tokenize(source);
    let blah = parse(tokens, input);
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

fn parse(tokens: Vec<Operator>, input: String) -> Vec<i8> {
    let x = std::iter::repeat(0).take(30000).collect::<Vec<i8>>();
    x
}