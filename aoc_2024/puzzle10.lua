require("aoc_2024/libaoc")
local ffi = require("ffi")
local Vector2 = aoc.Vector2(double)
local Matrix = aoc.Matrix

local input_lines = aoc.read_lines(arg[1])

local map = Matrix.new(#input_lines[1], #input_lines)
local trail_starts = {}
for y, line in ipairs(input_lines) do
	for x = 1, #line do
		local value = tonumber(line:sub(x, x))
		map:set(x, y, value)

		if value == 0 then
			table.insert(trail_starts, Vector2.new(x, y))
		end
	end
end

local function find_trails(map, start_pos)
	local function trans_fn(head, next_pos)
		local head_cell = map:get(head.pos.x, head.pos.y)
		local next_cell = map:get(next_pos.x, next_pos.y)
		if next_cell ~= head_cell + 1 then
			return nil
		end

		return {
			cost = head.cost + 1,
			state = nil
		}
	end
	local function end_fn(head)
		return map:get(head.pos.x, head.pos.y) == 9
	end

	local gen = aoc.find_2d_paths(Vector2, { pos = start_pos, state = nil }, end_fn, trans_fn)
	local trails = {}
	for trail in gen do
		table.insert(trails, trail)
	end
	return trails
end

-- Part 1 & Part 2
local reachable_ends_sum = 0
local total_trail_count = 0
for _, start in pairs(trail_starts) do
	local trails = find_trails(map, start)

	local trail_ends = {}
	for _, trail in pairs(trails) do
		aoc.set_insert(trail_ends, trail.path[#trail.path])
	end
	reachable_ends_sum = reachable_ends_sum + aoc.set_len(trail_ends)

	total_trail_count = total_trail_count + #trails
end
print("reachable_ends_sum", reachable_ends_sum)
print("total_trail_count", total_trail_count)
