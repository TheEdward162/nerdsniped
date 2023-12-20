package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
	"strconv"
	"slices"
)

type Point = aoc.PointI2
type Grid = *aoc.Grid[int]

func FmtSparseGroundTile(t int) string {
	return fmt.Sprintf("%5d ", t)
}

func ParseDirection(d rune) Point {
	switch (d) {
		case 'U': return Point{0,-1}
		case 'R': return Point{1,0}
		case 'D': return Point{0,1}
		case 'L': return Point{-1,0}
		default: panic("invalid direction")
	}
}

func ParseColorAsCommand(color string) (Point, int) {
	var count, _ = strconv.ParseInt(color[:5], 16, 0)
	
	var direction = Point{0,0}
	switch color[5] {
		case '0': direction = Point{1,0}
		case '1': direction = Point{0,1}
		case '2': direction = Point{-1,0}
		case '3': direction = Point{0,-1}
		default: panic("invalid hex direction")
	}

	return direction, int(count)
}

type DigCommand struct {
	Direction Point
	Count int
}

func FindGridSize(commands []DigCommand) (Point, Point) {
	var minPoint = Point{0,0}
	var maxPoint = Point{0,0}
	
	var currentPoint = Point{0,0}
	for _, command := range commands {
		currentPoint = currentPoint.Add(command.Direction.Mul(command.Count))

		minPoint = Point{
			aoc.MinI(minPoint.X, currentPoint.X),
			aoc.MinI(minPoint.Y, currentPoint.Y),
		}
		maxPoint = Point{
			aoc.MaxI(maxPoint.X, currentPoint.X),
			aoc.MaxI(maxPoint.Y, currentPoint.Y),
		}
	}

	return minPoint, maxPoint
}

func RunDig(sparseGround Grid, commands []DigCommand, start Point) {
	var current = start
	sparseGround.AddToRow(current, current.X)

	for _, c := range commands {
		aoc.LogTrace("Running command %v\n", c)
		for i := 0; i < c.Count; i += 1 {
			current = current.Add(c.Direction)
			sparseGround.AddToRow(Point{sparseGround.Width(current.Y),current.Y}, current.X)
		}
	}

	for y := 0; y < sparseGround.Height(); y += 1 {
		var row = sparseGround.GetRow(y)
		slices.Sort(row)
		row = slices.Compact(row)
		sparseGround.SetRow(y, row)
	}
}

const (
	DirectionUp = -1
	DirectionNone = 0
	DirectionDown = 1
)
type InsideOutsideTracker struct {
	cameFrom int
	cameFromX int
	isInside bool
	insideStartX int
}
func NewInsideOutsideTracker() *InsideOutsideTracker {
	var t = new(InsideOutsideTracker)
	t.cameFrom = DirectionNone
	t.isInside = false
	return t
}
func (t *InsideOutsideTracker) Update(x int, direction int) int {
	if direction != DirectionUp && direction != DirectionDown { panic("invalid direction") }

	switch {
		case t.cameFrom == DirectionNone: {
			t.cameFrom = direction
			t.cameFromX = x
			return 0
		}
		case t.cameFrom == direction: {
			t.cameFrom = DirectionNone
			if t.isInside {
				// correct for border crossing inside
				return -(x - t.cameFromX + 1)
			} else {
				return 0
			}
		}
		default: {
			t.cameFrom = DirectionNone
			if t.isInside {
				// leaving inside
				t.isInside = false
				return t.cameFromX - t.insideStartX
			} else {
				// entering inside
				t.isInside = true
				t.insideStartX = x + 1
				return 0
			}
		}
	}
}
func (t *InsideOutsideTracker) End(x int) int {
	if t.isInside {
		return x - t.insideStartX
	}
	return 0
}

func CountDug(sparseGround Grid, width int) int {
	var result = 0

	aoc.LogDebug("Counting\n")
	for y := 0; y < sparseGround.Height(); y += 1 {
		// aoc.LogTrace("Counting row %d\n", y)
		var tracker = NewInsideOutsideTracker()

		for x := 0; x < sparseGround.Width(y); x += 1 {
			var currentX = sparseGround.Get(Point{x,y})
			if slices.Index(sparseGround.GetRow(y - 1), currentX) != -1 {
				// there is a border tile above
				result += tracker.Update(currentX, DirectionUp)
			}
			if slices.Index(sparseGround.GetRow(y + 1), currentX) != -1 {
				// there is a border tile below
				result += tracker.Update(currentX, DirectionDown)
			}

			result += 1
		}

		result += tracker.End(width)
	}

	return result
}

func solve(commands []DigCommand) int {
	var gridMinPoint, gridMaxPoint = FindGridSize(commands)
	aoc.LogDebug("gridMinPoint: %v, gridMaxPoint: %v\n", gridMinPoint, gridMaxPoint)
	var gridSize = gridMaxPoint.Sub(gridMinPoint).Add(Point{1,1})
	aoc.LogDebug("gridSize: %v\n", gridSize)

	var sparseGround = aoc.MakeGrid[int]()
	for y := 0; y < gridSize.Y; y += 1 {
		sparseGround.AddRow(y, nil)
	}

	RunDig(sparseGround, commands, gridMinPoint.Mul(-1))
	// aoc.LogTrace("ground borders:\n%v", sparseGround.FmtDebug(FmtSparseGroundTile))

	return CountDug(sparseGround, gridSize.X)
}

func solveShoelace(commands []DigCommand) int {
	var edges = make([][2]Point, 0, len(commands))
	var current = Point{0,0}
	for i, c := range commands {
		// if the next command continues in the same direction or goes 90deg counter-clockwise we only take Count steps
		// otherwise we take Count+1 steps to stay on the outer perimeter
		var correctForOutsidePerimeter = 0
		var previousCommand = commands[(i - 1 + len(commands)) % len(commands)]
		if !(previousCommand.Direction == c.Direction || previousCommand.Direction.RotNeg90() == c.Direction) {
			correctForOutsidePerimeter += 1
		}
		var nextCommand = commands[(i + 1) % len(commands)]
		if !(c.Direction == nextCommand.Direction || c.Direction.RotNeg90() == nextCommand.Direction) {
			correctForOutsidePerimeter += 1
		}

		var end = current.Add(c.Direction.Mul(c.Count + correctForOutsidePerimeter - 1))
		edges = append(edges, [2]Point{current,end})
		current = end
	}

	aoc.LogTrace("shoelace edges %v\n", edges)
	return aoc.ShoelaceArea(edges)
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var commands = make([]DigCommand, 0)
	var commands2 = make([]DigCommand, 0)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)

		var parts = strings.Split(t, " ")
		var direction = ParseDirection(rune(parts[0][0]))
		var count, _ = strconv.Atoi(parts[1])
		commands = append(commands, DigCommand{direction,count})

		var color = strings.Trim(parts[2], "(#)")
		var direction2, count2 = ParseColorAsCommand(color)
		commands2 = append(commands2, DigCommand{direction2, count2})
	}
	aoc.LogTrace("commands: %v\n", commands)
	aoc.LogTrace("commands2: %v\n", commands2)

	// result = solve(commands)
	// result2 = solve(commands2)

	result = solveShoelace(commands)
	result2 = solveShoelace(commands2)

	fmt.Println(result, result2)
}
