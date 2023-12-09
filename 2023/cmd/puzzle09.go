package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
)

type SampleSequence struct {
	Samples []int
}
func (s SampleSequence) IsZero() bool {
	for _, s := range s.Samples {
		if s != 0 { return false }
	}

	return true
}
func (s SampleSequence) FirstValue() int {
	return s.Samples[0]
}
func (s SampleSequence) LastValue() int {
	return s.Samples[len(s.Samples) - 1]
}
func (s SampleSequence) Derive() SampleSequence {
	var result = make([]int, 0, len(s.Samples) - 1)
	
	var last = s.Samples[0]
	for _, s := range s.Samples[1:] {
		result = append(result, s - last)
		last = s
	}

	return SampleSequence{result}
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)
		
		var sequences = make([]SampleSequence, 0)
		sequences = append(sequences, SampleSequence{aoc.ParseIntList(t)})

		for {
			var d = sequences[len(sequences) - 1].Derive()
			if d.IsZero() { break }

			sequences = append(sequences, d)
		}
		aoc.LogDebug("sequences = %v\n", sequences)

		var next = 0
		var previous = 0
		for i := len(sequences) - 1; i >= 0; i -= 1 {
			next += sequences[i].LastValue()
			previous = sequences[i].FirstValue() - previous
		}
		result += next
		result2 += previous
	}

	fmt.Println(result, result2)
}