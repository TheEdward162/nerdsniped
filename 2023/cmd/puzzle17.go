package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"math"
	"strconv"
	"container/heap"
	"k8s.io/apimachinery/pkg/util/sets"
)

type Point = aoc.PointI2
type Grid = *aoc.Grid[District]

type District struct {
	HeatLoss int
}
func ParseDistrict(d rune) District {
	var loss, _ = strconv.Atoi(string(d))
	return District{loss}
}
func FmtDistrict(d District) string {
	return strconv.Itoa(d.HeatLoss)
}

type Candidate struct {
	Pos Point
	Dir Point
	ForwardCount int
	TotalLoss int
}
func MakeCandidate(p Point, d Point) Candidate {
	return Candidate{p, d, 0, 0}
}
func NextCandidates(grid Grid, c Candidate, minForwardCount int, maxForwardCount int) []Candidate {
	var r = make([]Candidate, 0)
	if c.ForwardCount < maxForwardCount {
		var forwardPos = c.Pos.Add(c.Dir)
		if grid.Has(forwardPos) {
			var nextLoss = c.TotalLoss + grid.Get(forwardPos).HeatLoss
			r = append(r, Candidate{forwardPos, c.Dir, c.ForwardCount + 1, nextLoss})
		}
	}

	if c.ForwardCount >= minForwardCount {
		var rightDir = c.Dir.Rot90()
		var rightPos = c.Pos.Add(rightDir)
		if grid.Has(rightPos) {
			var nextLoss = c.TotalLoss + grid.Get(rightPos).HeatLoss
			r = append(r, Candidate{rightPos, rightDir, 1, nextLoss})
		}

		var leftDir = c.Dir.RotNeg90()
		var leftPos = c.Pos.Add(leftDir)
		if grid.Has(leftPos) {
			var nextLoss = c.TotalLoss + grid.Get(leftPos).HeatLoss
			r = append(r, Candidate{leftPos, leftDir, 1, nextLoss})
		}
	}

	return r
}

type CandidateSeen struct {
	Pos Point
	Dir Point
	ForwardCount int
}
func ToSeen(c Candidate) CandidateSeen {
	return CandidateSeen{c.Pos,c.Dir,c.ForwardCount}
}

func PrioBfs(city Grid, start Point, target Point, minForwardCount int, maxForwardCount int) int {
	var seen = sets.Set[CandidateSeen]{}

	var current = make(PriorityQueue, 0)
	heap.Init(&current)
	heap.Push(&current, &Item{MakeCandidate(start, Point{1, 0}),0})

	for current.Len() > 0 {
		var c = heap.Pop(&current).(*Item).value
		if seen.Has(ToSeen(c)) {
			continue
		}
		aoc.LogDebug("current: %v\n", c)
		seen.Insert(ToSeen(c))

		for _, next := range NextCandidates(city, c, minForwardCount, maxForwardCount) {
			if next.Pos == target && next.ForwardCount >= minForwardCount {
				return next.TotalLoss
			} else {
				if !seen.Has(ToSeen(next)) {
					heap.Push(&current, &Item{next, 0})
					aoc.LogTrace("candidate %+v\n", next)
				}
			}
		}
	}

	return math.MaxInt32
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = math.MaxInt32
	var result2 = 0

	var city = aoc.MakeGrid[District]()

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)

		var row = make([]District, 0)
		for _, r := range t {
			var d = ParseDistrict(r)
			row = append(row, d)
		}
		city.AddRow(city.Height(), row)
	}
	aoc.LogTrace("city:\n%v", city.FmtDebug(FmtDistrict))

	result = PrioBfs(
		city,
		Point{0,0},
		Point{city.Width(0)-1,city.Height()-1},
		0,
		3,
	)

	result2 = PrioBfs(
		city,
		Point{0,0},
		Point{city.Width(0)-1,city.Height()-1},
		4,
		10,
	)

	fmt.Println(result, result2)
}


// lamo suck my dick go
type Item struct {
	value Candidate
	index int
}
type PriorityQueue []*Item
func (pq PriorityQueue) Len() int { return len(pq) }
func (pq PriorityQueue) Less(i, j int) bool {
	var a = pq[i].value
	var b = pq[j].value

	if a.TotalLoss < b.TotalLoss {
		return true
	}

	return false
}
func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}
func (pq *PriorityQueue) Push(x any) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}
func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.index = -1
	*pq = old[0 : n-1]
	return item
}
