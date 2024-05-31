package interpreter

import (
	"brainfuck/abstractSyntaxTree"
	"brainfuck/runtime"
)

func InterpretCodeCustomRuntime(runtime_ *runtime.Runtime, script *string) error {
	ast := abstractSyntaxTree.ParseCode(script)
	err := ast.SyntaxCheck()
	if err != nil {
		return err
	}
	err = executeCode(runtime_, &ast)

	return nil
}

func InterpretCode(script *string) error {
	runtime_ := runtime.New()
	return InterpretCodeCustomRuntime(&runtime_, script)
}

func executeCode(runtime *runtime.Runtime, ast *abstractSyntaxTree.Ast) error {
	nodeIndex := 0
	for nodeIndex < len(ast.SubAsts) {
		node := ast.SubAsts[nodeIndex]
		if !node.IsLeaf {
			if runtime.JumpToNextBracket() {
				nodeIndex++
			} else {
				err := executeCode(runtime, node)
				if err != nil {
					return err
				}
			}
		} else if node.Char == ']' {
			if runtime.JumpToPreviousBracket() {
				nodeIndex -= 2
			} else {
				// do nothing
			}
		} else {
			err := executeLeaf(runtime, node)
			if err != nil {
				return err
			}
		}
		nodeIndex++
	}

	return nil
}

func executeLeaf(runtime *runtime.Runtime, ast *abstractSyntaxTree.Ast) error {
	switch ast.Char {
	case '>':
		err := runtime.IncrementPointer()
		if err != nil {
			return err
		}
	case '<':
		err := runtime.DecrementPointer()
		if err != nil {
			return err
		}
	case '+':
		runtime.IncrementValue()
	case '-':
		runtime.DecrementValue()
	case '.':
		err := runtime.PutChar()
		if err != nil {
			return err
		}
	case ',':
		err := runtime.GetChar()
		if err != nil {
			return err
		}
	default:
		// do nothing
	}

	return nil
}
