package aoc_commons

import (
	"errors"
	"os"
	"io"
	"log"
	"fmt"
	"strings"
	"strconv"
	"slices"
)

const (
	LogLevelOff = iota
	LogLevelError
	LogLevelWarn
	LogLevelInfo
	LogLevelDebug
	LogLevelTrace
)
var logLevel = LogLevelError

type Cli struct {
	InputPath string
	LogLevel int
}
func parseCli() (*Cli, error) {	
	args := os.Args[1:]

	cli := new(Cli)
	cli.InputPath = ""
	cli.LogLevel = LogLevelError
	
	i := 0
	for i < len(args) {
		switch args[i] {
			case "--log-level":
				i += 1
				if len(args) <= i {
					return nil, errors.New("--log-level requires an argument")
				}
				switch args[i] {
					case "trace": cli.LogLevel = LogLevelTrace
					case "debug": cli.LogLevel = LogLevelDebug
					case "info": cli.LogLevel = LogLevelInfo
					case "warn": cli.LogLevel = LogLevelWarn
					case "error": cli.LogLevel = LogLevelError
					default:
						return nil, errors.New("Invalid --log-level value: " + args[i])
				}
			default:
				cli.InputPath = args[i]
		}

		i += 1
	}

	if cli.InputPath == "" {
		return nil, errors.New("Missing input file")
	}

	return cli, nil
}

func Initialize() (io.Reader, error) {	
	cli, err := parseCli()
	if err != nil {
		return nil, err
	}

	log.SetFlags(log.Lmicroseconds | log.Lshortfile)
	logLevel = cli.LogLevel
	if logLevel == LogLevelOff {
		log.SetOutput(io.Discard)
	} else {
		log.SetOutput(os.Stderr)
	}

	file, err := os.Open(cli.InputPath)
	if err != nil {
		return nil, err
	}

	return file, nil
}

/////////////
// Logging //
func LogEnabled(level int) bool {
	return level <= logLevel
}
func LogError(format string, a ...any) {
	if logLevel >= LogLevelError {
		log.Output(2, fmt.Sprintf("[E] " + format, a...))
	}
}
func LogWarn(format string, a ...any) {
	if logLevel >= LogLevelWarn {
		log.Output(2, fmt.Sprintf("[W] " + format, a...))
	}
}
func LogInfo(format string, a ...any) {
	if logLevel >= LogLevelInfo {
		log.Output(2, fmt.Sprintf("[I] " + format, a...))
	}
}
func LogDebug(format string, a ...any) {
	if logLevel >= LogLevelDebug {
		log.Output(2, fmt.Sprintf("[D] " + format, a...))
	}
}
func LogTrace(format string, a ...any) {
	if logLevel >= LogLevelTrace {
		log.Output(2, fmt.Sprintf("[T] " + format, a...))
	}
}

////////////////////////
// Parsing and slices //
func ParseIntListBy(t string, split string) []int {
	result := make([]int, 0)

	for _, s := range strings.Split(t, split) {
		value, err := strconv.Atoi(s)
		if err == nil {
			result = append(result, value)
		}
	}

	return result
}
func ParseIntList(t string) []int {
	return ParseIntListBy(t, " ")
}
func SplitGroupsFunc[T any](s []T, iseq func(T, T) bool) [][]T {
	var result = make([][]T, 0)

	if len(s) == 0 {
		result = append(result, s)
		return result
	}

	var runStart = 0
	var lastElement = s[0]
	for i, e := range s {
		if !iseq(e, lastElement) {
			result = append(result, s[runStart:i])
			runStart = i
			lastElement = e
		}
	}
	if runStart < len(s) {
		result = append(result, s[runStart:])
	}

	return result
}
func SplitGroups[T comparable](s []T) [][]T {
	return SplitGroupsFunc(s, func (a T, b T) bool { return a == b })
}
func ConcatSlices[T any](ss ...[]T) []T {
	var result = []T(nil)
	for _, s := range ss {
		result = append(result, s...)
	}

	return result
}
func AppendCopy[T any](base []T, values ...T) []T {
	var copy = make([]T, 0, len(base) + len(values))
	copy = append(copy, base...)
	copy = append(copy, values...)

	return copy
}

