package customBuffer

import (
	"bytes"
	"testing"
)

func assertReadBytes(buffer *Buffer, t *testing.T, pBuffer, expected []byte, expectedRed int) {
	red, _ := buffer.Read(pBuffer)
	if red != expectedRed {
		t.Fatalf("should have red %d bytes", expectedRed)
	}
	if !bytes.Equal(pBuffer, expected) {
		t.Fatalf("bad data red: %v", expectedRed)
	}
}

func TestBufferReadSimple(t *testing.T) {
	input := make(chan []byte, 5)

	input <- []byte{1, 2, 3, 4, 5}
	buffer := New(input, nil)

	assertReadBytes(&buffer, t, []byte{0, 0}, []byte{1, 2}, 2)
	assertReadBytes(&buffer, t, []byte{0, 0, 0, 0}, []byte{3, 4, 5, 0}, 3)
}

func TestBufferReadMultipleTimes(t *testing.T) {
	input := make(chan []byte, 5)

	input <- []byte{1}
	buffer := New(input, nil)

	assertReadBytes(&buffer, t, []byte{0, 0}, []byte{1, 0}, 1)

	input <- []byte{2}
	input <- []byte{3}
	input <- []byte{4}
	assertReadBytes(&buffer, t, []byte{0}, []byte{2}, 1)
	assertReadBytes(&buffer, t, []byte{0}, []byte{3}, 1)
	assertReadBytes(&buffer, t, []byte{0}, []byte{4}, 1)
}

func TestBufferWriteSimple(t *testing.T) {
	output := make(chan []byte, 1)
	buffer := New(nil, output)

	written, err := buffer.Write([]byte{1, 2, 3})
	if err != nil {
		return
	}
	if written != 3 {
		t.Fatalf("written bytes should be 3 (!= %v)", written)
	}

	data := <-output
	if !bytes.Equal(data, []byte{1, 2, 3}) {
		t.Fatalf("Data written incorrect: %v", data)
	}
}
