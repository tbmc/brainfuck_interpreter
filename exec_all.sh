#!/bin/bash


cd svelte
(node build) &
cd ..
./brainfuck

trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT
