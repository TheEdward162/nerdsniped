package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
	"strconv"
	"slices"
	"sync"
	"sync/atomic"
)

const (
	SpringOperational = 0
	SpringDamaged = 1
	SpringUnknown = 2
)
type SpringEntry struct {
	Type int
}
func ParseSpringEntry(entry rune) SpringEntry {
	switch entry {
		case '.': return SpringEntry{SpringOperational}
		case '#': return SpringEntry{SpringDamaged}
		case '?': return SpringEntry{SpringUnknown}
		default: panic("invalid entry")
	}
}
func FmtSpringEntry(entry SpringEntry) string {
	var result rune

	switch entry.Type {
		case SpringOperational: result = '.'
		case SpringDamaged: result = '#'
		case SpringUnknown: result = '?'
		default: result = rune(entry.Type)
	}

	return string(result)
}
func CanBeOperational(entry SpringEntry) bool {
	return entry.Type == SpringOperational || entry.Type == SpringUnknown
}
func CanBeDamaged(entry SpringEntry) bool {
	return entry.Type == SpringDamaged || entry.Type == SpringUnknown
}
func CmpOperationalOrOther(a SpringEntry, b SpringEntry) bool {
	return (a.Type == SpringOperational && b.Type == SpringOperational) || (a.Type != SpringOperational && b.Type != SpringOperational)
}

type SpringRow struct {
	Row []SpringEntry
	Groups []int
}
func FmtSpringRow(row SpringRow) string {
	var result = ""
	for _, e := range row.Row {
		result += FmtSpringEntry(e)
	}
	result += " "
	for i, g := range row.Groups {
		if i > 0 { result += "," }
		result += strconv.Itoa(g)
	}

	return result
}
func UnfoldRow(row SpringRow) SpringRow {
	var resultRow = make([]SpringEntry, 0, len(row.Row) * 5 + 5)
	var resultGroups = make([]int, 0, len(row.Groups) * 5)
	for i := 0; i < 5; i += 1 {
		resultRow = append(resultRow, row.Row...)
		if i < 4 {
			resultRow = append(resultRow, SpringEntry{SpringUnknown})
		}
		resultGroups = append(resultGroups, row.Groups...)
	}

	return SpringRow{resultRow, resultGroups}
}

