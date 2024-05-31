package abstractSyntaxTree

import (
	"errors"
	"fmt"
)

type Ast struct {
	Char          byte
	SubAsts       []*Ast
	parent        *Ast
	IsLeaf        bool
	indexInString int
}

func New() Ast {
	return Ast{
		0, make([]*Ast, 0), nil, false, 0,
	}
}

func (ast *Ast) AddNewNode(value byte, isLeaf bool, indexInString int) *Ast {
	node := &Ast{value, make([]*Ast, 0), ast, isLeaf, indexInString}
	ast.SubAsts = append(ast.SubAsts, node)
	return node
}

func (ast *Ast) CloseBranch(value byte, indexInString int) *Ast {
	ast.parent.AddNewNode(value, true, indexInString)
	return ast.parent
}

func (ast *Ast) SyntaxCheck() error {
	if len(ast.SubAsts) == 0 {
		return nil
	}

	return ast.internalSyntaxCheck()
}

func (ast *Ast) internalSyntaxCheck() error {
	if len(ast.SubAsts) == 0 {
		return errors.New("loop empty, it creates an infinite loop")
	}

	for index, subNode := range ast.SubAsts {
		if subNode.IsLeaf {
			continue
		}

		if subNode.Char == ']' {
			continue
		}

		if subNode.Char != '[' {
			return fmt.Errorf("invalid branch at %d, it should not happen", subNode.indexInString)
		}

		err := subNode.internalSyntaxCheck()
		if err != nil {
			return err
		}

		if index+1 >= len(ast.SubAsts) {
			return fmt.Errorf("invalid branch closing at %d, no closing found (no node after this one)", subNode.indexInString)
		}
		next := ast.SubAsts[index+1]
		if next.Char != ']' {
			return fmt.Errorf("invalid branch closing at %d, no closing found", subNode.indexInString)
		}
	}

	return nil
}

func ParseCode(code *string) Ast {
	root := New()
	ast := &root
	for i, char := range *code {
		b := byte(char)
		switch char {
		case '>', '<', '+', '-', '.', ',':
			ast.AddNewNode(b, true, i)
		case '[':
			ast = ast.AddNewNode(b, false, i)
		case ']':
			ast = ast.CloseBranch(b, i)
		default:
			// Char is comment, so it is ignored
		}
	}
	return root
}
