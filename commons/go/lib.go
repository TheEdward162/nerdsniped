package aoc_commons

import (
	"errors"
	"os"
	"io"
	"log"
	"fmt"
	"strings"
	"strconv"
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

func ParseIntList(t string) []int {
	result := make([]int, 0)

	for _, s := range strings.Split(t, " ") {
		value, err := strconv.Atoi(s)
		if err == nil {
			result = append(result, value)
		}
	}

	return result
}

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
	var g = GreatestCommonDivisor(a, nums...)
	for _, b := range nums {
		r = r * b / g
	}
	
	return r
}
