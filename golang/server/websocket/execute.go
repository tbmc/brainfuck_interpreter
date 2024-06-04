package websocket

import (
	"brainfuck/customBuffer"
	"brainfuck/interpreter"
	"brainfuck/runtime"
	"bufio"
	"time"
)

func execute(code string, input chan []byte, output chan []byte, closeFn func()) {

	customInput := customBuffer.New(input, nil)
	reader := bufio.NewReader(&customInput)

	customOutput := customBuffer.New(nil, output)
	writer := bufio.NewWriter(&customOutput)

	runtime_ := runtime.NewCustomRuntime(reader, writer)

	err := interpreter.InterpretCodeCustomRuntime(&runtime_, &code)

	time.Sleep(100 * time.Millisecond)
	output <- nil
	time.Sleep(1 * time.Second)

	close(input)
	close(output)
	closeFn()

	if err != nil {
		return
	}
}
