package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"k8s.io/apimachinery/pkg/util/sets"
)

type Point = aoc.PointI2
type Grid = *aoc.Grid[Object]

const (
	ObjectUnknown = 0
	ObjectEmpty = 1
	ObjectSplitterVertical = 2
	ObjectSplitterHorizontal = 3
	ObjectMirrorForward = 4
	ObjectMirrorBackward = 5
	//
	MarkerIdle = 6
	MarkerEnergized = 7
)
type Object struct {
	Type int
}
func ParseObject(object rune) Object {
	switch object {
		case '.': return Object{ObjectEmpty}
		case '|': return Object{ObjectSplitterVertical}
		case '-': return Object{ObjectSplitterHorizontal}
		case '/': return Object{ObjectMirrorForward}
		case '\\': return Object{ObjectMirrorBackward}
		default: panic("invalid object")
	}
}
func FmtObject(object Object) string {
	var result rune

	switch object.Type {
		case ObjectEmpty: result = '.'
		case ObjectSplitterVertical: result = '|'
		case ObjectSplitterHorizontal: result = '-'
		case ObjectMirrorForward: result = '/'
		case ObjectMirrorBackward: result = '\\'
		case MarkerIdle: result = '.'
		case MarkerEnergized: result = '#'
		default: result = rune(object.Type)
	}

	return string(result)
}

type Beam struct {
	Pos Point
	Dir Point
}
func updateBeam(contraption Grid, beam Beam) []Beam {
	var nextBeams = make([]Beam, 0)

	switch (contraption.Get(beam.Pos).Type) {
		case ObjectEmpty: {
			nextBeams = append(nextBeams, Beam{beam.Pos.Add(beam.Dir), beam.Dir})
		}
		case ObjectSplitterVertical: {
			if beam.Dir.X != 0 {
				nextBeams = append(nextBeams, Beam{beam.Pos, beam.Dir.RotNeg90()})
				nextBeams = append(nextBeams, Beam{beam.Pos, beam.Dir.Rot90()})
			} else {
				nextBeams = append(nextBeams, Beam{beam.Pos.Add(beam.Dir), beam.Dir})
			}
		}
		case ObjectSplitterHorizontal: {
			if beam.Dir.Y != 0 {
				nextBeams = append(nextBeams, Beam{beam.Pos, beam.Dir.RotNeg90()})
				nextBeams = append(nextBeams, Beam{beam.Pos, beam.Dir.Rot90()})
			} else {
				nextBeams = append(nextBeams, Beam{beam.Pos.Add(beam.Dir), beam.Dir})
			}
		}
		case ObjectMirrorForward: {
			var newDir Point
			if beam.Dir.X != 0 {
				newDir = Point{0, -beam.Dir.X}
			} else if beam.Dir.Y != 0 {
				newDir = Point{-beam.Dir.Y, 0}
			}
			nextBeams = append(nextBeams, Beam{beam.Pos.Add(newDir), newDir})
		}
		case ObjectMirrorBackward: {
			var newDir Point
			if beam.Dir.X != 0 {
				newDir = Point{0, beam.Dir.X}
			} else if beam.Dir.Y != 0 {
				newDir = Point{beam.Dir.Y, 0}
			}
			nextBeams = append(nextBeams, Beam{beam.Pos.Add(newDir), newDir})
		}
		default: {}
	}
	aoc.LogTrace("updated beam at %v (tile %d) going %v to %v\n", beam.Pos, contraption.Get(beam.Pos), beam.Dir, nextBeams)

	return nextBeams
}

func computeEnergy(contraption Grid, start Beam) int {
	var result = 0
	
	var energyMap = contraption.Clone()
	if aoc.LogEnabled(aoc.LogLevelTrace) {
		for y := 0; y < energyMap.Height(); y += 1 {
			for x := 0; x < energyMap.Width(y); x += 1 {
				energyMap.Set(Point{x,y}, Object{MarkerIdle})
			}
		}
	}

	var seenBeams = sets.Set[Beam]{}
	var currentBeams = append([]Beam(nil), start)
	for len(currentBeams) > 0 {
		var c = currentBeams
		currentBeams = make([]Beam, 0)

		for _, beam := range c {
			if seenBeams.Has(beam) {
				continue
			}

			sets.Insert(seenBeams, beam)
			energyMap.Set(beam.Pos, Object{MarkerEnergized})
			currentBeams = append(currentBeams, updateBeam(contraption, beam)...)
		}
	}
	aoc.LogTrace("energy map:\n%v", energyMap.FmtDebug(FmtObject))

	for y := 0; y < energyMap.Height(); y += 1 {
		for x := 0; x < energyMap.Width(y); x += 1 {
			if energyMap.Get(Point{x,y}).Type == MarkerEnergized {
				result += 1
			}
		}
	}

	return result
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var contraption Grid = aoc.MakeGrid[Object]()

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)

		var row = make([]Object, 0)
		for _, r := range t {
			var object = ParseObject(r)
			row = append(row, object)
		}
		contraption.AddRow(contraption.Height(), row)
	}
	aoc.LogTrace("contraption:\n%v", contraption.FmtDebug(FmtObject))

	result = computeEnergy(contraption, Beam{Point{0, 0}, Point{1,0}})
	for y := 0; y < contraption.Height(); y += 1 {
		var awayFromY Point
		if y == 0 {
			awayFromY = Point{0, 1}
		} else if y == contraption.Height() - 1 {
			awayFromY = Point{0, -1}
		}
		
		for x := 0; x < contraption.Width(y); x += 1 {
			var awayFromX Point
			if x == 0 {
				awayFromX = Point{1, 0}
			} else if x == contraption.Width(y) - 1 {
				awayFromX = Point{-1, 0}
			}

			if awayFromX.LenSquared() > 0 {
				result2 = aoc.MaxI(
					result2,
					computeEnergy(contraption, Beam{Point{x, y}, awayFromX}),
				)
			}
			if awayFromY.LenSquared() > 0 {
				result2 = aoc.MaxI(
					result2,
					computeEnergy(contraption, Beam{Point{x, y}, awayFromY}),
				)
			}
		}
	}

	fmt.Println(result, result2)
}