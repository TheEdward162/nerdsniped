package main

import (
	aoc "aoc_commons"
	"io"
	"fmt"
	"bufio"
	"strings"
)

const (
	SignalLow = 0
	SignalHigh = 1
)

type ActivationResult struct {
	Output string
	Signal int
}
type Module interface {
	Name() string
	Clone() Module
	Activate(input string, signal int) []ActivationResult
}

type NoopModule struct {
	name string
}
func (m *NoopModule) Name() string {
	return m.name
}
func (m *NoopModule) Clone() Module {
	return &NoopModule{}
}
func (m *NoopModule) Activate(input string, signal int) []ActivationResult {
	return nil
}

type ButtonModule struct {
	name string
	output string
}
func (m *ButtonModule) Name() string {
	return m.name
}
func (m *ButtonModule) Clone() Module {
	return &ButtonModule{m.name,m.output}
}
func (m *ButtonModule) Activate(input string, signal int) []ActivationResult {
	return append([]ActivationResult(nil), ActivationResult{m.output, SignalLow})
}

type BroadcasterModule struct {
	name string
	outputs []string
}
func (m *BroadcasterModule) Name() string {
	return m.name
}
func (m *BroadcasterModule) Clone() Module {
	return &BroadcasterModule{m.name,m.outputs}
}
func (m *BroadcasterModule) Activate(input string, signal int) []ActivationResult {
	var r = make([]ActivationResult, 0)
	for _, d := range m.outputs {
		r = append(r, ActivationResult{d,signal})
	}
	return r
}

type FlipFlopModule struct {
	name string
	state int
	outputs []string
}
func (m *FlipFlopModule) Name() string {
	return m.name
}
func (m *FlipFlopModule) Clone() Module {
	return &FlipFlopModule{m.name,m.state,m.outputs}
}
func (m *FlipFlopModule) Activate(input string, signal int) []ActivationResult {
	var r = make([]ActivationResult, 0)
	if signal == SignalHigh {
		return r
	}

	if m.state == SignalLow {
		m.state = SignalHigh
	} else {
		m.state = SignalLow
	}

	for _, d := range m.outputs {
		r = append(r, ActivationResult{d,m.state})
	}

	return r
}

type ConjunctionModule struct {
	name string
	inputMemory map[string]int
	outputs []string
}
func (m *ConjunctionModule) Name() string {
	return m.name
}
func (m *ConjunctionModule) Clone() Module {
	return &ConjunctionModule{m.name,aoc.CloneMap(m.inputMemory, func(a int) int { return a }),m.outputs}
}
func (m *ConjunctionModule) Activate(input string, signal int) []ActivationResult {
	var r = make([]ActivationResult, 0)
	m.inputMemory[input] = signal

	var pulse = SignalLow
	for _, last := range m.inputMemory {
		if last == SignalLow {
			pulse = SignalHigh
			break
		}
	}

	for _, d := range m.outputs {
		r = append(r, ActivationResult{d,pulse})
	}

	return r
}

type DetectionModuleEntry struct {
	Input string
	Signal int
}
type DetectionModule struct {
	inner Module
	Detections []DetectionModuleEntry
}
func (m *DetectionModule) Name() string {
	return m.inner.Name() + "_d"
}
func (m *DetectionModule) Clone() Module {
	return &DetectionModule{m.inner.Clone(),aoc.AppendCopy(m.Detections)}
}
func (m *DetectionModule) Activate(input string, signal int) []ActivationResult {
	m.Detections = append(m.Detections, DetectionModuleEntry{input,signal})
	
	return m.inner.Activate(input, signal)
}

type InFlightSignal struct {
	Source string
	Destination string
	Signal int
}
func InFlightFromActivationResult(source string, result []ActivationResult) []InFlightSignal {
	var r = make([]InFlightSignal, 0, len(result))
	for _, a := range result {
		r = append(r, InFlightSignal{source,a.Output,a.Signal})
	}

	return r
}

