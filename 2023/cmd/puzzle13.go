package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"slices"
)

type Point = aoc.PointI2
type Grid = *aoc.Grid[GroundObject]

const (
	ObjectAsh = 0
	ObjectRock = 1
)
type GroundObject struct {
	Type int
}
func ParseGroundObject(object rune) GroundObject {
	switch object {
		case '.': return GroundObject{ObjectAsh}
		case '#': return GroundObject{ObjectRock}
		default: panic("invalid object")
	}
}
func FmtGroundObject(object GroundObject) string {
	var result rune

	switch object.Type {
		case ObjectAsh: result = '.'
		case ObjectRock: result = '#'
		default: result = rune(object.Type)
	}

	return string(result)
}

// returns number of elements mismatching
func compareLines(a []GroundObject, b []GroundObject) int {
	if len(a) != len(b) {
		panic("lines must have the same length")
	}
	
	var mismatching = 0
	for i := 0; i < len(a); i += 1 {
		if a[i] != b[i] {
			mismatching += 1
		}
	}

	return mismatching
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var patterns = make([]Grid, 0)
	var currentPattern = 0

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" {
			currentPattern += 1
			continue
		}
		aoc.LogTrace("text = %s\n", t)

		if len(patterns) <= currentPattern {
			patterns = append(patterns, aoc.MakeGrid[GroundObject]())
		}

		var row = make([]GroundObject, 0)
		for _, r := range t {
			var object = ParseGroundObject(r)
			row = append(row, object)
		}
		patterns[currentPattern].AddRow(patterns[currentPattern].Height(), row)
	}
	if aoc.LogEnabled(aoc.LogLevelTrace) {
		for _, pattern := range patterns {
			aoc.LogTrace("pattern:\n%v", pattern.FmtDebug(FmtGroundObject))
		}
	}

	for _, pattern := range patterns {
		var foundMirror = false
		for y := 0; y < pattern.Height() - 1; y += 1 {
			var allEqual = true
			var maxOffset = aoc.MinI(y, pattern.Height() - y - 2)
			for o := 0; o <= maxOffset; o += 1 {
				var top = pattern.GetRow(y - o)
				var bottom = pattern.GetRow(y + 1 + o)
				if !slices.Equal(top, bottom) {
					allEqual = false
					break
				}
			}

			if allEqual {
				aoc.LogDebug("row mirror %d\n", y)
				result += (y + 1) * 100
				foundMirror = true
				break
			}
		}
		if foundMirror { continue }

		for x := 0; x < pattern.Width(0) - 1; x += 1 {
			var allEqual = true
			var maxOffset = aoc.MinI(x, pattern.Width(0) - x - 2)
			for o := 0; o <= maxOffset; o += 1 {
				var left = pattern.GetColumn(x - o)
				var right = pattern.GetColumn(x + 1 + o)
				if !slices.Equal(left, right) {
					allEqual = false
					break
				}
			}

			if allEqual {
				aoc.LogDebug("column mirror %d\n", x)
				result += x + 1
				foundMirror = true
				break
			}
		}

		if !foundMirror {
			panic("Pattern has no mirror")
		}
	}

	// smudge go brr
	for _, pattern := range patterns {
		var foundMirror = false
		for y := 0; y < pattern.Height() - 1; y += 1 {
			var differences = 0
		
			var maxOffset = aoc.MinI(y, pattern.Height() - y - 2)
			for o := 0; o <= maxOffset; o += 1 {
				var top = pattern.GetRow(y - o)
				var bottom = pattern.GetRow(y + 1 + o)

				differences += compareLines(top, bottom)
			}

			if differences == 1 {
				aoc.LogDebug("row smudge mirror %d\n", y)
				result2 += (y + 1) * 100
				foundMirror = true
				break
			}
		}
		if foundMirror { continue }

		for x := 0; x < pattern.Width(0) - 1; x += 1 {
			var differences = 0
			var maxOffset = aoc.MinI(x, pattern.Width(0) - x - 2)
			for o := 0; o <= maxOffset; o += 1 {
				var left = pattern.GetColumn(x - o)
				var right = pattern.GetColumn(x + 1 + o)

				differences += compareLines(left, right)
			}

			if differences == 1 {
				aoc.LogDebug("column smudge mirror %d\n", x)
				result2 += x + 1
				foundMirror = true
				break
			}
		}

		if !foundMirror {
			panic("Pattern has no mirror")
		}
	}

	fmt.Println(result, result2)
}