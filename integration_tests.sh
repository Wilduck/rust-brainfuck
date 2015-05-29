#!/bin/sh

# Setup interpreter
BRAINFUCK="./target/debug/brainfuck"
for i in "$@"
do
    if [ $i = "--release" ]; then
        BRAINFUCK="./target/release/brainfuck"
    fi
done

RED="\033[0;31m"
GREEN="\033[0;32m"
NOCOLOR="\033[0m"
ERR_FILE="ERR_TMP.txt"
touch $ERR_FILE

pass () {
    printf "${GREEN}PASS${NOCOLOR} ${1}\n"
}

fail () {
    printf "${RED}FAIL${NOCOLOR} ${1}\n"
}

run () {
    name=$1
    cmd=$2
    expected=$3
    out=$(eval $cmd 2> "$ERR_FILE")
    err=$(cat "$ERR_FILE")
    if [[ $out = $expected && $err = '' ]]; then 
        pass $name
    else 
        fail $name
        echo ".... saw: '${out}' expected: '${expected}'"
        echo ".... error: ${err}"
    fi
}

# Tests
run "command-line" "$BRAINFUCK -i 'hi' -s ',.,.'" 'hi'
run "no-input" "$BRAINFUCK -i '' -s ',,,'" ''
run "file-processing" "$BRAINFUCK brainfuck_source/loops.bf -i ''" 'E'
run "std-in-processing" "printf hi | $BRAINFUCK -s ',.,.'" 'hi'
run "empty-loop" "$BRAINFUCK brainfuck_source/empty_loop.bf -i ''" ''

rm $ERR_FILE
