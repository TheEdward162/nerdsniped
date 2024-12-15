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

local DIRS = { { 1, 0 }, { -1, 0 }, { 0, 1 }, { 0, -1 } }
local function find_trails(map, start)
	local reached_end = {}
	
	local current_heads = {}
	table.insert(current_heads, start)

	while #current_heads > 0 do
		local head = table.remove(current_heads)
		local head_value = map:get(head.x, head.y)
		for _, dir in pairs(DIRS) do
			local next = head:add(dir)
			local next_value = map:get(next.x, next.y)
			if next_value ~= nil and next_value - head_value == 1 then
				if next_value == 9 then
					table.insert(reached_end, next)
				else
					table.insert(current_heads, next)
				end
			end
		end
	end

	return reached_end
end

-- Part 1
local reachable_sum = 0
for _, start in pairs(trail_starts) do
	local trails = find_trails(map, start)
	local dedup_trails = {}
	for _, v in pairs(trails) do
		dedup_trails[aoc.dump(v)] = v
	end
	reachable_sum = reachable_sum + #aoc.keys(dedup_trails)
end
print("reachable_sum", reachable_sum)

-- Part 2
local reachable_ratings = 0
for _, start in pairs(trail_starts) do
	local trails = find_trails(map, start)
	reachable_ratings = reachable_ratings + #trails
end
print("reachable_ratings", reachable_ratings)
