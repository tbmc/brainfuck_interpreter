package utils

import "strings"

func Trim(str string) string {
	return strings.Trim(strings.TrimSpace(str), "\r\n\u0000")
}
