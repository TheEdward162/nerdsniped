package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
)

type GameRound struct {
	Red int
	Green int
	Blue int
}
var BagLoad = GameRound {12, 13, 14}
func isRoundPossible(max GameRound, r GameRound) bool {
	return r.Red <= max.Red && r.Green <= max.Green && r.Blue <= max.Blue
}
func maxInt(a int, b int) int {
	if a > b {
		return a
	} else {
		return b
	}
}
func maxGameRound(a GameRound, b GameRound) GameRound {
	return GameRound {
		maxInt(a.Red, b.Red),
		maxInt(a.Green, b.Green),
		maxInt(a.Blue, b.Blue),
	}
}

func main() {
	input, err := aoc.Initialize()
	if err != nil { panic(err) }

	games := make(map[int][]GameRound)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		t := scanner.Text()
		aoc.LogTrace("text = %s\n", t)

		// get game id
		parts := strings.SplitN(t, ":", 2)
		var gameId int
		_, err := fmt.Sscanf(parts[0], "Game %d", &gameId)
		if err != nil { panic(err) }
		
		// parse rounds
		var rounds []GameRound = make([]GameRound, 0, 5)
		for _, roundStr := range strings.Split(parts[1], ";") {
			// parse draws per round
			var round GameRound
			for _, drawStr := range strings.Split(roundStr, ",") {
				var drawCount int
				var drawColor string
				_, err := fmt.Sscanf(drawStr, "%d %s", &drawCount, &drawColor)
				if err != nil { panic(err) }

				switch drawColor {
					case "red": round.Red = drawCount
					case "green": round.Green = drawCount
					case "blue": round.Blue = drawCount
					default: panic("Unexpected draw color " + drawColor)
				}
			}
			rounds = append(rounds, round)
		}
		aoc.LogTrace("game %d, rounds %+v\n", gameId, rounds)
		games[gameId] = rounds
	}

	result := 0
	result2 := 0
	for gameId, rounds := range games {
		isPossible := true
		maxBag := rounds[0]
		for _, round := range rounds {
			if !isRoundPossible(BagLoad, round) {
				isPossible = false
			}
			maxBag = maxGameRound(maxBag, round)
		}
		if isPossible {
			result += gameId
		} else {
			aoc.LogDebug("Game %d not possible", gameId)
		}

		aoc.LogTrace("Game %d requires %+v = %d", gameId, maxBag, maxBag.Red * maxBag.Green * maxBag.Blue)
		result2 += maxBag.Red * maxBag.Green * maxBag.Blue
	}

	fmt.Println(result, result2)
}
