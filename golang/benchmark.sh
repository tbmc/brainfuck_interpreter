#!/usr/bin/env bash

go build
# Not necessary, the only difference is 200ms
# go build  -ldflags="-s -w"

time ./brainfuck ../brain_fuck_scripts/prime_benchmark.bf
