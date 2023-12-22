package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
	"slices"
	"k8s.io/apimachinery/pkg/util/sets"
)

type Point = aoc.PointI3
type Point2 = aoc.PointI2
type Range = aoc.Range[int]

type Brick struct {
	// left near botttom
	lnb Point
	// right far top
	rft Point
}
func (b Brick) LeftRightRange() Range {
	return Range{b.lnb.X,b.rft.X+1}
}
func (b Brick) NearFarRange() Range {
	return Range{b.lnb.Y,b.rft.Y+1}
}
func (b Brick) BottomTopRange() Range {
	return Range{b.lnb.Z,b.rft.Z+1}
}
func BrickIntersects(a Brick, b Brick) bool {
	if !a.LeftRightRange().Intersects(b.LeftRightRange()) { return false }
	if !a.NearFarRange().Intersects(b.NearFarRange()) { return false }
	if !a.BottomTopRange().Intersects(b.BottomTopRange()) { return false }

	return true
}
func (b Brick) FallBrick() Brick {
	var down = Point{0,0,-1}
	return Brick{
		b.lnb.Add(down),
		b.rft.Add(down),
	}
}


func SortBricksZAscending(a Brick, b Brick) int {
	return a.rft.Z - b.rft.Z
}
/// returns indices of all bricks blocking brick at i
func findAllBlocking(bricks []Brick, i int) []int {
	var result = make([]int, 0)

	var checkedBrick = bricks[i]
	// early return for a brick resting on the ground
	if checkedBrick.lnb.Z == 1 {
		result = append(result, -1)
		return result
	}

	// construct checking brick
	var collisionBrick = Brick{
		Point{checkedBrick.lnb.X,checkedBrick.lnb.Y,checkedBrick.lnb.Z-1},
		Point{checkedBrick.rft.X,checkedBrick.rft.Y,checkedBrick.lnb.Z-1},
	}

	// assuming sorted from beginning
	var potential = bricks[:i]
	for pi, p := range potential {
		if BrickIntersects(collisionBrick, p) {
			result = append(result, pi)
		}
	}

	return result
}
func fallBricks(bricks []Brick) {
	for {
		var changed = false
		// find lowest brick that can be fallen
		for i, _ := range bricks {
			var blocking = findAllBlocking(bricks, i)
			if len(blocking) == 0 {
				bricks[i] = bricks[i].FallBrick()
				changed = true
			}
		}

		if !changed {
			break
		}
	}
}

type BrickState struct {
	Index int
	Supports sets.Set[int]
	SupportedBy sets.Set[int]
}
func (b *BrickState) Clone() *BrickState {
	var s = new(BrickState)
	s.Index = b.Index
	s.Supports = b.Supports.Clone()
	s.SupportedBy = b.SupportedBy.Clone()
	return s
}
func EnsureBrickState(m map[int]*BrickState, i int) {
	var _, ok = m[i]
	if !ok {
		m[i] = &BrickState{i,sets.Set[int]{},sets.Set[int]{}}
	}
}

func SimulateRemoved(m map[int]*BrickState, i int) int {
	var states = make(map[int]*BrickState)
	for k, v := range m {
		states[k] = v.Clone()
	}

	var result = -1
	var current = append([]int(nil), i)
	for len(current) > 0 {
		var c = current[0]
		current = current[1:]
		
		result += 1
		// simulate chain-reaction removal
		for _, directlySupported := range states[c].Supports.UnsortedList() {
			states[directlySupported].SupportedBy.Delete(c)
			if states[directlySupported].SupportedBy.Len() == 0 {
				current = append(current, directlySupported)
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

	var bricks = make([]Brick, 0)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)

		var parts = strings.Split(t, "~")
		var lnb = aoc.ParseIntListBy(parts[0], ",")
		var rft = aoc.ParseIntListBy(parts[1], ",")
		bricks = append(bricks, Brick{Point{lnb[0],lnb[1],lnb[2]}, Point{rft[0],rft[1],rft[2]}})
	}
	// when we sort by top Z we can rule out all bricks with top Z above it when trying to fall it
	// this holds even once we start falling them, because if a brick cannot fall below another brick and block it if
	// it doesn't start in that way
	slices.SortFunc(bricks, SortBricksZAscending)
	aoc.LogTrace("bricks: %v\n", bricks)

	fallBricks(bricks)
	aoc.LogTrace("bricks fallen: %v\n", bricks)

	var brickStates = make(map[int]*BrickState)
	for i, _ := range bricks {
		EnsureBrickState(brickStates, i)

		var blocking = findAllBlocking(bricks, i)
		brickStates[i].SupportedBy.Insert(blocking...)
		for _, bi := range blocking {
			if bi < 0 { continue }
			EnsureBrickState(brickStates, bi)
			brickStates[bi].Supports.Insert(i)
		}
	}
	aoc.LogTrace("brick states: %+v\n", brickStates)

	var loadBearingBricks = sets.Set[int]{}
	for _, s := range brickStates {
		if s.SupportedBy.Len() == 1 && !s.SupportedBy.Has(-1) {
			loadBearingBricks.Insert(s.SupportedBy.UnsortedList()...)
		}
	}
	result = len(bricks) - loadBearingBricks.Len()

	for i, _ := range brickStates {
		result2 += SimulateRemoved(brickStates, i)
	}

	fmt.Println(result, result2)
}
