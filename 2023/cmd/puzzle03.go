package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"unicode"
	"strconv"
)

const (
	SchemaObjectDot = iota
	SchemaObjectSymbol
	SchemaObjectNumber
)
type SchemaObject struct {
	Start int
	End int
	Type int
	Value int
}
func parseSchemaObject(schema string, offset int) SchemaObject {
	t := schema[offset:]
	
	if len(t) == 0 {
		return SchemaObject { -1, 0, SchemaObjectDot, 0 }
	}

	if t[0] == '.' {
		return SchemaObject { offset, offset + 1, SchemaObjectDot, 0 }
	}

	numLen := 0
	// asuming `schema` is ascii
	for len(t) > numLen && unicode.IsNumber(rune(t[numLen])) {
		numLen += 1
	}
	if numLen > 0 {
		value, err := strconv.Atoi(t[:numLen])
		if err != nil { panic(err) }
		return SchemaObject { offset, offset + numLen, SchemaObjectNumber, value }
	}

	return SchemaObject { offset, offset + 1, SchemaObjectSymbol, int(t[0]) }
}
func findInArea(rows [][]SchemaObject, start int, end int, findFn func(SchemaObject) bool) []SchemaObject {
	var result []SchemaObject = make([]SchemaObject, 0, 2)
	
	for _, row := range rows {
		for _, obj := range row {
			if obj.Start < end && obj.End > start && findFn(obj) {
				result = append(result, obj)
			}
		}
	}

	return result
}

func main() {
	input, err := aoc.Initialize()
	if err != nil { panic(err) }

	var schematic [][]SchemaObject = make([][]SchemaObject, 0)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		t := scanner.Text()
		aoc.LogTrace("text = %s\n", t)

		var objects []SchemaObject = make([]SchemaObject, 0, 10)
		offset := 0
		for offset < len(t) {
			object := parseSchemaObject(t, offset)
			offset = object.End

			if object.Type != SchemaObjectDot {
				objects = append(objects, object)
			}
		}

		aoc.LogDebug("objects: %v\n", objects)
		schematic = append(schematic, objects)
	}

	result := 0
	result2 := 0
	for row := 0; row < len(schematic); row += 1 {
		var prevRow []SchemaObject = nil
		var nextRow []SchemaObject = nil
		if row > 0 {
			prevRow = schematic[row - 1]
		}
		if row < len(schematic) - 1 {
			nextRow = schematic[row + 1]
		}

		for _, object := range schematic[row] {
			if object.Type == SchemaObjectNumber {
				symbols := findInArea(
					[][]SchemaObject{prevRow, schematic[row], nextRow},
					object.Start - 1, object.End + 1,
					func (o SchemaObject) bool { return o.Type == SchemaObjectSymbol },
				)

				if len(symbols) > 0 {
					result += object.Value
				}
			} else if object.Type == SchemaObjectSymbol && object.Value == int('*') {
				numbers := findInArea(
					[][]SchemaObject{prevRow, schematic[row], nextRow},
					object.Start - 1, object.End + 1,
					func (o SchemaObject) bool { return o.Type == SchemaObjectNumber },
				)

				if len(numbers) > 2 {
					aoc.LogWarn("Found more than two numbers around a gear at %d:%d-%d\n", row, object.Start, object.End)
				}
				if len(numbers) >= 2 {
					result2 += numbers[0].Value * numbers[1].Value
				}
			}
		}
	}

	fmt.Println(result, result2)
}
