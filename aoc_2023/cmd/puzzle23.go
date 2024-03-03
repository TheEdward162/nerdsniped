package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"k8s.io/apimachinery/pkg/util/sets"
	"sync"
	"sync/atomic"
)

type Point = aoc.PointI2
type Grid = *aoc.Grid[Trail]

var STEP_DIRECTIONS = append(
	[]Point(nil),
	Point{0,-1},
	Point{1,0},
	Point{0,1},
	Point{-1,0},
)

const (
	TrailUnknown = 0
	TrailPath = 1
	TrailForest = 2
	TrailSlopeNorth = 3
	TrailSlopeEast = 4
	TrailSlopeSouth = 5
	TrailSlopeWest = 6
	TrailMarkStart = 7
	TrailMarkVisited = 8
)
type Trail struct {
	Type int
}
func ParseTrail(t rune) Trail {
	switch t {
		case '.': return Trail{TrailPath}
		case '#': return Trail{TrailForest}
		case '^': return Trail{TrailSlopeNorth}
		case '>': return Trail{TrailSlopeEast}
		case 'v': return Trail{TrailSlopeSouth}
		case '<': return Trail{TrailSlopeWest}
		default: panic("invalid trail")
	}
}
func FmtTrail(t Trail) string {
	var result rune

	switch t.Type {
		case TrailPath: result = '.'
		case TrailForest: result = '#'
		case TrailSlopeNorth: result = '^'
		case TrailSlopeEast: result = '>'
		case TrailSlopeSouth: result = 'v'
		case TrailSlopeWest: result = '<'
		case TrailMarkStart: result = 'S'
		case TrailMarkVisited: result = 'O'
		default: result = rune(t.Type)
	}

	return string(result)
}

func nextSteps(trails Grid, current Point, respectSlopes bool, seen sets.Set[Point]) []Point {
	var result = make([]Point, 0)
	var directions = STEP_DIRECTIONS
	
	var currentType = trails.Get(current).Type
	if currentType == TrailForest {
		return result
	}

	if respectSlopes {
		switch currentType {
			case TrailSlopeNorth: directions = directions[0:1]
			case TrailSlopeEast: directions = directions[1:2]
			case TrailSlopeSouth: directions = directions[2:3]
			case TrailSlopeWest: directions = directions[3:4]
			default: {}
		}
	}

	for _, d := range directions {
		var next = current.Add(d)
		if seen.Has(next) { continue }

		switch trails.Get(next).Type {
			case TrailPath, TrailSlopeNorth, TrailSlopeEast, TrailSlopeSouth, TrailSlopeWest: {
				result = append(result, next)
			}
		}
	}

	return result
}
func findAllIntersections(trails Grid) []Point {
	var result = make([]Point, 0)
	var emptySet = sets.Set[Point]{}

	for y := 0; y < trails.Height(); y += 1 {
		for x := 0; x < trails.Width(y); x += 1 {
			var current = Point{x,y}

			var possibleSteps = nextSteps(trails, current, false, emptySet)
			if len(possibleSteps) > 2 {
				aoc.LogTrace("found intersection %v (%v)\n", current, possibleSteps)
				result = append(result, current)
			}
		}
	}

	return result
}
type TrialEdge struct {
	From Point
	To Point
	/// How many steps it costs to take this edge
	/// An edge goes from a center of one intersection to the center of another intersection
	Len int
}
func findEdge(trails Grid, edgeFrom Point, edgeFirstStep Point, nodes sets.Set[Point], respectSlopes bool) (TrialEdge, bool) {
	aoc.LogTrace("finding edge from node %v, first step %v\n", edgeFrom, edgeFirstStep)
	var edge TrialEdge
	edge.From = edgeFrom

	var seen = sets.Set[Point]{}
	seen.Insert(edgeFrom)
	var current = edgeFirstStep
	var stepCount = 0
	for {
		seen.Insert(current)
		stepCount += 1

		var nexts = nextSteps(trails, current, respectSlopes, seen)
		if len(nexts) == 0 {
			return edge, false
		}
		if len(nexts) > 1 {
			aoc.LogError("multiple nexts: %v from: %v seen: %v\n", nexts, current, seen)
			panic("multiple nexts")
		}
		if nodes.Has(nexts[0]) {
			edge.To = nexts[0]
			edge.Len = stepCount + 1 // plus one for this next, which hasn't been accounted for yet
			break
		}
		current = nexts[0]
	}

	return edge, true
}
/// Returns a mapping from nodes to edges connected to that node.
func findAllEdges(trails Grid, start Point, end Point, respectSlopes bool) map[Point][]TrialEdge {
	var edges = make(map[Point][]TrialEdge)

	var nodes sets.Set[Point] = sets.New(findAllIntersections(trails)...)
	nodes.Insert(start)
	nodes.Insert(end)
	var emptySet = sets.Set[Point]{}

	for _, node := range nodes.UnsortedList() {
		edges[node] = make([]TrialEdge, 0)
		for _, edgeFirstStep := range nextSteps(trails, node, false, emptySet) {
			var edge, wasFound = findEdge(trails, node, edgeFirstStep, nodes, respectSlopes)
			if wasFound {
				edges[node] = append(edges[node], edge)
			}
		}
	}

	return edges
}

