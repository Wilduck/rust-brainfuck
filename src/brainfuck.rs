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
            let tokens = tokenize::tokenize(&parsed_args.source);
            let final_state = interpreter::interpret(tokens, &parsed_args.input);
            let output = final_state.output.iter().cloned().collect::<String>();
            if (parsed_args.verbose == 0) | (parsed_args.verbose == 1) {
                println!("{}", output)
            }
            if parsed_args.verbose == 3 {
                super_verbose(&final_state, &output, &parsed_args.input);
            }
            if parsed_args.verbose == 2 {
                println!("Running program: \n{}\n", parsed_args.source);
                println!("With input: \n{}\n", parsed_args.input);
                println!("Output: \n{}", output);
            }
            if (parsed_args.verbose == 1) | (parsed_args.verbose == 2) {
                match final_state.exit {
                    Some(err) => { println!("\nErrors:\n{}", err) }
                    None => { }
                }
            }
            if parsed_args.verbose > 0 {
                println!("");
            }
        }
    }
}

fn super_verbose(state: &interpreter::State, output: &String, input: &String) {
    println!("Input:\n{}\n", input);
    println!("Output:\n{}\n", output);
    println!("\nState\n-----");
    println!("Input Location:       {}", state.input_loc);
    println!("Final Cell [0-{}]: {}", state.memory.len(), state.data_loc);
    println!("Memory around final cell:\n");
    println!("{}", memory_view(&state.memory, state.data_loc));
}

fn memory_view(memory: &Vec<u8>, location: usize) -> String{
    let memsize = memory.len();
    let top_bound = memsize - 10;
    let mut output = String::new();
    let mut indicator = String::new();
    let start = match location {
        0 ... 10 => 0,
        _ => location - 10,
    };
    let end = match location {
        x if x > top_bound => memsize,
        _ => location + 10,
    };
    for i in &memory[start..end] {
        output.push_str(format!("{:02x} ", i).as_ref());
    };
    for i in start..end {
        let s = if i == location {"^^ "} else {"   "};
        indicator.push_str(s);
    }
    format!("{}\n{}", output, indicator)
}