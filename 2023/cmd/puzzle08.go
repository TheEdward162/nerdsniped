package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
	"slices"
)

const (
	MoveLeft = iota
	MoveRight
)
type MoveSequence struct {
	moves string
	offset int
}
func NewMoveSequence(moves string) *MoveSequence {
	var self = new(MoveSequence)
	self.moves = moves
	self.offset = 0
	return self
}
/// Returns number of this move (indexed from 1, periodic) and the move itself (`MoveLeft|MoveRight`)
func (m *MoveSequence) NextMove() (int, int) {
	var i = m.offset
	var move = m.moves[i]
	m.offset = (m.offset + 1) % len(m.moves)

	switch move {
		case 'L': return i, MoveLeft
		case 'R': return i, MoveRight
		default: panic("invalid move")
	}
}

type MapNode struct {
	Name string
	LeftName string
	RightName string
}
func NextNode(node MapNode, move int) string {
	switch move {
		case MoveLeft: return node.LeftName
		case MoveRight: return node.RightName
		default: panic("invalid move")
	}
}

type CycleInfo struct {
	/// Number of moves before the period starts
	Prefix int
	Period int
	/// First-cycle Offsets at which terminals appear
	/// This does include the prefix (for cases where a terminal happens in the prefix stage)
	Terminals []int
}
type HistoryEntry struct {
	Step int
	Node string
}
func FindCycle(nodeMap map[string]MapNode, sequence *MoveSequence, start string) CycleInfo {
	var result = CycleInfo{0, 0, make([]int, 0)}

	// history of visited nodes
	var history = make([]HistoryEntry, 0)
	var currentNode = start
	var totalSteps = 0
	for {
		var step, move = sequence.NextMove()
		currentNode = NextNode(nodeMap[currentNode], move)
		var entry = HistoryEntry{step, currentNode}
		
		var cycleStart = slices.Index(history, entry)
		if cycleStart >= 0 {
			result.Prefix = cycleStart
			result.Period = totalSteps - history[cycleStart].Step
			break
		}
		history = append(history, entry)
		totalSteps += 1
	}

	for _, e := range history {
		if strings.HasSuffix(e.Node, "Z") {
			result.Terminals = append(result.Terminals, e.Step)
		}
	}

	return result
}

type GhostPath struct {
	Cycle CycleInfo
	Start MapNode
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var sequence string
	var nodeMap = make(map[string]MapNode)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)
		
		if sequence == "" {
			sequence = t
			continue
		}

		var parts = strings.Split(t, " = ")
		var name = parts[0]
		parts = strings.Split(strings.Trim(parts[1], "()"), ", ")
		var left, right = parts[0], parts[1]
		nodeMap[name] = MapNode{name, left, right}
	}
	aoc.LogDebug("sequence = %v\n", sequence)
	aoc.LogDebug("nodeMap = %v\n", nodeMap)

	var currentNode = "AAA"
	var currentSequence = NewMoveSequence(sequence)
	for {
		if currentNode == "ZZZ" { break }

		var _, currentMove = currentSequence.NextMove()
		currentNode = NextNode(nodeMap[currentNode], currentMove)

		result += 1
	}
	aoc.LogDebug("result: %d\n", result)

	var ghostPaths = make([]GhostPath, 0)
	for k, n := range nodeMap {
		if strings.HasSuffix(k, "A") {
			var cycle = FindCycle(nodeMap, NewMoveSequence(sequence), k)
			ghostPaths = append(ghostPaths, GhostPath{cycle, n})
		}
	}
	aoc.LogDebug("ghostPaths: %+v\n", ghostPaths)
	var periods = make([]int, 0)
	for _, g := range ghostPaths {
		periods = append(periods, g.Cycle.Period)
	}
	aoc.LogInfo("lcm: %v\n", aoc.LeastCommonMultiple(periods[0], periods[1:]...))

	fmt.Println(result, result2)
}