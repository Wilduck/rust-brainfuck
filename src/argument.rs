use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

extern crate getopts;
use self::getopts::Options;


pub struct ParsedArgs {
    pub source: String,
    pub input: String,
    pub verbose: u8,
}

pub fn create_options() -> (Options) {
    let mut opts = Options::new();
    opts.optflagopt("v", "verbose", "Print More Info", "VERBOSITY");
    opts.optopt("s", "source", "Source string", "SOURCE");
    opts.optopt("i", "input", "Input string", "INPUT");
    opts
}

pub fn parse_args(opts: Options, args: Vec<String>) -> Result<ParsedArgs, io::Error> {
    // parse out everything we're interested in handling
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let verbose = matches.opt_default("v", "0");
    let input_flag = matches.opt_str("i");
    let source_flag = matches.opt_str("s");

    // Get the verbosity as an integer
    let verbosity = match verbose {
        Some(s) => {
            match s.as_ref() {
                "0" => 0,
                "1" => 1,
                "2" => 2,
                "3" => 3,
                _ => 0,
            }
        }
        None => { 0 }
    };

    // Get the source code string
    let source = match source_flag {
        Some(s) => s,
        None => {
            let first_arg = args[1].clone();
            let mut source_file = try!(File::open(first_arg));
            let mut source_string = String::new();
            try!(source_file.read_to_string(&mut source_string));
            source_string
        }
    };

    // Get the input string, either from an argument or stdin.
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

    Ok(ParsedArgs {
        source: source,
        input: input,
        verbose: verbosity,
    })
}