func pressButton(net map[string]Module) [2]int {
	var lowCount = 0
	var highCount = 0

	var signals = make([]InFlightSignal, 0)
	signals = append(
		signals,
		InFlightFromActivationResult("button", net["button"].Activate("", SignalLow))...
	)
	for len(signals) > 0 {
		var s = signals[0]
		signals = signals[1:]

		if s.Signal == SignalLow {
			lowCount += 1
		} else {
			highCount += 1
		}

		signals = append(
			signals,
			InFlightFromActivationResult(s.Destination, net[s.Destination].Activate(s.Source, s.Signal))...
		)
	}
	
	// aoc.LogDebug("Simulated press with %d low, %d high\n", lowCount, highCount)
	return [2]int{lowCount, highCount}
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	if false {
		fmt.Printf("graphviz:\n%s", fmtGraphviz(input))
		return
	}
	if true {
		fmt.Printf("lcm: %d\n", aoc.LeastCommonMultiple(3917, 3943, 3947, 4001))
	}

	var result = 0
	var result2 = 0

	var net = make(map[string]Module)
	// reverse map stores a list of inputs by outputs
	var reverseMap = make(map[string][]string)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)

		var parts = strings.Split(t, " -> ")
		var name = parts[0]
		var outputs = strings.Split(parts[1], ", ")
		switch {
			case strings.HasPrefix(name, "broadcaster"): {
				// broadcaster
				net[name] = &BroadcasterModule{name,outputs}
			}
			case strings.HasPrefix(name, "%"): {
				// flip flop
				name = strings.TrimPrefix(name, "%")
				net[name] = &FlipFlopModule{name,SignalLow,outputs}
			}
			case strings.HasPrefix(name, "&"): {
				// conjunction
				name = strings.TrimPrefix(name, "&")
				net[name] = &ConjunctionModule{name,make(map[string]int),outputs}
			}
		}
		for _, o := range outputs {
			reverseMap[o] = append(reverseMap[o], name)
		}
	}
	net["button"] = &ButtonModule{"button","broadcaster"}
	net["rx"] = &NoopModule{"rx"}
	// run through all conjuction modules and fill in their inputMemory map
	for k, m := range net {
		var cm, ok = m.(*ConjunctionModule)
		if ok {
			for _, i := range reverseMap[k] {
				cm.inputMemory[i] = SignalLow
			}
		}
	}
	aoc.LogTrace("net: %v\n", net)
	var net2 = aoc.CloneMap(net, func (a Module) Module { return a.Clone() })

	var signals = [2]int{0,0}
	for i := 0; i < 1000; i += 1 {
		var r = pressButton(net)
		signals[0] += r[0]
		signals[1] += r[1]
	}
	result = signals[0] * signals[1]
	aoc.LogInfo("result: %v\n", result)

	// to find the subgraphs we assume a certain graph shape and just find the first node with more that one input starting from rx and going backwards
	// the shape we are assuming is that just before rx there is an AND of outputs of inverters attached to each subgraph
	// this means that as each inverter receives a low signal, the final AND receives all high signals, sending a low signal to rx
	var subgraphOutputNodes = make([]string, 0)
	var currentReverse = "rx"
	for {
		var inputs = reverseMap[currentReverse]
		if len(inputs) == 1 {
			currentReverse = inputs[0]
			continue
		}

		subgraphOutputNodes = inputs
		break
	}
	// we attach detection modules to subgraph output nodes
	var detectionModules = make([]*DetectionModule, 0)
	for _, nodeName := range subgraphOutputNodes {
		var dm = &DetectionModule{net2[nodeName],nil}
		net2[nodeName] = dm
		detectionModules = append(detectionModules, dm)
	}
	// finally we press the button until we detect the period of each subgraph
	// periods are initialize to zero, and since no subgraph is expected to have a period of zero we use that as a sentinel to know we are not done yet
	var subgraphPeriods = make([]int, len(detectionModules))
	var pressCount = 0
	for {
		for _, dm := range detectionModules {
			dm.Detections = nil
		}
		pressButton(net2)
		pressCount += 1

		// detect for each subgraph if there was an attempt to send a low pulse
		for i, dm := range detectionModules {
			if subgraphPeriods[i] == 0 {
				var detectedLow = 0
				for _, d := range dm.Detections {
					if d.Signal == SignalLow {
						detectedLow += 1
					}
				}
				if detectedLow == 1 {
					subgraphPeriods[i] = pressCount
					aoc.LogInfo("period of %s is %d\n", dm.Name(), pressCount)
				}
			}
		}

		var gotAllPeriods = true
		for _, period := range subgraphPeriods {
			if period == 0 {
				gotAllPeriods = false
				break
			}
		}
		if gotAllPeriods { break }
	}
	result2 = aoc.LeastCommonMultiple(subgraphPeriods[0], subgraphPeriods[1:]...)

	fmt.Println(result, result2)
}

func fmtGraphviz(input io.Reader) string {
	var conjunctions = make([]string, 0)
	var flipflops = make([]string, 0)
	var edges = make([][2]string, 0)
	
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }

		var parts = strings.Split(t, " -> ")
		var name = parts[0]
		var outputs = strings.Split(parts[1], ", ")
		switch {
			case strings.HasPrefix(name, "%"): {
				name = strings.TrimPrefix(name, "%")
				flipflops = append(flipflops, name)
			}
			case strings.HasPrefix(name, "&"): {
				name = strings.TrimPrefix(name, "&")
				conjunctions = append(conjunctions, name)
			}
		}

		for _, o := range outputs {
			edges = append(edges, [2]string{name,o})
		}
	}

	var lines = make([]string, 0)
	lines = append(lines, "digraph puzzle20 {")
	lines = append(lines, "node [shape = doublecircle]; broadcaster; rx;")


	lines = append(lines, "node [shape=circle,fixedsize=true,width=0.9]; " + strings.Join(conjunctions, "; "))
	lines = append(lines, "node [shape=box]; " + strings.Join(flipflops, "; "))
	for _, edge := range edges {
		lines = append(lines, fmt.Sprintf("%s -> %s;", edge[0], edge[1]))
	}

	lines = append(lines, "}")
	return strings.Join(lines, "\n")
}
