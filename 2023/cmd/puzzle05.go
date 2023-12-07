package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
	"strconv"
	"math"
	"golang.org/x/exp/constraints"
)

func parseNumbersList(t string) []int {
	result := make([]int, 0)

	for _, s := range strings.Split(t, " ") {
		value, err := strconv.Atoi(s)
		if err == nil {
			result = append(result, value)
		}
	}

	return result
}

type Range[T constraints.Ordered] struct {
	Start T
	End T
}
func (r *Range[T]) contains(value T) bool {
	return value >= r.Start && value < r.End
}

type ConversionRange struct {
	From Range[int]
	ToStart int
}
type ConversionMap struct {
	/// Source from which conversion is done
	Source string
	/// Target to which the conversion is done
	Target string
	ranges []ConversionRange
}
func newConversionMap(source string, target string) *ConversionMap {
	r := new(ConversionMap)
	r.Source = source
	r.Target = target
	r.ranges = make([]ConversionRange, 0)
	return r
}
func (m *ConversionMap) insertMapping(destinationStart int, sourceStart int, length int) {
	m.ranges = append(
		m.ranges,
		ConversionRange {
			Range[int]{sourceStart, sourceStart + length},
			destinationStart,
		},
	)
}
func (m *ConversionMap) mapValue(value int) int {
	for _, r := range m.ranges {
		if r.From.contains(value) {
			return r.ToStart + (value - r.From.Start)
		}
	}

	return value
}

func followSeed(
	conversions map[string]*ConversionMap,
	seed int,
) int {
	currentMap := "seed"
	currentValue := seed

	for {
		conv := conversions[currentMap]
		if conv == nil { break }

		currentValue = conv.mapValue(currentValue)
		currentMap = conv.Target
	}

	return currentValue
}

func main() {
	input, err := aoc.Initialize()
	if err != nil { panic(err) }

	result := math.MaxInt32
	var seeds []int
	var conversions = make(map[string]*ConversionMap)
	var currentMap string

	result2 := math.MaxInt32

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		t := scanner.Text()
		aoc.LogTrace("text = %s\n", t)

		// skip empty lines
		if t == "" {
			continue
		}

		// seeds line
		if strings.HasPrefix(t, "seeds:") {
			seeds = parseNumbersList(strings.Split(t, ":")[1])
			continue
		}

		// map header
		if strings.HasSuffix(t, "map:") {
			mapName := strings.Split(strings.Split(t, " ")[0], "-")
			source := mapName[0]
			target := mapName[2]

			currentMap = source
			conversions[source] = newConversionMap(source, target)
			continue
		}

		// other lines are mappings for the current conversion map
		nums := parseNumbersList(t)
		conversions[currentMap].insertMapping(nums[0], nums[1], nums[2])
	}
	aoc.LogDebug("seeds = %v\nconversions = %v\n", seeds, conversions)

	for _, seed := range seeds {
		end := followSeed(conversions, seed)
		if result > end {
			result = end
		}
	}

	for i := 0; i < len(seeds); i += 2 {
		seedStart := seeds[i]
		seedEnd := seeds[i] + seeds[i + 1]

		aoc.LogDebug("seed range %v-%v (%v)\n", seedStart, seedEnd, seedEnd - seedStart)
		for seed := seedStart; seed < seedEnd; seed += 1 {
			end := followSeed(conversions, seed)
			if result2 > end {
				result2 = end
			}
		}
	}

	fmt.Println(result, result2)
}