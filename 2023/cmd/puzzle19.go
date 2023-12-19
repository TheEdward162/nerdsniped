package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"strings"
	"strconv"
)

const (
	ParamNone = 0
	ParamX = 1
	ParamM = 2
	ParamA = 3
	ParamS = 4
)
const (
	OpNone = 0
	OpLessThan = 1
	OpGreaterThan = 2
)
const (
	DestinationReject = "R"
	DestinationAccept = "A"
)

type Range struct {
	From int
	To int
}
func EmptyRange() Range {
	return Range{1,0}
}
func (r Range) Len() int {
	if r.From > r.To {
		return 0
	}
	return r.To - r.From + 1
}
func (r Range) Contains(v int) bool {
	return v >= r.From && v <= r.To
}
/// Returns two ranges. The first one satisfies the op, the second one is the remainder.
func (r Range) SplitByOp(op int, value int) (Range, Range) {
	var sat = r
	var rem = EmptyRange()
	switch op {
		case OpNone: {}
		case OpLessThan: {
			var split = aoc.MinI(r.To,value-1)
			sat = Range{r.From,split}
			rem = Range{split+1,r.To}
		}
		case OpGreaterThan: {
			var split = aoc.MaxI(r.From,value+1)
			sat = Range{split,r.To}
			rem = Range{r.From,split-1}
		}
		default: panic("invalid op")
	}

	return sat, rem
}

type RatingRanges struct {
	X Range
	M Range
	A Range
	S Range
}
func EmptyRatingRanges() RatingRanges {
	return RatingRanges{EmptyRange(),EmptyRange(),EmptyRange(),EmptyRange()}
}
func (r RatingRanges) Combinations() int {
	return r.X.Len() * r.M.Len() * r.A.Len() * r.S.Len()
}

type WorkflowRule struct {
	Param int
	Op int
	Value int
	Destination string
}
func ParseWorkflowRule(input string) WorkflowRule {
	if !strings.Contains(input, ":") {
		return WorkflowRule{ParamNone,OpNone,0,input}
	}

	var parts = strings.Split(input, ":")
	var destination = parts[1]

	var param = ParamNone
	switch parts[0][0] {
		case 'x': param = ParamX
		case 'm': param = ParamM
		case 'a': param = ParamA
		case 's': param = ParamS
		default: panic("invalid param")
	}

	var op = OpNone
	switch parts[0][1] {
		case '<': op = OpLessThan
		case '>': op = OpGreaterThan
		default: panic("invalid op")
	}

	var value, _ = strconv.Atoi(string(parts[0][2:]))

	return WorkflowRule{param,op,value,destination}
}
func (r WorkflowRule) Evaluate(part Part) (bool, string) {
	var param = 0
	switch r.Param {
		// last rule
		case ParamNone: return true, r.Destination
		case ParamX: param = part.X
		case ParamM: param = part.M
		case ParamA: param = part.A
		case ParamS: param = part.S
		default: panic("invalid param spec")
	}

	var passed = false
	switch r.Op {
		case OpLessThan: passed = param < r.Value
		case OpGreaterThan: passed = param > r.Value
		default: panic("invalid op spec")
	}

	return passed, r.Destination
}
/// Returns two rating ranges. The first one satisfies the rule, the second one is the remainder.
func (r WorkflowRule) EvaluateRanges(ranges RatingRanges) (RatingRanges, RatingRanges) {
	var sat = ranges
	var rem = EmptyRatingRanges()
	switch r.Param {
		case ParamNone: {}
		case ParamX: {
			var xsat, xrem = ranges.X.SplitByOp(r.Op, r.Value)
			sat = RatingRanges{xsat,ranges.M,ranges.A,ranges.S}
			rem = RatingRanges{xrem,ranges.M,ranges.A,ranges.S}
		}
		case ParamM: {
			var msat, mrem = ranges.M.SplitByOp(r.Op, r.Value)
			sat = RatingRanges{ranges.X,msat,ranges.A,ranges.S}
			rem = RatingRanges{ranges.X,mrem,ranges.A,ranges.S}
		}
		case ParamA: {
			var asat, arem = ranges.A.SplitByOp(r.Op, r.Value)
			sat = RatingRanges{ranges.X,ranges.M,asat,ranges.S}
			rem = RatingRanges{ranges.X,ranges.M,arem,ranges.S}
		}
		case ParamS: {
			var ssat, srem = ranges.S.SplitByOp(r.Op, r.Value)
			sat = RatingRanges{ranges.X,ranges.M,ranges.A,ssat}
			rem = RatingRanges{ranges.X,ranges.M,ranges.A,srem}
		}
		default: panic("invalid param")
	}

	return sat, rem
}

