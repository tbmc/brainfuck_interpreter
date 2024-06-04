package main

import (
	"brainfuck/interpreter"
	"brainfuck/server"
	"fmt"
	"os"
	"strings"
)

func main() {
	fmt.Println("Brainfuck Interpreter")

	if len(os.Args) == 0 || strings.ToLower(os.Args[1]) == "serve" {
		server.Serve(3001)
	} else {
		filePath := os.Args[1]

		content, err := os.ReadFile(filePath)
		if err != nil {
			panic("File does not exists")
		}

		code := string(content)

		err = interpreter.InterpretCode(&code)
		if err != nil {
			panic("Error during execution: " + fmt.Sprintf("%v", err))
		}
	}
}
