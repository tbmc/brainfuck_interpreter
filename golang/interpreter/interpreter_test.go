package interpreter

import (
	"brainfuck/runtime"
	"brainfuck/utils"
	"testing"
)

func TestInterpreterNoInstruction(t *testing.T) {
	code := "test"
	runtime_ := runtime.New()
	err := InterpretCodeCustomRuntime(&runtime_, &code)

	data := runtime_.ExtractData()
	if err != nil || !utils.IsEqual(data, []byte{0}) {
		t.Fatalf("There is an error %v, data %#v", err, data)
	}
}

func TestInterpreterSimple(t *testing.T) {
	code := "+++ > +"
	runtime_ := runtime.New()
	err := InterpretCodeCustomRuntime(&runtime_, &code)

	data := runtime_.ExtractData()
	if err != nil || !utils.IsEqual(data, []byte{3, 1}) {
		t.Fatalf("There is an error")
	}
}

func TestInterpreterSimpleOperators2Minus2(t *testing.T) {
	code := "+++ > -- < - >> +"
	runtime_ := runtime.New()
	err := InterpretCodeCustomRuntime(&runtime_, &code)

	data := runtime_.ExtractData()
	if err != nil || !utils.IsEqual(data, []byte{2, 254, 1}) {
		t.Fatalf("There is an error")
	}
}

func TestInterpreterSimpleLoop(t *testing.T) {
	code := "+++[-]+"
	runtime_ := runtime.New()
	err := InterpretCodeCustomRuntime(&runtime_, &code)

	data := runtime_.ExtractData()
	if err != nil || !utils.IsEqual(data, []byte{1}) {
		t.Fatalf("There is an error")
	}
}

func TestInterpreterLoopIn(t *testing.T) {
	code := "[-]+"
	runtime_ := runtime.New()
	err := InterpretCodeCustomRuntime(&runtime_, &code)

	data := runtime_.ExtractData()
	if err != nil || !utils.IsEqual(data, []byte{1}) {
		t.Fatalf("There is an error")
	}
}
