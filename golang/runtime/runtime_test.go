package runtime

import (
	"brainfuck/utils"
	"bufio"
	"bytes"
	"testing"
)

func TestIncrementDecrementMove(t *testing.T) {
	runtime := New()

	runtime.IncrementValue()
	runtime.IncrementValue()

	_ = runtime.IncrementPointer()
	_ = runtime.IncrementPointer()

	runtime.DecrementValue()

	_ = runtime.DecrementPointer()
	runtime.IncrementValue()

	data := runtime.ExtractData()
	if !utils.IsEqual(data, []byte{2, 1, 255}) {
		t.Fatalf("bad data result")
	}
}

func TestPutReadChar(t *testing.T) {
	stdin := bytes.NewBuffer(make([]byte, 0))
	reader := bufio.NewReader(stdin)

	stdout := bytes.NewBuffer(make([]byte, 0))
	writer := bufio.NewWriter(stdout)

	runtime := NewCustomRuntime(reader, writer)

	stdin.WriteString("569")

	_ = runtime.GetChar()
	_ = runtime.PutChar()

	_ = runtime.GetChar()
	_ = runtime.PutChar()

	written := make([]byte, 3)
	_, _ = stdout.Read(written)

	if !utils.IsEqual(written, []byte{53, 54, 0}) {
		t.Fatalf("bad result written to stdout")
	}
}