//////////
// Math //
func gcd(a int, b int) int {
	if (b == 0) { return a }
	return gcd(b, a % b)
}
func GreatestCommonDivisor(a int, nums ...int) int {
	var r = a
	for _, b := range nums {
		r = gcd(r, b)
	}

	return r
}
func LeastCommonMultiple(a int, nums ...int) int {
	var r = a
	for _, b := range nums {
		r = r * b / GreatestCommonDivisor(r, b)
	}
	
	return r
}

func MaxI(a int, b int) int {
	if a < b {
		return b
	}
	return a
}
func MinI(a int, b int) int {
	if a > b {
		return b
	}
	return a
}

//////////////
// Geometry //
type PointI2 struct {
	X int
	Y int
}
func (a PointI2) DistanceManhattan(b PointI2) int {
	var x = a.X - b.X
	var y = a.Y - b.Y
	if x < 0 { x = -x }
	if y < 0 { y = -y }

	return x + y
}
func (a PointI2) Add(b PointI2) PointI2 {
	return PointI2{a.X + b.X, a.Y + b.Y}
}
func (a PointI2) Sub(b PointI2) PointI2 {
	return PointI2{a.X - b.X, a.Y - b.Y}
}
func (a PointI2) LenSquared() int {
	return a.X * a.X + a.Y * a.Y
}

type Grid[T any] struct {
	grid [][]T
}
func MakeGrid[T any]() *Grid[T] {
	var g = new(Grid[T])
	g.grid = make([][]T, 0)
	return g
}
func (g *Grid[T]) Clone() *Grid[T] {
	var g2 = MakeGrid[T]()
	for y := 0; y < g.Height(); y += 1 {
		var row = make([]T, 0)
		for x := 0; x < g.Width(y); x += 1 {
			row = append(row, g.Get(PointI2{x, y}))
		}
		g2.AddRow(y, row)
	}

	return g2
}
func GridEqual[T comparable](a *Grid[T], b *Grid[T]) bool {
	if a.Height() != b.Height() {
		return false
	}
	
	for y := 0; y < a.Height(); y += 1 {
		if !slices.Equal(a.GetRow(y), b.GetRow(y)) {
			return false
		}
	}

	return true
}
func (g *Grid[T]) Width(y int) int {
	return len(g.grid[y])
}
func (g *Grid[T]) Height() int {
	return len(g.grid)
}
func (g *Grid[T]) Get(p PointI2) T {
	var zero T
	if p.Y < 0 || p.Y >= g.Height() {
		return zero
	}

	for p.X < 0 || p.X >= g.Width(p.Y) {
		return zero
	}

	return g.grid[p.Y][p.X]
}
func (g *Grid[T]) GetRow(y int) []T {
	return g.grid[y]
}
func (g *Grid[T]) GetColumn(x int) []T {
	var result = make([]T, g.Height())
	for y := 0; y < g.Height(); y += 1 {
		if x >= g.Width(y) {
			var zeroT T
			result = append(result, zeroT)
		} else {
			result = append(result, g.grid[y][x])
		}
	}

	return result
}
func (g *Grid[T]) Set(p PointI2, v T) {
	if p.Y < 0 || p.Y >= g.Height() {
		return
	}

	for p.X < 0 || p.X >= g.Width(p.Y) {
		return
	}

	g.grid[p.Y][p.X] = v
}
func (g *Grid[T]) SetRow(y int, r []T) {
	g.grid[y] = r
}
func (g *Grid[T]) AddRow(y int, r []T) {
	if y >= g.Height() {
		g.grid = append(g.grid, r)
	} else {
		var tmp = append(g.grid[:y], r)
		g.grid = append(tmp, g.grid[y:]...)
	}
}
func (g *Grid[T]) AddColumn(x int, c []T) {
	for y := 0; y < g.Height(); y += 1 {
		if x >= g.Width(y) {
			g.grid[y] = append(g.grid[y], c[y])
		} else {
			var tmp = append(g.grid[y][:x], c[y])
			g.grid[y] = append(tmp, g.grid[y][x:]...)
		}
	}
}
func (g *Grid[T]) FmtDebug(fmtTile func(T) string) string {
	var result = ""
	
	for y := 0; y < g.Height(); y += 1 {
		var row string
		for x := 0; x < g.Width(y); x += 1 {
			row = row + fmtTile(g.Get(PointI2{x,y}))
		}
		result += row + "\n"
	}

	return result
}