type SolveState struct {
	Pos Point
	Seen sets.Set[Point]
	Len int
}
func solve(edges map[Point][]TrialEdge, start Point, end Point) int {
	var longestPath = 0

	var current = append([]SolveState(nil), SolveState{start,sets.Set[Point]{},0})
	for len(current) > 0 {
		var c = current[0]
		current = current[1:]
		c.Seen.Insert(c.Pos)

		if c.Pos == end {
			if c.Len > longestPath {
				aoc.LogDebug("Found path %d\n", c.Len)
			}
			longestPath = aoc.MaxI(longestPath, c.Len)
			continue
		}

		for _, edge := range edges[c.Pos] {
			if !c.Seen.Has(edge.To) {
				var nextSeen = c.Seen.Clone()
				var next = SolveState{edge.To,nextSeen,c.Len + edge.Len}
				current = append(current, next)
			}
		}
	}

	return longestPath
}

type ConcurrentSolver struct {
	edges map[Point][]TrialEdge
	start Point
	end Point
	wg sync.WaitGroup
	longestPath atomic.Int64
}
func (s *ConcurrentSolver) solveTask(task SolveState) {
	task.Seen.Insert(task.Pos)

	if task.Pos == s.end {
		var newLongestPath = int64(task.Len)
		var longestPath = s.longestPath.Load()

		for newLongestPath > longestPath {
			if s.longestPath.CompareAndSwap(longestPath, newLongestPath) {
				aoc.LogDebug("Found path %d (old %d)\n", newLongestPath, longestPath)
				break
			}
			longestPath = s.longestPath.Load()
		}
		return
	}

	for _, edge := range s.edges[task.Pos] {
		if !task.Seen.Has(edge.To) {
			var nextSeen = task.Seen.Clone()
			var nextTask = SolveState{edge.To,nextSeen,task.Len + edge.Len}
			
			s.wg.Add(1)
			go func() {
				defer s.wg.Done()
				s.solveTask(nextTask)
			}()
		}
	}
}
func solveConcurrent(edges map[Point][]TrialEdge, start Point, end Point) int {
	var solver = ConcurrentSolver {
		edges,
		start,
		end,
		sync.WaitGroup{},
		atomic.Int64{},
	}

	solver.solveTask(SolveState{start,sets.Set[Point]{},0})
	solver.wg.Wait()

	return int(solver.longestPath.Load())
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var trails = aoc.MakeGrid[Trail]()
	var start Point
	var end Point

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)

		var row = make([]Trail, 0)
		for _, r := range t {
			var d = ParseTrail(r)
			row = append(row, d)
		}
		trails.AddRow(trails.Height(), row)
	}
	aoc.LogTrace("trails:\n%v", trails.FmtDebug(FmtTrail))

	for x := 0; x <= trails.Width(0); x += 1 {
		var p = Point{x,0}
		if trails.Get(p).Type == TrailPath {
			start = p
			break
		}
	}
	aoc.LogTrace("start: %v\n", start)
	for x := 0; x <= trails.Width(trails.Height() - 1); x += 1 {
		var p = Point{x,trails.Height() - 1}
		if trails.Get(p).Type == TrailPath {
			end = p
			break
		}
	}
	aoc.LogTrace("end: %v\n", end)

	var edges = findAllEdges(trails, start, end, true)
	result = solveConcurrent(edges, start, end)
	aoc.LogInfo("result: %v\n", result)

	var edgesNoSlopes = findAllEdges(trails, start, end, false)
	result2 = solveConcurrent(edgesNoSlopes, start, end)

	fmt.Println(result, result2)
}
