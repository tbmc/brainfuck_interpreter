package runtime

import (
	"bufio"
	"errors"
	"os"
)

const ArraySize = 30_000
const MaxInstructions int64 = 5_000_000_000

type Runtime struct {
	ptr                int
	data               []byte
	maxPtr             int
	instructionCounter int

	reader *bufio.Reader
	writer *bufio.Writer
}

func NewCustomRuntime(reader *bufio.Reader, writer *bufio.Writer,

) Runtime {
	data := make([]byte, ArraySize)
	return Runtime{
		0, data, 0, 0, reader, writer,
	}
}

func New() Runtime {
	return NewCustomRuntime(bufio.NewReader(os.Stdin), bufio.NewWriter(os.Stdout))

}

func (r *Runtime) incrementInstructionCounter() {
	r.instructionCounter++

	if int64(r.instructionCounter) > MaxInstructions {
		// todo: dump
		panic("Max instructions exceeded")
	}
}

func (r *Runtime) ExtractData() []byte {
	data := make([]byte, r.maxPtr+1)
	copy(data, r.data)
	return data
}

func (r *Runtime) IncrementPointer() error {
	r.ptr++

	if r.ptr >= ArraySize {
		return errors.New("array overflow")
	}

	if r.maxPtr < r.ptr {
		r.maxPtr = r.ptr
	}

	r.incrementInstructionCounter()

	return nil
}

func (r *Runtime) DecrementPointer() error {
	r.ptr--

	if r.ptr < 0 {
		return errors.New("array index < 0")
	}

	return nil
}

func (r *Runtime) IncrementValue() {
	r.data[r.ptr]++
	r.incrementInstructionCounter()
}

func (r *Runtime) DecrementValue() {
	r.data[r.ptr]--
	r.incrementInstructionCounter()
}

func (r *Runtime) PutChar() error {
	char := r.data[r.ptr]

	err := r.writer.WriteByte(char)
	if err != nil {
		return err
	}

	err = r.writer.Flush()
	if err != nil {
		return err
	}

	r.incrementInstructionCounter()
	return nil
}

func (r *Runtime) GetChar() error {
	byte_, err := r.reader.ReadByte()

	if err != nil {
		return err
	}
	r.data[r.ptr] = byte_

	r.incrementInstructionCounter()
	return nil
}

func (r *Runtime) JumpToNextBracket() bool {
	r.incrementInstructionCounter()
	currentValue := r.data[r.ptr]
	return currentValue == 0
}

func (r *Runtime) JumpToPreviousBracket() bool {
	r.incrementInstructionCounter()
	currentValue := r.data[r.ptr]
	return currentValue != 0
}
