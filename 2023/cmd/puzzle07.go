package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
	"cmp"
	"slices"
	"strconv"
)

const (
	HandTypeHighCard = iota
	HandTypeOnePair
	HandTypeTwoPair
	HandTypeThreeOfAKind
	HandTypeFullHouse
	HandTypeFourOfAKind
	HandTypeFiveOfAKind
)
type Hand struct {
	Cards string
	Type int
	TypeJokers int
	Bid int
}
func countRunes(s string) map[rune]int {
	result := make(map[rune]int)

	for _, r := range s {
		result[r] = result[r] + 1
	}

	return result
}
func classifyRuneCounts(c []int) int {
	slices.Sort(c)
	switch {
		// [] | [1|2|3|4|5]
		case len(c) == 0 || len(c) == 1: return HandTypeFiveOfAKind
		// [1, 1|2|3|4]
		case len(c) == 2 && c[0] == 1: return HandTypeFourOfAKind
		// [2, 2|3]
		case len(c) == 2 && c[0] == 2: return HandTypeFullHouse
		// [1, 1, 1|2|3]
		case len(c) == 3 && c[1] == 1: return HandTypeThreeOfAKind
		// [1, 2, 2]
		case len(c) == 3 && c[1] == 2: return HandTypeTwoPair
		// [1, 1, 1, 1|2]
		case len(c) == 4: return HandTypeOnePair
		// [1, 1, 1, 1, 1]
		case len(c) == 5: return HandTypeHighCard
		default: panic("invalid rune counts")
	}
}
func MakeHand(cards string, bid int) Hand {
	runes := countRunes(cards)
	aoc.LogTrace("runes = %v\n", runes)
	
	runeCounts := make([]int, 0)
	runeCountsWithoutJokers := make([]int, 0)
	for r, count := range runes {
		runeCounts = append(runeCounts, count)
		if r != 'J' {
			runeCountsWithoutJokers = append(runeCountsWithoutJokers, count)
		}
	}
	
	return Hand{cards, classifyRuneCounts(runeCounts), classifyRuneCounts(runeCountsWithoutJokers), bid}
}
func RankCard(card byte) int {
	switch card {
		case '2': return 2
		case '3': return 3
		case '4': return 4
		case '5': return 5
		case '6': return 6
		case '7': return 7
		case '8': return 8
		case '9': return 9
		case 'T': return 10
		case 'J': return 11
		case 'Q': return 12
		case 'K': return 13
		case 'A': return 14
		default: return 0
	}
}
func CompareHand(a Hand, b Hand) int {
	c := cmp.Compare(a.Type, b.Type)
	if c != 0 { return c }
	
	for i := 0; i < len(a.Cards); i += 1 {
		c = cmp.Compare(RankCard(a.Cards[i]), RankCard(b.Cards[i]))
		if c != 0 { return c }
	}

	return 0
}
func RankCardJokers(card byte) int {
	switch card {
		case 'J': return 1
		case '2': return 2
		case '3': return 3
		case '4': return 4
		case '5': return 5
		case '6': return 6
		case '7': return 7
		case '8': return 8
		case '9': return 9
		case 'T': return 10
		case 'Q': return 12
		case 'K': return 13
		case 'A': return 14
		default: return 0
	}
}
func CompareHandJokers(a Hand, b Hand) int {
	c := cmp.Compare(a.TypeJokers, b.TypeJokers)
	if c != 0 { return c }
	
	for i := 0; i < len(a.Cards); i += 1 {
		c = cmp.Compare(RankCardJokers(a.Cards[i]), RankCardJokers(b.Cards[i]))
		if c != 0 { return c }
	}

	return 0
}

func main() {
	input, err := aoc.Initialize()
	if err != nil { panic(err) }

	result := 0
	result2 := 0

	hands := make([]Hand, 0)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		t := scanner.Text()
		aoc.LogTrace("text = %s\n", t)

		parts := strings.Split(t, " ")
		hand := parts[0]
		bid, _ := strconv.Atoi(parts[1])

		hands = append(hands, MakeHand(hand, bid))
	}

	slices.SortFunc(hands, CompareHand)
	aoc.LogDebug("hands = %v\n", hands)
	for i, hand := range hands {
		result += hand.Bid * (i + 1)
	}

	slices.SortFunc(hands, CompareHandJokers)
	for i, hand := range hands {
		result2 += hand.Bid * (i + 1)
	}

	fmt.Println(result, result2)
}