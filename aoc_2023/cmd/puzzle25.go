package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
	"k8s.io/apimachinery/pkg/util/sets"
)

type VertexSet = sets.Set[string]

type Vertex struct {
	Names VertexSet
	Edges []string
}
func CloneVertex(vertex *Vertex) *Vertex {
	return &Vertex{vertex.Names.Clone(),aoc.AppendCopy(vertex.Edges)}
}
func EnsureVertex(vertices map[string]*Vertex, name string) {
	var _, ok = vertices[name]
	if !ok {
		vertices[name] = &Vertex{sets.New[string](name),nil}
	}
}

func mergeVertices(graph map[string]*Vertex, a string, b string) {
	var aVert = graph[a]
	var bVert = graph[b]
	
	var newKey = a + "," + b

	var names = aVert.Names.Union(bVert.Names)
	var edges = make([]string, 0)
	for _, to := range aVert.Edges {
		if to == a || to == b { continue }
		edges = append(edges, to)
	}
	for _, to := range bVert.Edges {
		if to == a || to == b { continue }
		edges = append(edges, to)
	}
	graph[newKey] = &Vertex{names,edges}
	for _, otherVert := range graph[newKey].Edges {
		for i, to := range graph[otherVert].Edges {
			if to == a || to == b {
				graph[otherVert].Edges[i] = newKey
			}
		}
	}
	aoc.LogTrace("Merged %v and %v -> %v", a, b, newKey)
	// aoc.LogTrace("Merged %+v", graph[newKey])
	
	delete(graph, a)
	delete(graph, b)
}

func stoerWagnerMinimumCutPhase(graph map[string]*Vertex, start string) (int, [2]VertexSet) {
	var selectedSet = sets.New[string](start)
	var lastSelected = start

	for selectedSet.Len() < len(graph) - 1 {
		var maxCost = -1
		var maxCostV = ""
		for v := range graph {
			if selectedSet.Has(v) { continue }
			var weight = 0
			for _, to := range graph[v].Edges {
				if selectedSet.Has(to) { weight += 1 }
			}

			if weight > maxCost {
				maxCost = weight
				maxCostV = v
			}
		}
		if maxCostV == "" {
			panic("sanity check failed")
		}

		selectedSet.Insert(maxCostV)
		lastSelected = maxCostV
	}

	var onlyNonSelected string
	for v := range graph {
		if !selectedSet.Has(v) {
			onlyNonSelected = v
			break
		}
	}
	if onlyNonSelected == "" {
		panic("sanity check failed")
	}

	var cutWeight = 0
	for _, to := range graph[onlyNonSelected].Edges {
		if selectedSet.Has(to) { cutWeight += 1 }
	}
	var cutParts = [2]VertexSet{sets.Set[string]{},sets.Set[string]{}}
	for v := range selectedSet {
		cutParts[0] = cutParts[0].Union(graph[v].Names)
	}
	cutParts[1] = graph[onlyNonSelected].Names.Clone()

	mergeVertices(graph, lastSelected, onlyNonSelected)
	return cutWeight, cutParts
}
func stoerWagnerBestCut(graph map[string]*Vertex, rankFunc func(cost int, a VertexSet, b VertexSet) int) [2]VertexSet {
	var start string
	for k := range graph {
		start = k
		break
	}
	aoc.LogDebug("stoer-wagner start: %v\n", start)

	var bestRank = 0
	var bestCutParts [2]VertexSet
	for len(graph) > 1 {
		var cost, parts = stoerWagnerMinimumCutPhase(graph, start)
		var rank = rankFunc(cost, parts[0], parts[1])
		
		if aoc.LogEnabled(aoc.LogLevelTrace) {
			var cutLeft = ""
			for v := range parts[0] { cutLeft += v + " " }
			var cutRight = ""
			for v := range parts[1] { cutRight += v + " " }
			aoc.LogTrace("new best cut: %v\n%s\n%s", cost, cutLeft, cutRight)
		}
		if rank > bestRank {
			bestRank = rank
			bestCutParts = parts
		}
	}
	aoc.LogTrace("best cut: %d %+v\n", bestRank, bestCutParts)

	return bestCutParts
}

func rankFunc(cost int, a VertexSet, b VertexSet) int {
	if cost != 3 {
		return 0
	}

	return a.Len() * b.Len()
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var graph = make(map[string]*Vertex)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)

		var parts = strings.Split(t, ": ")
		var from = parts[0]
		var tos = strings.Split(parts[1], " ")

		EnsureVertex(graph, from)
		graph[from].Edges = append(graph[from].Edges, tos...)
		for _, to := range tos {
			EnsureVertex(graph, to)
			graph[to].Edges = append(graph[to].Edges, from)
		}
	}

	if aoc.LogEnabled(aoc.LogLevelTrace) {
		aoc.LogTrace("graphviz:\b%v", fmtGraphviz(graph))
	}

	var resultParts = stoerWagnerBestCut(graph, rankFunc)
	result = resultParts[0].Len() * resultParts[1].Len()

	fmt.Println(result, result2)
}

func fmtGraphviz(graph map[string]*Vertex) string {
	var lines = make([]string, 0)
	lines = append(lines, "strict graph puzzle25 {")
	
	for from, vert := range graph {
		for _, to := range vert.Edges {
			lines = append(lines, fmt.Sprintf("%s -- %s;", from, to))
		}
	}
	
	lines = append(lines, "}")
	return strings.Join(lines, "\n")
}