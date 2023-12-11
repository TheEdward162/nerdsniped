package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
)

type Point = aoc.PointI2
type Grid = *aoc.Grid[SkyObject]

const (
	ObjectSpace = 0
	ObjectGalaxy = 1
)
type SkyObject struct {
	Type int
}
func ParseSkyObject(object rune) SkyObject {
	switch object {
		case '.': return SkyObject{ObjectSpace}
		case '#': return SkyObject{ObjectGalaxy}
		default: panic("invalid object")
	}
}
func PrintSkyObject(object SkyObject) string {
	var result rune

	switch object.Type {
		case ObjectSpace: result = '.'
		case ObjectGalaxy: result = '#'
		default: result = rune(object.Type)
	}

	return string(result)
}

func DebugSky(sky Grid) {
	if aoc.LogEnabled(aoc.LogLevelDebug) {
		aoc.LogDebug(
			"sky:\n%s",
			sky.FmtDebug(PrintSkyObject),
		)
	}
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var sky Grid = aoc.MakeGrid[SkyObject]()

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)
		
		var row = make([]SkyObject, 0)
		for _, r := range t {
			var object = ParseSkyObject(r)
			row = append(row, object)
		}
		sky.AddRow(sky.Height(), row)
	}

	// find galaxies
	var galaxies = make([]Point, 0)
	for y := 0; y < sky.Height(); y += 1 {
		for x := 0; x < sky.Width(y); x += 1 {
			var p = Point{x,y}
			if sky.Get(p).Type == ObjectGalaxy {
				galaxies = append(galaxies, p)
			}
		}
	}
	aoc.LogTrace("galaxies = %v\n", galaxies)
	var galaxies2 = append([]Point(nil), galaxies...)

	// SPACE ELONGATION
	// add rows
	for y := sky.Height() - 1; y >= 0; y -= 1 {
		var allSky = true
		for x := 0; x < sky.Width(y); x += 1 {
			if sky.Get(Point{x,y}).Type != ObjectSpace {
				allSky = false
				break
			}
		}

		if allSky {
			aoc.LogTrace("empty row at %d\n", y)
			
			for i := range galaxies {
				if galaxies[i].Y > y { galaxies[i].Y += 1 }
				if galaxies2[i].Y > y { galaxies2[i].Y += 1_000_000-1 }
			}
		}
	}
	// add columns
	for x := sky.Width(0) - 1; x >= 0; x -= 1 {
		var allSky = true
		for y := 0; y < sky.Height(); y += 1 {
			if sky.Get(Point{x,y}).Type != ObjectSpace {
				allSky = false
				break
			}
		}

		if allSky {
			aoc.LogTrace("empty column at %d\n", x)

			for i := range galaxies {
				if galaxies[i].X > x { galaxies[i].X += 1 }
				if galaxies2[i].X > x { galaxies2[i].X += 1_000_000-1 }
			}
		}
	}
	DebugSky(sky)
	aoc.LogTrace("galaxies = %v\n", galaxies)
	aoc.LogTrace("galaxies2 = %v\n", galaxies2)

	for i := 0; i < len(galaxies); i += 1 {
		for j := i; j < len(galaxies); j += 1 {
			result += galaxies[i].DistanceManhattan(galaxies[j])
			result2 += galaxies2[i].DistanceManhattan(galaxies2[j])
		}
	}

	fmt.Println(result, result2)
}