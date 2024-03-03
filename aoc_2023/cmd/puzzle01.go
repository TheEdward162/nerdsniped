package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
	"unicode"
)

/// Returns the first number found in the string i or -1.
func parsePrefix(i string) int {
	switch i[0] {
		case '0': return 0
		case '1': return 1
		case '2': return 2
		case '3': return 3
		case '4': return 4
		case '5': return 5
		case '6': return 6
		case '7': return 7
		case '8': return 8
		case '9': return 9
	}

	switch {
		case strings.HasPrefix(i, "zero"): return 0
		case strings.HasPrefix(i, "one"): return 1
		case strings.HasPrefix(i, "two"): return 2
		case strings.HasPrefix(i, "three"): return 3
		case strings.HasPrefix(i, "four"): return 4
		case strings.HasPrefix(i, "five"): return 5
		case strings.HasPrefix(i, "six"): return 6
		case strings.HasPrefix(i, "seven"): return 7
		case strings.HasPrefix(i, "eight"): return 8
		case strings.HasPrefix(i, "nine"): return 9
	}

	return -1
}

func main() {
	input, err := aoc.Initialize()
	if err != nil { panic(err) }

	result := 0
	result2 := 0

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		t := scanner.Text()
		aoc.LogTrace("text = %s\n", t)

		trimmed := strings.TrimFunc(
			t,
			func(r rune) bool {
				return !unicode.IsNumber(r)
			},
		)
		aoc.LogDebug("trimmed = %s\n", trimmed)
		if len(trimmed) > 0 {
			result += int(trimmed[0] - '0') * 10 + int(trimmed[len(trimmed) - 1] - '0')
		}

		r2First := -1
		r2Last := -1
		for len(t) > 0 {
			num := parsePrefix(t)
			aoc.LogTrace("parsePrefix(%s) = %d, %d\n", t, num)
			t = t[1:]
			if num == -1 {
				continue
			}

			if r2First == -1 {
				r2First = num
			}
			r2Last = num
		}
		aoc.LogDebug("first = %d, last = %d\n", r2First, r2Last)
		result2 += r2First * 10 + r2Last
	}

	fmt.Println(result, result2)
}