// trim leading and trailing operational
// also coalesce runs of operational into singles
func PassCoalesceOperational(s *SpringRow) bool {
	if len(s.Row) == 0 || len(s.Groups) == 0 { return false }

	var changed = false
	var runs = aoc.SplitGroups(s.Row)
	if s.Row[0].Type == SpringOperational {
		runs = runs[1:]
		changed = true
	}
	if s.Row[len(s.Row) - 1].Type == SpringOperational {
		runs = runs[:len(runs) - 1]
		changed = true
	}
	for i, run := range runs {
		if len(run) > 1 && run[0].Type == SpringOperational {
			runs[i] = run[:1]
			changed = true
		}
	}

	if changed {
		s.Row = aoc.ConcatSlices(runs...)
	}
	return changed
}
// if the first run of the row starts with a `#` then the first group can be resolved right away (and mirrored)
func PassTrimDamagedStart(s *SpringRow) bool {
	if len(s.Row) == 0 || len(s.Groups) == 0 { return false }
	
	var changed = false
	if len(s.Row) > 0 && s.Row[0].Type == SpringDamaged {
		// group len plus a dot after (if not at the end)
		var groupLen = s.Groups[0]
		if len(s.Row) > groupLen { groupLen += 1 }
		s.Row = s.Row[groupLen:]
		s.Groups = s.Groups[1:]

		changed = true
	}
	if len(s.Row) > 0 && s.Row[len(s.Row) - 1].Type == SpringDamaged {
		var groupLen = s.Groups[len(s.Groups) - 1]
		if len(s.Row) > groupLen { groupLen += 1}
		s.Row = s.Row[:len(s.Row) - groupLen]
		s.Groups = s.Groups[:len(s.Groups) - 1]

		changed = true
	}

	return changed
}
// if the row starts (or ends) with a run of `?`, at least one `#`, is terminated by a `.` and is the right length
// the resolve it right away to the first (last) group
func PassTrimExactMatch(s *SpringRow) bool {
	if len(s.Row) == 0 || len(s.Groups) == 0 { return false }

	var changed = false
	var runs = aoc.SplitGroupsFunc(s.Row, CmpOperationalOrOther)

	// first run is exact match
	// if the run is one element longer BUT the run starts or ends with damaged we can resolve it too
	if len(runs[0]) == s.Groups[0] && slices.Contains(runs[0], SpringEntry{SpringDamaged}) || len(runs[0]) == s.Groups[0] + 1 && (slices.Index(runs[0], SpringEntry{SpringDamaged}) == 0 || slices.Index(runs[0], SpringEntry{SpringDamaged}) == len(runs[0]) - 1) {
		runs = runs[1:]
		s.Groups = s.Groups[1:]
		changed = true
	}
	// mirrored
	if len(runs) > 0 && len(s.Groups) > 0 {
		var lastRun = runs[len(runs) - 1]
		var lastGroup = s.Groups[len(s.Groups) - 1]
		if len(lastRun) == lastGroup && slices.Contains(lastRun, SpringEntry{SpringDamaged}) || len(lastRun) == lastGroup + 1 && (slices.Index(lastRun, SpringEntry{SpringDamaged}) == 0 || slices.Index(lastRun, SpringEntry{SpringDamaged}) == len(lastRun) - 1) {
			runs = runs[:len(runs) - 1]
			s.Groups = s.Groups[:len(s.Groups) - 1]
			changed = true
		}
	}

	if changed {
		s.Row = aoc.ConcatSlices(runs...)
	}
	return changed
}
// if the row starts (or ends) with a run shorter than the first (last) group, trim the run
func PassTrimSmallRuns(s *SpringRow) bool {
	if len(s.Row) == 0 || len(s.Groups) == 0 { return false }

	var changed = false
	var runs = aoc.SplitGroupsFunc(s.Row, CmpOperationalOrOther)
	if len(runs[0]) < s.Groups[0] {
		runs = runs[1:]
		changed = true
	}
	// mirrored
	if len(runs) > 0 && len(s.Groups) > 0 {
		var lastRun = runs[len(runs) - 1]
		var lastGroup = s.Groups[len(s.Groups) - 1]
		if len(lastRun) < lastGroup {
			runs = runs[:len(runs) - 1]
			changed = true
		}
	}

	if changed {
		s.Row = aoc.ConcatSlices(runs...)
	}
	return changed
}

