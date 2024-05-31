package main

import (
	"brainfuck/consts"
	"brainfuck/interpreter"
	"fmt"
	"os"
)

const defaultScriptPath = consts.ScriptPath + "/prime_2.bf"

func main() {
	fmt.Println("Brainfuck Interpreter")

	var filePath string = defaultScriptPath
	if len(os.Args) > 1 {
		filePath = os.Args[1]
	}

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
