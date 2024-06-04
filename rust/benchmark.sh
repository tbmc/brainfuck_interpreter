#!/usr/bin/env bash

# Very useful, 30.5s in release and 4m50s in debug. Release is almost 10x faster
cargo build --release
cp target/release/brainfuck_interpreter.exe brainfuck_interpreter.exe
#cargo build
#cp target/debug/brainfuck_interpreter.exe brainfuck_interpreter.exe

time ./brainfuck_interpreter.exe ../brain_fuck_scripts/prime_benchmark.bf