type DestinatedRatingRanges struct {
	Ranges RatingRanges
	Destination string
}
type Workflow struct {
	Name string
	Rules []WorkflowRule
}
func (w Workflow) Evaluate(part Part) string {
	for _, r := range w.Rules {
		var passed, destination = r.Evaluate(part)
		if passed { return destination }
	}

	panic("invalid workflow spec")
}
func (w Workflow) EvaluateRanges(ranges RatingRanges) []DestinatedRatingRanges {
	var r = make([]DestinatedRatingRanges, 0)

	var current = ranges
	for _, rule := range w.Rules {
		var sat, rem = rule.EvaluateRanges(current)
		if sat.Combinations() > 0 {
			r = append(r, DestinatedRatingRanges{sat,rule.Destination})
		}
		current = rem
	}
	if current.Combinations() > 0 {
		panic("invalid workflow spec")
	}

	return r
}

type Part struct {
	X int
	M int
	A int
	S int
}
func ParsePart(input string) Part {
	var parts = strings.Split(strings.Trim(input, "{}"), ",")

	var x = 0
	var m = 0
	var a = 0
	var s = 0
	for _, p := range parts {
		var parts = strings.Split(p, "=")
		var value, _ = strconv.Atoi(parts[1])
		switch parts[0] {
			case "x": x = value
			case "m": m = value
			case "a": a = value
			case "s": s = value
			default: panic("invalid part value name")
		}
	}

	return Part{x,m,a,s}
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var parts = make([]Part, 0)
	var workflows = make(map[string]Workflow)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)

		if strings.HasPrefix(t, "{") {
			// part
			parts = append(parts, ParsePart(t))
		} else {
			// workflow
			var parts = strings.Split(t, "{")
			var name = parts[0]
			var workflow = strings.TrimRight(parts[1], "}")
			var rulesStr = strings.Split(workflow, ",")

			var rules = make([]WorkflowRule, 0)
			for _, r := range rulesStr {
				rules = append(rules, ParseWorkflowRule(r))
			}
			workflows[name] = Workflow{name,rules}
		}
	}
	aoc.LogTrace("workflows: %v\n", workflows)
	aoc.LogTrace("parts: %v\n", parts)

	for i, part := range parts {
		aoc.LogDebug("Evaluating part %d\n", i)
		var currentWorkflow = workflows["in"]
		var done = false
		for !done {
			aoc.LogDebug("Evaluating workflow %s\n", currentWorkflow.Name)
			var destination = currentWorkflow.Evaluate(part)
			switch destination {
				case DestinationReject: done = true
				case DestinationAccept: {
					result += part.X + part.M + part.A + part.S
					done = true
				}
				default: currentWorkflow = workflows[destination]
			}
		}
	}

	{
		var currentRanges = append([]DestinatedRatingRanges(nil), DestinatedRatingRanges{
			RatingRanges{
				Range{1,4000},
				Range{1,4000},
				Range{1,4000},
				Range{1,4000},
			},
			"in",
		})
		for len(currentRanges) > 0 {
			var c = currentRanges[0]
			currentRanges = currentRanges[1:]

			switch c.Destination {
				case DestinationReject: {}
				case DestinationAccept: {
					result2 += c.Ranges.Combinations()
				}
				default: {
					currentRanges = append(currentRanges, workflows[c.Destination].EvaluateRanges(c.Ranges)...)
				}
			}
		}
	}

	fmt.Println(result, result2)
}
