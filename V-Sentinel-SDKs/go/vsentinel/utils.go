package vsentinel

import (
	"strconv"
	"strings"
)

// intToString converts an int to a string.
func intToString(i int) string {
	return strconv.Itoa(i)
}

// joinStrings joins a slice of strings with a separator.
func joinStrings(strs []string, sep string) string {
	return strings.Join(strs, sep)
}

// joinIOCType joins a slice of IOCType with a separator.
func joinIOCType(types []IOCType, sep string) string {
	strs := make([]string, len(types))
	for i, t := range types {
		strs[i] = string(t)
	}
	return strings.Join(strs, sep)
}