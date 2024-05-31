package interpreter

import (
	"brainfuck/consts"
	"brainfuck/runtime"
	"brainfuck/utils"
	"bufio"
	"bytes"
	"os"
	"testing"
)

const scriptPath = "../" + consts.ScriptPath

func testForFile(t *testing.T, filePath, input string) string {
	content, err := os.ReadFile(scriptPath + filePath)
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}

	code := string(content)

	stdin := bytes.NewBuffer(make([]byte, 0))
	stdin.WriteString(input)
	reader := bufio.NewReader(stdin)

	stdout := bytes.NewBuffer(make([]byte, 0))
	writer := bufio.NewWriter(stdout)

	runtime_ := runtime.NewCustomRuntime(reader, writer)

	err = InterpretCodeCustomRuntime(&runtime_, &code)
	if err != nil {
		t.Fatalf("There is an error %v", err)
	}

	written := make([]byte, 1_000)
	red, _ := stdout.Read(written)
	result := utils.Trim(string(written[:red]))

	return result
}

func TestInterpreterFileHelloWorld(t *testing.T) {
	result := testForFile(t, "/hello_world.bf", "")
	utils.Expect(t, "Hello World!", result)
}

func TestInterpreterFileCellSize(t *testing.T) {
	result := testForFile(t, "/cell_size.bf", "")
	utils.Expect(t, "8 bit cells", result)
}

func TestInterpreterFileReadPrint(t *testing.T) {
	result := testForFile(t, "/test/read_print.bf", "This is a test!\n")
	utils.Expect(t, "This is a test!", result)
}

func TestInterpreterFileFibonacci(t *testing.T) {
	result := testForFile(t, "/fibonacci.bf", "")
	utils.Expect(t, "1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89", result)
}

func TestInterpreterFilePrime1(t *testing.T) {
	result := testForFile(t, "/prime.bf", "")
	utils.Expect(t, "29, 23, 19, 17, 13, 11, 7, 5, 3, 2, 1,", result)
}

func TestInterpreterFilePrime2_50(t *testing.T) {
	result := testForFile(t, "/prime_2.bf", "50\n")
	utils.Expect(t, "Primes up to: 2 3 5 7 11 13 17 19 23 29 31 37 41 43 47", result)
}

func TestInterpreterFilePrime2_20(t *testing.T) {
	result := testForFile(t, "/prime_2.bf", "20\n")
	utils.Expect(t, "Primes up to: 2 3 5 7 11 13 17 19", result)
}
