package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"k8s.io/apimachinery/pkg/util/sets"
)

type Point = aoc.PointI2
type Grid = *aoc.Grid[Tile]

const (
	// .
	TileGround = 0
	// |
	TilePipeNS = 1
	// -
	TilePipeEW = 2
	// L
	TilePipeNE = 3
	// J
	TilePipeNW = 4
	// 7
	TilePipeSW = 5
	// F
	TilePipeSE = 6
	// S
	TileStart = 7
)

const (
	DirectionNone = -1
	DirectionNorth = 0
	DirectionEast = 1
	DirectionSouth = 2
	DirectionWest = 3
)
func MoveDir(p Point, direction int) Point {
	switch direction {
		case DirectionNorth: return Point{p.X, p.Y - 1}
		case DirectionEast: return Point{p.X + 1, p.Y}
		case DirectionSouth: return Point{p.X, p.Y + 1}
		case DirectionWest: return Point{p.X - 1, p.Y}
		default: panic("invalid direction")
	}
}

type Tile struct {
	Type int
}
func ParseTile(tile rune) Tile {
	switch tile {
		case '.': return Tile{TileGround}
		case '|': return Tile{TilePipeNS}
		case '-': return Tile{TilePipeEW}
		case 'L': return Tile{TilePipeNE}
		case 'J': return Tile{TilePipeNW}
		case '7': return Tile{TilePipeSW}
		case 'F': return Tile{TilePipeSE}
		case 'S': return Tile{TileStart}
		default: panic("invalid tile")
	}
}
func PrintTile(tile Tile) string {
	var result rune

	switch tile.Type {
		case TilePipeNS: result = '┃' // '|'
		case TilePipeEW: result = '━' // '-'
		case TilePipeNE: result = '┗' // 'L'
		case TilePipeNW: result = '┛' // 'J'
		case TilePipeSW: result = '┓' // '7'
		case TilePipeSE: result = '┏' // 'F'
		case TileGround: result = '.'
		case TileStart: result = 'S'
		default: result = rune(tile.Type)
	}

	return string(result)
}
func (t Tile) ConnectsTo(direction int) bool {
	switch direction {
		case DirectionNorth: return t.Type == TilePipeNS || t.Type == TilePipeNE || t.Type == TilePipeNW
		case DirectionEast: return t.Type == TilePipeEW || t.Type == TilePipeNE || t.Type == TilePipeSE
		case DirectionSouth: return t.Type == TilePipeNS || t.Type == TilePipeSE || t.Type == TilePipeSW
		case DirectionWest: return t.Type == TilePipeEW || t.Type == TilePipeNW || t.Type == TilePipeSW
		default: panic("invalid direction")
	}
}

func CleanupTile(ground Grid, p Point) bool {
	var anyChanges = false
	
	// north, east, south, west
	var around = [4]Tile{
		ground.Get(MoveDir(p, DirectionNorth)),
		ground.Get(MoveDir(p, DirectionEast)),
		ground.Get(MoveDir(p, DirectionSouth)),
		ground.Get(MoveDir(p, DirectionWest)),
	}

	// find start tile type
	if ground.Get(p).Type == TileStart {
		// compute start tile orientation
		var north = around[DirectionNorth].ConnectsTo(DirectionSouth)
		var east = around[DirectionEast].ConnectsTo(DirectionWest)
		var south = around[DirectionSouth].ConnectsTo(DirectionNorth)
		var west = around[DirectionWest].ConnectsTo(DirectionEast)

		var startType int
		switch {
			case north && south: startType = TilePipeNS
			case north && east: startType = TilePipeNE
			case north && west: startType = TilePipeNW
			case south && east: startType = TilePipeSE
			case south && west: startType = TilePipeSW
			case east && west: startType = TilePipeEW
		}
		ground.Set(p, Tile{startType})
		anyChanges = true
		aoc.LogDebug("start type = %v\n", startType)
	} else {
		// clean up tiles which are clearly not a loop
		var northMismatch = ground.Get(p).ConnectsTo(DirectionNorth) && !around[DirectionNorth].ConnectsTo(DirectionSouth)
		var eastMismatch = ground.Get(p).ConnectsTo(DirectionEast) && !around[DirectionEast].ConnectsTo(DirectionWest)
		var southMismatch = ground.Get(p).ConnectsTo(DirectionSouth) && !around[DirectionSouth].ConnectsTo(DirectionNorth)
		var westMismatch = ground.Get(p).ConnectsTo(DirectionWest) && !around[DirectionWest].ConnectsTo(DirectionEast)
		if northMismatch || eastMismatch || southMismatch || westMismatch {
			ground.Set(p, Tile{TileGround})
			anyChanges = true
		}
	}

	return anyChanges
}

func DebugGround(ground Grid) {
	if aoc.LogEnabled(aoc.LogLevelDebug) {
		aoc.LogDebug(
			"ground:\n%s",
			ground.FmtDebug(PrintTile),
		)
	}
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var start Point
	var ground Grid = aoc.MakeGrid[Tile]()

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)
		
		var row = make([]Tile, 0)
		for x, r := range t {
			var tile = ParseTile(r)
			if tile.Type == TileStart {
				start = Point{x, ground.Height()}
			}
			row = append(row, tile)
		}
		ground.AddRow(ground.Height(), row)
	}
	aoc.LogInfo("start = %v\n", start)

	// post process
	CleanupTile(ground, start)
	// turns out we don't need this, but it was fun anyway
	for {
		var anyChanges = false
		for y := 0; y < ground.Height(); y += 1 {
			for x := 0; x < ground.Width(y); x += 1 {
				anyChanges = anyChanges || CleanupTile(ground, Point{x, y})
			}
		}

		if !anyChanges { break }
	}
	DebugGround(ground)

	// bfs
	var seen = sets.Set[Point]{}
	sets.Insert(seen, start)
	var active = [2]Point{start, start}
	for {
		aoc.LogTrace("bfs active = %v\n", active)

		var updated = false
		for i := 0; i < len(active); i += 1 {
			for d := DirectionNorth; d <= DirectionWest; d += 1 {
				var nextP = MoveDir(active[i], d)
				if ground.Get(active[i]).ConnectsTo(d) && !seen.Has(nextP) {
					active[i] = nextP
					updated = true
					break
				}
			}

			sets.Insert(seen, active[i])
		}
		if !updated { break }

		result += 1
	}

	for y := 0; y < ground.Height(); y += 1 {
		var rowCameFrom = DirectionNone
		var outside = true
		for x := 0; x < ground.Width(y); x += 1 {
			var p = Point{x, y}
			var tile = ground.Get(p)

			if seen.Has(p) {
				switch tile.Type {
					// enter and leave
					case TilePipeNS: {
						outside = !outside
					}
					// enter
					case TilePipeNE: {
						rowCameFrom = DirectionNorth
					}
					case TilePipeSE: {
						rowCameFrom = DirectionSouth
					}
					// leave
					case TilePipeNW: {
						if rowCameFrom == DirectionSouth {
							outside = !outside
						}
						rowCameFrom = DirectionNone
					}
					case TilePipeSW: {
						if rowCameFrom == DirectionNorth {
							outside = !outside
						}
						rowCameFrom = DirectionNone
					}
					// hold
					case TilePipeEW: {}
				}
			} else {
				if !outside {
					result2 += 1
					ground.Set(Point{x, y}, Tile{'.'})
				} else {
					ground.Set(Point{x, y}, Tile{' '})
				}
			}
		}
	}
	DebugGround(ground)

	fmt.Println(result, result2)
}