#!/bin/bash


cd svelte
(node build) &
cd ..
cd websocketServer
yarn start

trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT
