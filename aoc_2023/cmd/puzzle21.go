package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"k8s.io/apimachinery/pkg/util/sets"
)

type Point = aoc.PointI2
type Grid = *aoc.Grid[Plot]

const (
	PlotUnknown = 0
	PlotGarden = 1
	PlotRock = 2
	PlotStart = 3
	PlotReachable = 4
)
type Plot struct {
	Type int
}
func ParsePlot(p rune) Plot {
	switch p {
		case '.': return Plot{PlotGarden}
		case '#': return Plot{PlotRock}
		case 'S': return Plot{PlotStart}
		default: panic("invalid plot type")
	}
}
func FmtPlot(p Plot) string {
	var result rune
	switch p.Type {
		case PlotGarden: result = '.'
		case PlotRock: result = '#'
		case PlotStart: result = 'S'
		case PlotReachable: result = 'O'
		default: result = rune(p.Type)
	}

	return string(result)
}

var STEP_DIRECTIONS = append(
	[]Point(nil),
	Point{0,-1},
	Point{1,0},
	Point{0,1},
	Point{-1,0},
)
func wrapCoord(value int, min int, max int) int {
	var width = max - min
	for value < min {
		value += width
	}

	return (value - min) % width + min
}
func solve(garden Grid, start Point, maxSteps int, wrap bool) int {
	var result = 0

	// find all reachable points from start
	var reachable = sets.Set[Point]{}
	
	var current = make([]Point, 0)
	if maxSteps % 2 == 0 {
		current = append(current, start)
	} else {
		for _, d := range STEP_DIRECTIONS {
			var next = start.Add(d)
			if garden.Get(next).Type == PlotGarden {
				current = append(current, next)
			}
		}
	}
	for i := 0; i < maxSteps; i += 1 {
		var currentNow = current
		current = nil
		for _, c := range currentNow {
			for _, d := range STEP_DIRECTIONS {
				var next = c.Add(d)
				var nextWrapped = next
				if wrap {
					nextWrapped = Point{
						wrapCoord(next.X, 0, garden.Width(0)),
						wrapCoord(next.Y, 0, garden.Height()),
					}
				}

				if garden.Get(nextWrapped).Type == PlotGarden && !reachable.Has(next) && next.DistanceManhattan(start) <= maxSteps {
					reachable.Insert(next)
					current = append(current, next)
				}
			}
		}
	}

	// count 'em up, but filter by distance
	var gardenTrace = garden
	if aoc.LogEnabled(aoc.LogLevelTrace) {
		gardenTrace = garden.Clone()
	}
	for _, seen := range reachable.UnsortedList() {
		var distanceFromStart = start.DistanceManhattan(seen)
		if distanceFromStart % 2 == maxSteps % 2 {
			result += 1

			if aoc.LogEnabled(aoc.LogLevelTrace) {
				gardenTrace.Set(seen, Plot{PlotReachable})
			}
		}
	}
	if aoc.LogEnabled(aoc.LogLevelTrace) {
		aoc.LogTrace("garden reachable:\n%v", gardenTrace.FmtDebug(FmtPlot))
	}

	aoc.LogInfo("result %d steps: %d", maxSteps, result)
	return result
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var garden Grid = aoc.MakeGrid[Plot]()

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)

		var row = make([]Plot, 0)
		for _, r := range t {
			var plot = ParsePlot(r)
			row = append(row, plot)
		}
		garden.AddRow(garden.Height(), row)
	}
	aoc.LogTrace("garden:\n%v", garden.FmtDebug(FmtPlot))
	aoc.LogDebug("garden size: %vx%v\n", garden.Width(0), garden.Height())

	// get start and replace it with normal garden
	var start = Point{-1,-1}
	for y := 0; y < garden.Height(); y += 1 {
		for x := 0; x < garden.Width(y); x += 1 {
			var p = Point{x,y}
			if garden.Get(p).Type == PlotStart {
				start = p
				garden.Set(p, Plot{PlotGarden})
				break
			}
		}
		if start.X != -1 && start.Y != -1 { break }
	}
	aoc.LogDebug("start: %v\n", start)

	// aoc.LogDebug("example in 6: %v\n", solve(garden, start, 6, false))
	// aoc.LogDebug("example in 10: %v\n", solve(garden, start, 10, true))
	// aoc.LogDebug("example in 50: %v\n", solve(garden, start, 50, true))
	// aoc.LogDebug("example in 100: %v\n", solve(garden, start, 100, true))
	// aoc.LogDebug("example in 500: %v\n", solve(garden, start, 500, true))
	// aoc.LogDebug("example in 1000: %v\n", solve(garden, start, 1000, true))
	// aoc.LogDebug("example in 5000: %v\n", solve(garden, start, 5000, true))
	result = solve(garden, start, 64, true)

	aoc.LogDebug("p2 in 65+131*0: %v\n", solve(garden, start, 65+131*0, true))
	aoc.LogDebug("p2 in 65+131*1: %v\n", solve(garden, start, 65+131*1, true))
	aoc.LogDebug("p2 in 65+131*2: %v\n", solve(garden, start, 65+131*2, true))
	// something something quadratic fit of these three results and plug in (26501365 - 65) / 131
	// result2 = solve(garden, start, 26501365, true)

	fmt.Println(result, result2)
}
