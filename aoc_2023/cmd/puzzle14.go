package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
)

type Point = aoc.PointI2
type Grid = *aoc.Grid[GroundObject]

const (
	ObjectUnknown = 0
	ObjectEmpty = 1
	ObjectCube = 2
	ObjectRound = 3
)
type GroundObject struct {
	Type int
}
func ParseGroundObject(object rune) GroundObject {
	switch object {
		case '.': return GroundObject{ObjectEmpty}
		case '#': return GroundObject{ObjectCube}
		case 'O': return GroundObject{ObjectRound}
		default: panic("invalid object")
	}
}
func FmtGroundObject(object GroundObject) string {
	var result rune

	switch object.Type {
		case ObjectEmpty: result = '.'
		case ObjectCube: result = '#'
		case ObjectRound: result = 'O'
		default: result = rune(object.Type)
	}

	return string(result)
}

func rollDirection(platform Grid, direction Point) bool {
	var changed = false

	for y := 0; y < platform.Height(); y += 1 {
		for x := 0; x < platform.Width(y); x += 1 {
			var p = Point{x, y}
			var pNext = p.Add(direction)
			if platform.Get(p).Type == ObjectRound && platform.Get(pNext).Type == ObjectEmpty {
				platform.Set(p, GroundObject{ObjectEmpty})
				platform.Set(pNext, GroundObject{ObjectRound})
				changed = true
			}
		}
	}

	return changed
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var platform = aoc.MakeGrid[GroundObject]()

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)

		var row = make([]GroundObject, 0)
		for _, r := range t {
			var object = ParseGroundObject(r)
			row = append(row, object)
		}
		platform.AddRow(platform.Height(), row)
	}
	aoc.LogTrace("platform:\n%v", platform.FmtDebug(FmtGroundObject))
	var platform2 = platform.Clone()

	for rollDirection(platform, Point{0, -1}) {}
	aoc.LogTrace("platform rolled:\n%v", platform.FmtDebug(FmtGroundObject))

	for y := 0; y < platform.Height(); y += 1 {
		var load = platform.Height() - y
		for x := 0; x < platform.Width(y); x += 1 {
			if platform.Get(Point{x, y}).Type == ObjectRound {
				result += load
			}
		}
	}
	aoc.LogInfo("result = %d\n", result)

	var platform2History = make([]Grid, 0)
	var platform2Period = 0
	var platform2Offset = 0
	for i := 0; i < 1_000_000_000; i += 1 {
		for rollDirection(platform2, Point{0, -1}) {}
		for rollDirection(platform2, Point{-1, 0}) {}
		for rollDirection(platform2, Point{0, 1}) {}
		for rollDirection(platform2, Point{1, 0}) {}

		for hi, h := range platform2History {
			if aoc.GridEqual(platform2, h) {
				platform2Period = i - hi
				platform2Offset = hi
				break
			}
		}
		if platform2Period != 0 { break }
		platform2History = append(platform2History, platform2.Clone())
	}
	aoc.LogInfo("period %d offset %d\n", platform2Period, platform2Offset)
	var platform2Chosen = platform2History[platform2Offset + (1_000_000_000 - platform2Offset) % platform2Period - 1]
	aoc.LogTrace("platform2 rolled:\n%v", platform2Chosen.FmtDebug(FmtGroundObject))

	for y := 0; y < platform2Chosen.Height(); y += 1 {
		var load = platform2Chosen.Height() - y
		for x := 0; x < platform2Chosen.Width(y); x += 1 {
			if platform2Chosen.Get(Point{x, y}).Type == ObjectRound {
				result2 += load
			}
		}
	}

	fmt.Println(result, result2)
}