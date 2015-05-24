BrainFuck in Rust
=================

A brainfuck interpreter written in Rust.


Building the Brainfuck Interpreter
----------------------------------

Build with:

    cargo build --release


Running Brainfuck Programs
--------------------------

After building, a binary will be in `./target/release/`. This binary
takes a few command line arguments to execute programs. The `-s` or
`--source` flag takes a string for the brainfuck source code to
execute, the `-i` or `--input` flag will set the input for the program
to be executed with.

As an example, the following invocation:

    ./brainfuck --source '+++ +++ [ > ++ [> ++ ++ ++ <-] <-] >>.' --input '5'

when run, will produce the following output:

    Running program:
    +++ +++ [ > ++ [> ++ ++ ++ <-] <-] >>.
    
    With input:
    5
    
    Output:
    H


