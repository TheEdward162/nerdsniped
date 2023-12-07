package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
	"strconv"
	"math"
	"k8s.io/apimachinery/pkg/util/sets"
)

func parseNumbersSet(t string) sets.Set[int] {
	result := sets.Set[int]{}

	for _, s := range strings.Split(t, " ") {
		value, err := strconv.Atoi(s)
		if err == nil {
			sets.Insert(result, value)
		}
	}

	return result
}

func main() {
	input, err := aoc.Initialize()
	if err != nil { panic(err) }

	result := 0
	futureCards := make([]int, 0)
	result2 := 0

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		t := scanner.Text()
		aoc.LogTrace("text = %s\n", t)

		parts := strings.Split(t, ":")
		// get card id
		var cardId int
		_, err := fmt.Sscanf(parts[0], "Card %d", &cardId)
		if err != nil { panic(err) }

		parts = strings.Split(parts[1], "|")
		winning := parseNumbersSet(parts[0])
		scratched := parseNumbersSet(parts[1])
		aoc.LogTrace("winning %v\nscratched %v\n", winning, scratched)

		winners := winning.Intersection(scratched)
		aoc.LogDebug("winners %d %v\n", len(winners), winners)

		// part one, powers of winning numbers
		if len(winners) > 0 {
			result += int(math.Pow(2, float64(len(winners) - 1)))
		}

		// part two, cards win following cards
		winCount := 1
		if len(futureCards) > 0 {
			winCount += futureCards[0]
			futureCards = futureCards[1:]
		}
		result2 += winCount

		for i := 0; i < len(winners); i += 1 {
			if len(futureCards) <= i {
				futureCards = append(futureCards, winCount)
			} else {
				futureCards[i] += winCount
			}
		}
	}

	fmt.Println(result, result2)
}