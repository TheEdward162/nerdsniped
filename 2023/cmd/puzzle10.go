package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"k8s.io/apimachinery/pkg/util/sets"
)

type Point = aoc.PointI2

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
func PrintTile(tile Tile) rune {
	switch tile.Type {
		case TilePipeNS: return '┃' // '|'
		case TilePipeEW: return '━' // '-'
		case TilePipeNE: return '┗' // 'L'
		case TilePipeNW: return '┛' // 'J'
		case TilePipeSW: return '┓' // '7'
		case TilePipeSE: return '┏' // 'F'
		case TileGround: return '.'
		case TileStart: return 'S'
		default: return rune(tile.Type)
	}
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

func GetTile(ground [][]Tile, p Point) Tile {
	if p.Y < 0 || p.Y >= len(ground) {
		return Tile{TileGround}
	}

	for p.X < 0 || p.X >= len(ground[p.Y]) {
		return Tile{TileGround}
	}

	return ground[p.Y][p.X]
}
func SetTile(ground [][]Tile, p Point, v Tile) {
	if p.Y < 0 || p.Y >= len(ground) {
		return
	}

	for p.X < 0 || p.X >= len(ground[p.Y]) {
		return
	}

	ground[p.Y][p.X] = v
}

func CleanupTile(ground [][]Tile, p Point) bool {
	var anyChanges = false
	
	// north, east, south, west
	var around = [4]Tile{
		GetTile(ground, MoveDir(p, DirectionNorth)),
		GetTile(ground, MoveDir(p, DirectionEast)),
		GetTile(ground, MoveDir(p, DirectionSouth)),
		GetTile(ground, MoveDir(p, DirectionWest)),
	}

	// find start tile type
	if GetTile(ground, p).Type == TileStart {
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
		SetTile(ground, p, Tile{startType})
		anyChanges = true
		aoc.LogDebug("start type = %v\n", startType)
	} else {
		// clean up tiles which are clearly not a loop
		var northMismatch = GetTile(ground, p).ConnectsTo(DirectionNorth) && !around[DirectionNorth].ConnectsTo(DirectionSouth)
		var eastMismatch = GetTile(ground, p).ConnectsTo(DirectionEast) && !around[DirectionEast].ConnectsTo(DirectionWest)
		var southMismatch = GetTile(ground, p).ConnectsTo(DirectionSouth) && !around[DirectionSouth].ConnectsTo(DirectionNorth)
		var westMismatch = GetTile(ground, p).ConnectsTo(DirectionWest) && !around[DirectionWest].ConnectsTo(DirectionEast)
		if northMismatch || eastMismatch || southMismatch || westMismatch {
			SetTile(ground, p, Tile{TileGround})
			anyChanges = true
		}
	}

	return anyChanges
}

func DebugGround(ground [][]Tile) {
	if aoc.LogEnabled(aoc.LogLevelDebug) {
		aoc.LogDebug("ground:\n")
		for y := 0; y < len(ground); y += 1 {
			var rowDebug string
			for x := 0; x < len(ground[y]); x += 1 {
				rowDebug = rowDebug + string(PrintTile(ground[y][x]))
			}
			aoc.LogDebug("%s\n", rowDebug)
		}
	}
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var start Point
	var ground = make([][]Tile, 0)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)
		
		var row = make([]Tile, 0)
		for x, r := range t {
			var tile = ParseTile(r)
			if tile.Type == TileStart {
				start = Point{x, len(ground)}
			}
			row = append(row, tile)
		}
		ground = append(ground, row)
	}
	aoc.LogInfo("start = %v\n", start)

	// post process
	CleanupTile(ground, start)
	// turns out we don't need this, but it was fun anyway
	for {
		var anyChanges = false
		for y := 0; y < len(ground); y += 1 {
			for x := 0; x < len(ground[y]); x += 1 {
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
				if GetTile(ground, active[i]).ConnectsTo(d) && !seen.Has(nextP) {
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

	// mark seen tiles (mostly for visual effect) and run part 2
	// for t, _ := range seen {
	// 	var tile = GetTile(ground, t)
	// 	if tile.ConnectsTo(DirectionNorth) || tile.ConnectsTo(DirectionSouth) {
	// 		SetTile(ground, t, Tile{TileMarkSwitch})
	// 	} else {
	// 		SetTile(ground, t, Tile{TileMarkHold})
	// 	}
	// }

	for y := 0; y < len(ground); y += 1 {
		var rowCameFrom = DirectionNone
		var outside = true
		for x := 0; x < len(ground[y]); x += 1 {
			var p = Point{x, y}
			var tile = GetTile(ground, p)

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
					SetTile(ground, Point{x, y}, Tile{'.'})
				} else {
					SetTile(ground, Point{x, y}, Tile{' '})
				}
			}
		}
	}
	DebugGround(ground)

	fmt.Println(result, result2)
}