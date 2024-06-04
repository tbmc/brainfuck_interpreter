package customBuffer

type Buffer struct {
	input  chan []byte
	output chan []byte

	internalReadBuffer []byte
}

func New(input chan []byte, output chan []byte) Buffer {
	return Buffer{input, output, nil}
}

func (b *Buffer) Read(p []byte) (int, error) {
	var data []byte

	if b.internalReadBuffer != nil && len(b.internalReadBuffer) > 0 {
		data = make([]byte, len(b.internalReadBuffer))
		copy(data, b.internalReadBuffer)
	} else {
		data = <-b.input
	}

	if len(data) > len(p) {
		copied := copy(p, data[:len(p)])
		b.internalReadBuffer = make([]byte, len(data)-len(p))
		copy(b.internalReadBuffer, data[len(p):])
		return copied, nil
	} else {
		return copy(p, data), nil
	}
}

func (b *Buffer) Write(p []byte) (n int, err error) {
	data := make([]byte, len(p))
	copy(data, p)
	b.output <- data
	return len(data), nil
}
