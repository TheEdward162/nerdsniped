require("aoc_2024/libaoc")
local Vector2 = aoc.Vector2(double)
local Matrix = aoc.Matrix
local Cell = aoc.Enum("EMPTY", "OBSTACLE")

local input_lines = aoc.read_lines(arg[1])

local map = Matrix.new(#input_lines[1], #input_lines)
local guard_start = nil

for y = 1, map.height do
	for x = 1, map.width do
		local input_cell = input_lines[y]:sub(x, x)
		if input_cell == "." then
			map:set(x, y, Cell.EMPTY)
		elseif input_cell == "#" then
			map:set(x, y, Cell.OBSTACLE)
		elseif input_cell == "^" then
			guard_start = { Vector2.new(x, y), Vector2.new(0, -1) }
			map:set(x, y, Cell.EMPTY)
		else
			error("Unknown input " .. input_cell)
		end
	end
end

-- Part 1
local function set_visited(visited, x, y, dir)
	local v = x .. "," .. y
	local existing = visited[v]
	visited[v] = dir

	return existing ~= nil and existing:eq(dir)
end

local function simulate_guard(map, guard_start)
	local visited = {}
	local guard = guard_start

	local guarding = true
	while guarding do
		local guard_pos = guard[1]
		local guard_dir = guard[2]
		if set_visited(visited, guard_pos.x, guard_pos.y, guard_dir) then
			return nil
		end

		local next_pos = nil
		for i = 1,3 do
			next_pos = guard_pos:add(guard_dir)
			if map:index(next_pos.x, next_pos.y) == nil then
				guarding = false
				break
			end

			local next_cell = map:get(next_pos.x, next_pos.y)
			if next_cell == Cell.OBSTACLE then
				guard_dir = guard_dir:rot_90()
			else
				break
			end
		end
		guard = { next_pos, guard_dir }
	end

	return visited
end

print("visited", #aoc.keys(simulate_guard(map, guard_start)))

-- Part 2
local looping_maps = 0
for y = 1, map.height do
	for x = 1, map.width do
		if map:get(x, y) == Cell.EMPTY then
			local changed_map = map:copy_shallow()
			changed_map:set(x, y, Cell.OBSTACLE)

			if simulate_guard(changed_map, guard_start) == nil then
				looping_maps = looping_maps + 1
			end
		end
	end
	print("y", y)
end
print("looping_maps", looping_maps)
