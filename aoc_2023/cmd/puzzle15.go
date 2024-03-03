package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
	"strconv"
)

func hashAlgorithm(input string) int {
	var current = 0
	for i := 0; i < len(input); i += 1 {
		current = current + int(input[i])
		current *= 17
		current = current % 256
	}

	return current
}

type BoxEntry struct {
	Label string
	FocalLength int
}
func findEntry(box []BoxEntry, label string) int {
	for i, e := range box {
		if e.Label == label { return i }
	}

	return -1
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var boxes = make(map[int][]BoxEntry)
	for i := 0; i < 256; i += 1 {
		boxes[i] = make([]BoxEntry, 0)
	}

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)

		var parts = strings.Split(t, ",")
		for _, part := range parts {
			result += hashAlgorithm(part)

			if strings.HasSuffix(part, "-") {
				var label = part[:len(part) - 1]
				// remove
				var key = hashAlgorithm(label)
				var entryIndex = findEntry(boxes[key], label)
				if entryIndex != -1 {
					boxes[key] = append(boxes[key][:entryIndex], boxes[key][entryIndex + 1:]...)
				}
			} else {
				var p = strings.Split(part, "=")
				var label = p[0]
				var focalLength, _ = strconv.Atoi(p[1])
				// set
				var key = hashAlgorithm(label)
				var entryIndex = findEntry(boxes[key], label)
				var newEntry = BoxEntry{label,focalLength}
				if entryIndex != -1 {
					boxes[key][entryIndex] = newEntry
				} else {
					boxes[key] = append(boxes[key], newEntry)
				}
			}
		}
	}
	aoc.LogDebug("boxes = %v\n", boxes)

	for bi, box := range boxes {
		for ei, entry := range box {
			result2 += (bi + 1) * (ei + 1) * entry.FocalLength
		}
	}

	fmt.Println(result, result2)
}