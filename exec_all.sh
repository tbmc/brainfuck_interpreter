#!/bin/bash

cd websocketServer
(yarn start) &
cd ..
cd svelte
node build

trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT
