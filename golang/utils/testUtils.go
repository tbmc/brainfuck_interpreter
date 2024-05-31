package utils

import "testing"

func IsEqual(a, b []byte) bool {
	if len(a) != len(b) {
		return false
	}

	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}

	return true
}

func Expect(t *testing.T, expected, result string) {
	if result != expected {
		t.Fatalf("Expected \"%v\" != Result \"%v\"", expected, result)
	}
}
