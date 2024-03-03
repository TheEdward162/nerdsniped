package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
	"math"
	"cmp"
	"slices"
	"golang.org/x/exp/constraints"
)

type Range[T constraints.Integer | constraints.Float] struct {
	Start T
	End T
}
func makeRange[T constraints.Integer | constraints.Float](start T, length T) Range[T] {
	return Range[T]{start, start + length}
}
func (r *Range[T]) Length() T {
	return r.End - r.Start
}
func (r *Range[T]) IsEmpty() bool {
	return r.Length() <= 0
}
func (r *Range[T]) Contains(value T) bool {
	return value >= r.Start && value < r.End
}

type ConversionRange struct {
	Range[int]
	TargetStart int
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
		if r.Contains(value) {
			return r.TargetStart + (value - r.Start)
		}
	}

	return value
}
/// Intersects conversion ranges so that the returned slices contain `[intersection, remainders...]` or are empty.
///
/// For example, for ranges `A[3:8]->B[7:12]` and `B[5:9]->C[0:4]`:
/// ```
/// space A: ---11111-------
/// space B: -------11111---
/// space B: -----2222------
/// space C: 2222-----------
/// ```
///
/// Returns `{A[3:5]->B[7:9], A[5:8]->B[9:12]}, {B[7:9]->C[2:4], B[5:7]->C[0:2]}`
func intersectConversionRanges(ab ConversionRange, bc ConversionRange) ([]ConversionRange, []ConversionRange) {
	abResult := make([]ConversionRange, 0)
	bcResult := make([]ConversionRange, 0)
	
	// ab mapped to b
	abm := makeRange(ab.TargetStart, ab.Length())
	// bc reverse-mapped to a
	bcrm := makeRange(bc.Start - ab.TargetStart + ab.Start, bc.Length())

	intersect := Range[int]{max(abm.Start, bc.Start), min(abm.End, bc.End)}
	if intersect.IsEmpty() {
		return nil, nil
	}
	abResult = append(abResult, ConversionRange{
		makeRange(ab.Start + intersect.Start - abm.Start, intersect.Length()),
		ab.TargetStart + intersect.Start - abm.Start,
	})
	bcResult = append(bcResult, ConversionRange{
		intersect,
		bc.TargetStart + intersect.Start - bc.Start,
	})

	// mapping `a -> b`
	abBefore := ConversionRange{
		Range[int]{ab.Start, bcrm.Start},
		ab.TargetStart,
	}
	if !abBefore.IsEmpty() { abResult = append(abResult, abBefore) }
	abAfter := ConversionRange{
		Range[int]{bcrm.End, ab.End},
		ab.TargetStart + bcrm.End - ab.Start,
	}
	if !abAfter.IsEmpty() { abResult = append(abResult, abAfter) }

	// mapping `b -> c`
	bcBefore := ConversionRange{
		Range[int]{bc.Start, abm.Start},
		bc.TargetStart,
	}
	if !bcBefore.IsEmpty() { bcResult = append(bcResult, bcBefore) }
	bcAfter := ConversionRange{
		Range[int]{abm.End, bc.End},
		bc.TargetStart + abm.End - bc.Start,
	}
	if !bcAfter.IsEmpty() { bcResult = append(bcResult, bcAfter) }

	return abResult, bcResult
}
// pretend this is a test
func intersectConversionRangesTest() {
	// ---aaaaa-----
	// -------aaaaa-
	// -----bbbb----
	// bbbb---------
	a, b := intersectConversionRanges(ConversionRange{Range[int]{3,8}, 7}, ConversionRange{Range[int]{5,9}, 0})
	if len(a) != 2 || len(b) != 2 { panic("bad result ab") }
	if a[0].Start != 3 || a[0].End != 5 || a[0].TargetStart != 7 { panic("bad a[0]") }
	if a[1].Start != 5 || a[1].End != 8 || a[1].TargetStart != 9 { panic("bad a[1]") }
	if b[0].Start != 7 || b[0].End != 9 || b[0].TargetStart != 2 { panic("bad b[0]") }
	if b[1].Start != 5 || b[1].End != 7 || b[1].TargetStart != 0 { panic("bad b[1]") }

	// cc---
	// cc---
	// --d--
	// d----
	c, d := intersectConversionRanges(ConversionRange{Range[int]{0,2}, 0}, ConversionRange{Range[int]{2,3}, 0})
	if len(c) != 0 || len(d) != 0 { panic("bad result cd") }

	// ee-----
	// -ee----
	// --ff---
	// ---ff--
	e, f := intersectConversionRanges(ConversionRange{Range[int]{0,2}, 1}, ConversionRange{Range[int]{2,4}, 3})
	if len(e) != 2 || len(f) != 2 { panic("bad result ef") }
	if e[0].Start != 1 || e[0].End != 2 || e[0].TargetStart != 2 { panic("bad e[0]") }
	if e[1].Start != 0 || e[1].End != 1 || e[1].TargetStart != 1 { panic("bad e[1]") }
	if f[0].Start != 2 || f[0].End != 3 || f[0].TargetStart != 3 { panic("bad f[0]") }
	if f[1].Start != 3 || f[1].End != 4 || f[1].TargetStart != 4 { panic("bad f[1]") }

	// -gg---
	// -gg---
	// hhhh--
	// hhhh--
	g, h := intersectConversionRanges(ConversionRange{Range[int]{1,3}, 1}, ConversionRange{Range[int]{0,4}, 0})
	if len(g) != 1 || len(h) != 3 { panic("bad result gh") }
	if g[0].Start != 1 || g[0].End != 3 || g[0].TargetStart != 1 { panic("bad g[0]") }
	if h[0].Start != 1 || h[0].End != 3 || h[0].TargetStart != 1 { panic("bad h[0]") }
	if h[1].Start != 0 || h[1].End != 1 || h[1].TargetStart != 0 { panic("bad h[1]") }
	if h[2].Start != 3 || h[2].End != 4 || h[2].TargetStart != 3 { panic("bad h[2]") }

	// --iii--
	// iii----
	// -j-----
	// ---j---
	i, j := intersectConversionRanges(ConversionRange{Range[int]{2,5}, 0}, ConversionRange{Range[int]{1,2}, 3})
	if len(i) != 3 || len(j) != 1 { panic("bad result ij") }
	if i[0].Start != 3 || i[0].End != 4 || i[0].TargetStart != 1 { panic("bad i[0]") }
	if i[1].Start != 2 || i[1].End != 3 || i[1].TargetStart != 0 { panic("bad i[1]") }
	if i[2].Start != 4 || i[2].End != 5 || i[2].TargetStart != 2 { panic("bad i[2]") }
	if j[0].Start != 1 || j[0].End != 2 || j[0].TargetStart != 3 { panic("bad j[0]") }
}
/// Coalesce two conversion maps so that the new map converts from `a.Source` to `b.Target`
func coalesceConversionMaps(a *ConversionMap, b *ConversionMap, ignoreImplicitTo bool) *ConversionMap {
	// result from a -> c
	result := newConversionMap(a.Source, b.Target)
	// ranges from a -> b
	fromRanges := append([]ConversionRange(nil), a.ranges...)
	// ranges from b -> c
	toRanges := append([]ConversionRange(nil), b.ranges...)

	fromI := 0
	for fromI < len(fromRanges) {
		wasFromReplaced := false

		toI := 0
		for toI < len(toRanges) {
			splitFrom, splitTo := intersectConversionRanges(fromRanges[fromI], toRanges[toI])
			// no intersection
			if len(splitFrom) == 0 || len(splitTo) == 0 {
				toI += 1
				continue
			}

			// replace current with remainders
			fromRanges = append(fromRanges[:fromI], fromRanges[fromI + 1:]...)
			fromRanges = append(fromRanges, splitFrom[1:]...)
			toRanges = append(toRanges[:toI], toRanges[toI + 1:]...)
			toRanges = append(toRanges, splitTo[1:]...)

			// assumption: there aren't multiple ranges in `from` that map onto the same ranges in `to`
			result.ranges = append(result.ranges, ConversionRange { splitFrom[0].Range, splitTo[0].TargetStart })
			
			wasFromReplaced = true
			break
		}

		if !wasFromReplaced { fromI += 1 }
	}
	/// the remaining ranges have implicit direct mapping and can be directly inserted
	result.ranges = append(result.ranges, fromRanges...)
	if !ignoreImplicitTo {
		result.ranges = append(result.ranges, toRanges...)
	}

	return result
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
	intersectConversionRangesTest()

	input, err := aoc.Initialize()
	if err != nil { panic(err) }

	result := math.MaxInt32
	result2 := math.MaxInt32

	var seeds []int
	var conversions = make(map[string]*ConversionMap)
	var currentMap string

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
			seeds = aoc.ParseIntList(strings.Split(t, ":")[1])
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
		nums := aoc.ParseIntList(t)
		conversions[currentMap].insertMapping(nums[0], nums[1], nums[2])
	}
	aoc.LogDebug("seeds = %v\nconversions = %v\n", seeds, conversions)

	var coalescedMap *ConversionMap = conversions["seed"]
	for {
		if conversions[coalescedMap.Target] == nil { break }
		coalescedMap = coalesceConversionMaps(coalescedMap, conversions[coalescedMap.Target], false)
	}
	aoc.LogDebug("coalescedMap = %v\n", coalescedMap)

	for _, seed := range seeds {
		end := followSeed(conversions, seed)
		end2 := coalescedMap.mapValue(seed)
		if result > end2 {
			result = end2
		}

		if end != end2 {
			aoc.LogWarn("end (%v) != end2 (%v) for input %v\n", end, end2, seed)
		}
	}

	// pretend that inputs are ranges from a -> a
	inputPseudoConversionMap := new(ConversionMap)
	for i := 0; i < len(seeds); i += 2 {
		seedStart := seeds[i]
		inputPseudoConversionMap.insertMapping(seedStart, seedStart, seeds[i + 1])
	}
	// then coalesce with the conversion map, but this time skip all ranges which aren't mapped-into by inputs
	inputPseudoConversionMap = coalesceConversionMaps(inputPseudoConversionMap, coalescedMap, true)
	// this way we end up only with ranges which are reachable from inputs (seeds)
	slices.SortFunc(inputPseudoConversionMap.ranges, func (a, b ConversionRange) int {
		return cmp.Compare(a.TargetStart, b.TargetStart)
	})
	// if we sort these by their target start (location) and take the smallest then that is the smallest mapped-into location
	result2 = inputPseudoConversionMap.ranges[0].TargetStart

	fmt.Println(result, result2)
}