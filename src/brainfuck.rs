use std::env;

extern crate brainfuck;
use brainfuck::argument;
use brainfuck::interpreter;
use brainfuck::tokenize;


fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = argument::create_options();
    let parsed_args_result = argument::parse_args(opts, args);
    match parsed_args_result {
        Err(e) => {
            println!("Invalid Arguments: {}", e);
        }
        Ok(parsed_args) => {
            if parsed_args.verbose == 2 {
                println!("Running program: \n{}\n", parsed_args.source);
                println!("With input: \n{}\n", parsed_args.input);
                println!("Output:");
            }
            let tokens = tokenize::tokenize(parsed_args.source);
            let final_state = interpreter::interpret(tokens, parsed_args.input);
            if parsed_args.verbose >= 1 {
                match final_state.exit {
                    Some(err) => { println!("Errors: \n{}", err) }
                    None => { }
                }
            }
            if parsed_args.verbose > 0 {
                println!("");
            }
        }
    }
}
