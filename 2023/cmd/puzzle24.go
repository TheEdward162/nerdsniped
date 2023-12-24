package main

import (
	aoc "aoc_commons"
	"fmt"
	"bufio"
	"math"
	"strings"
)

type Point = aoc.Point3[float64]
type Matrix = [9]float64

func matrixMulScalar(m Matrix, f float64) Matrix {
	return Matrix{
		m[0] * f, m[1] * f, m[2] * f,
		m[3] * f, m[4] * f, m[5] * f,
		m[6] * f, m[7] * f, m[8] * f,
	}
}
func matrixMulVec(m Matrix, v Point) Point {
	return Point{
		m[0] * v.X + m[1] * v.Y  + m[2] * v.Z,
		m[3] * v.X + m[4] * v.Y  + m[5] * v.Z,
		m[6] * v.X + m[7] * v.Y  + m[8] * v.Z,
	}
}
func matrixDeterminant(m Matrix) float64 {
	return m[0]*m[4]*m[8] + m[1]*m[5]*m[6] + m[2]*m[3]*m[7] - m[2]*m[4]*m[6] - m[1]*m[3]*m[8] - m[0]*m[5]*m[7]
}
func matrixTranspose(m Matrix) Matrix {
	return Matrix{
		m[0], m[3], m[6],
		m[1], m[4], m[7],
		m[2], m[5], m[8],
	}
}
func matrixCofactor(m Matrix) Matrix {
	return Matrix{
		(m[4]*m[8]-m[5]*m[7]), -(m[3]*m[8]-m[5]*m[6]), (m[3]*m[7]-m[4]*m[6]),
		-(m[1]*m[8]-m[2]*m[7]), (m[0]*m[8]-m[2]*m[6]), -(m[0]*m[7]-m[1]*m[6]),
		(m[1]*m[5]-m[2]*m[4]), -(m[0]*m[5]-m[2]*m[3]), (m[0]*m[4]-m[1]*m[3]),
	}
}
func matrixAdjugate(m Matrix) Matrix {
	return matrixTranspose(matrixCofactor(m))
}
func matrixInverse(m Matrix) (Matrix, bool) {
	var det = matrixDeterminant(m)
	if det == 0 {
		return Matrix{}, false
	}
	var inverse = matrixMulScalar(matrixAdjugate(m), 1.0 / det)

	return inverse, true
}

type Ray struct {
	Pos Point
	Dir Point
}
func (r Ray) parametericAt(param float64) Point {
	return r.Pos.Add(r.Dir.Mul(param))
}

func RayIntersection(r1 Ray, r2 Ray) (float64, float64) {
	// we are solving Ax + b = 0, where:
	// A is [r1.Dir | r2.Dir]
	// b is P1 - P2
	// x is (t1, t2)
	var A = Matrix{
		r1.Dir.X,r2.Dir.X,0.0,
		r1.Dir.Y,r2.Dir.Y,0.0,
		r1.Dir.Z,r2.Dir.Z,1.0,
	}
	var b = r1.Pos.Sub(r2.Pos)

	// find the inverse and multiply b from the left
	var Ai, ok = matrixInverse(A)
	if !ok {
		// if the inverse doesn't exist, the determinant was zero and the intersection doesn't exist
		return math.NaN(), math.NaN()
	}

	// x = A^-1 * -b
	var x = matrixMulVec(Ai, b.Mul(-1.0))

	return x.X, -x.Y
}

var TEST_RANGE_X = aoc.MakeRange[float64](200000000000000, 200000000000000)
var TEST_RANGE_Y = aoc.MakeRange[float64](200000000000000, 200000000000000)
func checkRays(rays []Ray, i int, j int) bool {
	var r1 = rays[i]
	var r2 = rays[j]

	// for part 1 we ignore Z coordinates
	r1.Pos.Z = 0.0
	r1.Dir.Z = 0.0
	r2.Pos.Z = 0.0
	r2.Dir.Z = 0.0

	var t1, t2 = RayIntersection(rays[i], rays[j])
	aoc.LogTrace("Rays %d, %d in %v, %v\n", i, j, t1, t2)
	if math.IsNaN(t1) || math.IsNaN(t2) || t1 < 0 || t2 < 0 {
		// either no intersection or intersection in the past
		return false
	}

	var intersection = rays[i].parametericAt(t1)
	if !TEST_RANGE_X.Contains(intersection.X) || !TEST_RANGE_Y.Contains(intersection.Y) {
		// outside of check range
		return false
	}

	aoc.LogDebug("Rays %d and %d intersect (at %v)\n", i, j, intersection)
	return true
}

func main() {
	var input, err = aoc.Initialize()
	if err != nil { panic(err) }

	var result = 0
	var result2 = 0

	var rays = make([]Ray, 0)

	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		var t = scanner.Text()
		if t == "" { continue }
		aoc.LogTrace("text = %s\n", t)

		var parts = strings.Split(t, "@")
		var positions = aoc.ParseIntListBy(parts[0], ",")
		var directions = aoc.ParseIntListBy(parts[1], ",")
		rays = append(rays, Ray{
			Point{float64(positions[0]),float64(positions[1]),float64(positions[2])},
			Point{float64(directions[0]),float64(directions[1]),float64(directions[2])},
		})
	}
	aoc.LogTrace("rays:%v\n", rays)

	// example input
	if len(rays) <= 5 {
		TEST_RANGE_X = aoc.MakeRange[float64](7, 27)
		TEST_RANGE_Y = aoc.MakeRange[float64](7, 27)
	}

	for i := 0; i < len(rays); i += 1 {
		for j := i + 1; j < len(rays); j += 1 {
			if checkRays(rays, i, j) {
				result += 1
			}
		}
	}

	// run this in your python interpreter
	var solverInput = "import z3\n"
	solverInput += "dm_x = z3.BitVec(\"dm_x\", 64)\n"
	solverInput += "dm_y = z3.BitVec(\"dm_y\", 64)\n"
	solverInput += "dm_z = z3.BitVec(\"dm_z\", 64)\n"
	solverInput += "pm_x = z3.BitVec(\"pm_x\", 64)\n"
	solverInput += "pm_y = z3.BitVec(\"pm_y\", 64)\n"
	solverInput += "pm_z = z3.BitVec(\"pm_z\", 64)\n"
	solverInput += "solver = z3.Solver()\n"
	for i, ray := range rays[:3] {
		solverInput += fmt.Sprintf("t%d = z3.BitVec(\"t%d\", 64)\n", i, i)
		solverInput += fmt.Sprintf("solver.add(t%d > 0)\n", i)
		solverInput += fmt.Sprintf("solver.add(%f + t%d * %f == t%d * dm_x + pm_x)\n", ray.Pos.X, i, ray.Dir.X, i)
		solverInput += fmt.Sprintf("solver.add(%f + t%d * %f == t%d * dm_y + pm_y)\n", ray.Pos.Y, i, ray.Dir.Y, i)
		solverInput += fmt.Sprintf("solver.add(%f + t%d * %f == t%d * dm_z + pm_z)\n", ray.Pos.Z, i, ray.Dir.Z, i)
	}
	solverInput += "print(solver.check())\n"
	solverInput += "print(solver.model())\n"
	fmt.Println(solverInput)

	fmt.Println(result, result2)
}
