package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"math"
	"strings"
	"strconv"
)

// parameter T - race time
// parameter D - record distance
// d - distance
// s - speed
// h - hold time
//
// d = s * (T - h)
// d = sT - sh
// s = h
// d = hT - h^2
//
// we are looking for all integer solutions so that d > D
// we can bound using real solutions and then extract integer solutions
// bound can be found by finding where this function crosses D
// solve for h: d = hT - h^2 = D
//
// quadratic form: ax^2 + bx + c = 0
// a = -1, b = T, c = -D
// quadratic formula: x = (-b +- sqrt(b^2 - 4ac))/2a
//
// then h0 and h1 will be the interval from which we'll extract all integer solutions
type RaceRecord struct {
	Time int
	Distance int
}
func quadraticRootsIntegerRange(race RaceRecord) (int, int) {
	var a float64 = -1
	var b float64 = float64(race.Time)
	var c float64 = -float64(race.Distance)
	
	left := (-b + math.Sqrt(math.Pow(b, 2) - 4 * a * c)) / (2 * a)
	right := (-b - math.Sqrt(math.Pow(b, 2) - 4 * a * c)) / (2 * a)
	
	if left > right {
		left, right = right, left
	}

	return int(math.Ceil(left)), int(math.Floor(right))
}

func main() {
	input, err := aoc.Initialize()
	if err != nil { panic(err) }

	result := 1
	result2 := math.MaxInt32

	var times []int = make([]int, 0)
	var distances []int = make([]int, 0)
	var part2Record RaceRecord

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		t := scanner.Text()
		aoc.LogTrace("text = %s\n", t)

		parts := strings.Split(t, ":")
		switch parts[0] {
			case "Time":
				times = aoc.ParseIntList(parts[1])
				part2Record.Time, _ = strconv.Atoi(strings.ReplaceAll(parts[1], " ", ""))
			case "Distance":
				distances = aoc.ParseIntList(parts[1])
				part2Record.Distance, _ = strconv.Atoi(strings.ReplaceAll(parts[1], " ", ""))
		}
	}
	aoc.LogDebug("times = %v, distances = %v\n", times, distances)
	aoc.LogDebug("oneRecord = %v\n", part2Record)

	for i := 0; i < len(times); i += 1 {
		a, b := quadraticRootsIntegerRange(RaceRecord{times[i], distances[i]})
		result *= b - a + 1
	}
	r2a, r2b := quadraticRootsIntegerRange(part2Record)
	result2 = r2b - r2a + 1

	fmt.Println(result, result2)
}