// Generate all possible assignments of group into run
// Returns the rest of the run for each unique assignment
type generateAssignmentsResult struct {
	Assignment []SpringEntry
	Remaining []SpringEntry
}
func generateAssignmentsAssigmentHelper(prefix []SpringEntry, length int, hasTerminator bool) []SpringEntry {
	var result = make([]SpringEntry, 0, len(prefix) + length + 1)

	// result = append(result, prefix...)
	for i := 0; i < len(prefix); i += 1 {
		result = append(result, SpringEntry{SpringOperational})
	}
	for i := 0; i < length; i += 1 {
		result = append(result, SpringEntry{SpringDamaged})
	}
	if hasTerminator {
		result = append(result, SpringEntry{SpringOperational})
	}

	return result
}
func generateAssignments(run []SpringEntry, groupLen int) []generateAssignmentsResult {
	var lastPossibleStart = slices.Index(run, SpringEntry{SpringDamaged})
	if lastPossibleStart < 0 { lastPossibleStart = len(run) - 1 }
	lastPossibleStart = aoc.MinI(lastPossibleStart, len(run) - groupLen)
	
	var result = make([]generateAssignmentsResult, 0)
	if lastPossibleStart < 0 {
		return result
	}
	for start := 0; start <= lastPossibleStart; start += 1 {
		if start + groupLen == len(run) {
			// end of the run
			result = append(
				result,
				generateAssignmentsResult{
					generateAssignmentsAssigmentHelper(run[:start], groupLen, false),
					nil,
				},
			)
		} else if CanBeOperational(run[start + groupLen]) {
			// something remains
			result = append(
				result,
				generateAssignmentsResult{
					generateAssignmentsAssigmentHelper(run[:start], groupLen, true),
					run[start + groupLen + 1:],
				},
			)
		}
	}

	return result
}
func maxFittableGroups(run []SpringEntry, groups []int) int {
	var remaining = len(run)
	for i, g := range groups {
		if remaining < g {
			return i
		}

		remaining -= g + 1
	}

	return len(groups)
}
func canBeAssigned(runs [][]SpringEntry, groups []int) bool {	
	// check if we can assing every group to at least one run
	var currentGroup = 0
	for _, run := range runs {
		currentGroup += maxFittableGroups(run, groups[currentGroup:])
		if currentGroup >= len(groups) {
			return true
		}
	}

	// TODO: if there is a group that doesn't fit into anything?

	return false
}
func runsContainDamaged(runs ...[]SpringEntry) bool {
	for _, run := range runs {
		if slices.Contains(run, SpringEntry{SpringDamaged}) {
			return true
		}
	}

	return false
}

