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

########################################
#                Tests                 #
########################################

# Command Line Handling
#######################
echo "Command Line Handling"
echo "---------------------"
run "command-line" "$BRAINFUCK -i 'hi' -s ',.,.'" 'hi'
run "file-processing" "$BRAINFUCK brainfuck_source/loops.bf -i ''" 'E'
run "std-in-processing" "printf hi | $BRAINFUCK -s ',.,.'" 'hi'
echo ""

# Behavior
##########
echo "Behavior"
echo "--------"
run "no-input" "$BRAINFUCK -i '' -s ',,,'" ''
run "input-overrides-existing-value" "$BRAINFUCK -i 'A' -s '+++++,.'" 'A'
run "null-print" "$BRAINFUCK -i '' -s '++,.'" ''
run "empty-loop" "$BRAINFUCK -i '' -s '[]'" ''
run "immediate-dec-pointer" "$BRAINFUCK -i '' -s '<'" ''
run "fill-ones-forever" "$BRAINFUCK -i '' -s '+[>+]'" ''
run "print-216-code-point" "$BRAINFUCK -i '' -s '++++++[>++++++[>++++++<-]<-]>>.'" 'Ø'
run "overflow" "$BRAINFUCK -i '' -s '++++[>++++[>++++[>++++<-]<-]<-]>>>+'" ''
run "underflow" "$BRAINFUCK -i '' -s '-'" ''
echo ""

# Programs
##########
echo "Programs"
echo "--------"
run "copy-program" "$BRAINFUCK brainfuck_source/echo.bf -i 'echo'" 'echo'
run "reverse-program" "$BRAINFUCK brainfuck_source/reverse.bf -i 'echo'" 'ohce'
run "binary-to-ascii" "$BRAINFUCK brainfuck_source/binary-to-ascii.bf -i '0110100001101001'" 'hi'
echo ""

rm $ERR_FILE