type SolverCacheKey struct {
	RemainderLen int
	RunStart int
	GroupStart int
}
type Solver struct {
	Runs [][]SpringEntry
	Groups []int
	ComputeCache map[SolverCacheKey]int
}
func NewSolver(springRow SpringRow) *Solver {
	var s = new(Solver)
	for _, run := range aoc.SplitGroupsFunc(springRow.Row, CmpOperationalOrOther) {
		if len(run) > 0 && CanBeDamaged(run[0]) {
			s.Runs = append(s.Runs, run)
		}
	}
	s.Groups = springRow.Groups
	s.ComputeCache = make(map[SolverCacheKey]int)

	return s
}
func (s *Solver) fmtCurrentAssignment(current [][]SpringEntry, remainder []SpringEntry, runStart int, _groupStart int) string {	
	var result = ""
	for _, run := range current {
		for _, e := range run {
			result += FmtSpringEntry(e)
		}
	}
	if len(remainder) > 0 {
		for _, e := range remainder {
			result += FmtSpringEntry(e)
		}
	}
	if runStart < len(s.Runs) {
		for _, run := range s.Runs[runStart:] {
			result += FmtSpringEntry(SpringEntry{SpringOperational})
			for _, e := range run {
				result += FmtSpringEntry(e)
			}
		}
	}

	return result
}
func (s *Solver) solveRec(remainder []SpringEntry, runStart int, groupStart int, currentAssignment [][]SpringEntry) int {	
	var depth = len(currentAssignment)
	aoc.LogTrace("%scurrent = %v\n", strings.Repeat("  ", depth), s.fmtCurrentAssignment(currentAssignment, remainder, runStart, groupStart))
	
	if groupStart >= len(s.Groups) {
		var result = 0
		if !runsContainDamaged(remainder) && !(runStart < len(s.Runs) && runsContainDamaged(s.Runs[runStart:]...)) {
			result = 1
		}

		aoc.LogTrace("%s|%d %s\n", strings.Repeat("  ", depth), result, s.fmtCurrentAssignment(currentAssignment, remainder, runStart, groupStart))
		return result
	}
	var groups = s.Groups[groupStart:]

	var cacheKey = SolverCacheKey{len(remainder),runStart,groupStart}
	if cached, ok := s.ComputeCache[cacheKey]; ok {
		aoc.LogTrace("%s#%d\n", strings.Repeat("  ", depth), cached)
		return cached
	}

	var result = 0
	if len(remainder) >= groups[0] {
		var remainderAssignments = generateAssignments(remainder, groups[0])
		
		for _, a := range remainderAssignments {
			var ca [][]SpringEntry = nil
			if aoc.LogEnabled(aoc.LogLevelTrace) { ca = aoc.AppendCopy(currentAssignment, a.Assignment) }

			result += s.solveRec(a.Remaining, runStart, groupStart + 1, ca)
		}

		aoc.LogTrace("%s~%d\n", strings.Repeat("  ", depth), result)
	}

	if runStart >= len(s.Runs) || slices.Contains(remainder, SpringEntry{SpringDamaged}) {
		aoc.LogTrace("%s=%d\n", strings.Repeat("  ", depth), result)
		s.ComputeCache[cacheKey] = result
		return result
	}

	var runs = s.Runs[runStart:]
	// build assignments, and allowing skipping runs which don't contain any damaged
	var allAssignments = make([][]generateAssignmentsResult, 0, len(runs))
	for i, run := range runs {
		allAssignments = append(allAssignments, generateAssignments(run, groups[0]))

		// stop after first run that contains at least one damaged
		if slices.Contains(run, SpringEntry{SpringDamaged}) {
			break
		}

		// pruning
		if !canBeAssigned(runs[i + 1:], groups[1:]) {
			break
		}
	}

	for runI, assignments := range allAssignments {
		for _, a := range assignments {
			var ca [][]SpringEntry = nil
			if aoc.LogEnabled(aoc.LogLevelTrace) { ca = aoc.AppendCopy(currentAssignment, aoc.ConcatSlices(make([]SpringEntry, len(remainder) + 1), a.Assignment)) }
			
			result += s.solveRec(
				a.Remaining,
				runStart + runI + 1,
				groupStart + 1,
				ca,
			)
		}
	}

	aoc.LogTrace("%s=%d\n", strings.Repeat("  ", depth), result)
	s.ComputeCache[cacheKey] = result
	return result
}
func CountRowArrangements(springRow SpringRow) int {
	var result int = 0
	
	for {
		var changed = false

		// first pass - coalesce operational
		changed = changed || PassCoalesceOperational(&springRow)
		// second pass - if the row starts or ends with a `#` then resolve the border group right away
		changed = changed || PassTrimDamagedStart(&springRow)
		// third pass - resolve border groups which only have one option
		changed = changed || PassTrimExactMatch(&springRow)
		// fourth pass - if border run is smaller than its group, trim it
		changed = changed || PassTrimSmallRuns(&springRow)

		if !changed { break }
	}
	aoc.LogDebug("row opt = %v\n", FmtSpringRow(springRow))
	var solver = NewSolver(springRow)
	result = solver.solveRec(nil, 0, 0, nil)
	aoc.LogInfo("arrangements = %v\n", result)

	return result
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var springs = make([]SpringRow, 0)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)
		
		parts := strings.Split(t, " ")

		var row = make([]SpringEntry, 0)
		for _, r := range parts[0] {
			row = append(row, ParseSpringEntry(r))
		}
		var groups = aoc.ParseIntListBy(parts[1], ",")

		springs = append(springs, SpringRow{row, groups})
	}
	aoc.LogTrace("springs = %v\n", springs)

	for _, row := range springs {
		result += CountRowArrangements(row)
	}

	if true {
		for i, row := range springs {
			aoc.LogDebug("row n: %d\n", i)
			var unfoldedRow = UnfoldRow(row)
			result2 += CountRowArrangements(unfoldedRow)
		}
	} else {
		var wg sync.WaitGroup
		var result2Parallel atomic.Int64
		var parallelDone atomic.Int32
		for i, row := range springs {
			i := i
			row := row
			wg.Add(1)
			go func() {
				defer wg.Done()
				var unfoldedRow = UnfoldRow(row)
				var res = CountRowArrangements(unfoldedRow)
				result2Parallel.Add(int64(res))
				aoc.LogInfo("done: %d %d\n", parallelDone.Add(1), i)
			}()
		}
		wg.Wait()
		result2 = int(result2Parallel.Load())
	}

	fmt.Println(result, result2)
}