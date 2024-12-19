require("aoc_2024/libaoc")
local Vector2 = aoc.Vector2(int32)
local Matrix = aoc.Matrix
local PriorityQueue = aoc.PriorityQueue

local Cell = aoc.Enum({"EMPTY", "."}, {"WALL", "#"}, {"PATH", "O"})

local input_lines = aoc.read_lines(arg[1])

local map = Matrix.new(71, 71)
local start_pos = Vector2.new(1, 1)
local end_pos = Vector2.new(71, 71)
local fallen_bytes_count = 1024

-- for example problem
if #input_lines < 1024 then
	map = Matrix.new(7, 7)
	end_pos = Vector2.new(7, 7)
	fallen_bytes_count = 12
end

for x, y, _ in Matrix.iter(map) do
	map:set(x, y, Cell.EMPTY)
end

local falling_bytes = aoc.map(
	function(l)
		local x, y = unpack(aoc.string_split(",", l))
		return Vector2.new(tonumber(x) + 1, tonumber(y) + 1)
	end,
	input_lines
)
-- print(Matrix.dump(map, Cell.dump))
-- print(aoc.dump(falling_bytes))

local function print_path(map, path)
	local m = map:copy_shallow()
	for _, pos in pairs(path) do
		m:set(pos.x, pos.y, Cell.PATH)
	end
	print(Matrix.dump(m, Cell.dump))
end

local DIRS = { Vector2.new(1, 0), Vector2.new(-1, 0), Vector2.new(0, 1), Vector2.new(0, -1) }
local function find_min_path(map, start_pos, end_pos)
	local heads = PriorityQueue.new(function(a, b)
		return a.cost - b.cost
	end)
	heads:insert({ pos = start_pos, cost = 0, path = { start_pos } })

	local seen_global = {}
	aoc.map_insert(seen_global, start_pos, 0)

	while heads:len() > 0 do
		local head = heads:pop()

		if head.pos:eq(end_pos) then
			return { path = head.path, cost = head.cost }
		else
			for _, dir in pairs(DIRS) do
				local next_pos = head.pos:add(dir)
				local next_cost = head.cost + 1
				local visited_global = aoc.map_get(seen_global, next_pos)
				if visited_global == nil or visited_global > next_cost then
					aoc.map_insert(seen_global, next_pos, next_cost)
					local next_cell = map:get(next_pos.x, next_pos.y)
					if next_cell == Cell.EMPTY then
						local next_path = aoc.append(head.path, next_pos)
						heads:insert(
							{ pos = next_pos, cost = next_cost, path = next_path }
						)
					end
				end
			end
		end
	end

	return nil
end

-- Part 1
local map_fallen = map:copy_shallow()
for i = 1, fallen_bytes_count do
	local byte_pos = falling_bytes[i]
	map_fallen:set(byte_pos.x, byte_pos.y, Cell.WALL)
end
print(Matrix.dump(map_fallen, Cell.dump))

local best_path = find_min_path(map_fallen, start_pos, end_pos)
print("best path", best_path.cost)
print_path(map_fallen, best_path.path)

-- Part 2
local final_byte_index = nil
for i = fallen_bytes_count + 1, #falling_bytes do
	local byte_pos = falling_bytes[i]
	map_fallen:set(byte_pos.x, byte_pos.y, Cell.WALL)
	if aoc.contains(best_path.path, byte_pos, Vector2.methods.eq) then
		best_path = find_min_path(map_fallen, start_pos, end_pos)
		if best_path == nil then
			final_byte_index = i
			break
		end
		print_path(map_fallen, best_path.path)
	end
end
print("final_byte", final_byte_index, aoc.dump(falling_bytes[final_byte_index]:sub(Vector2.new(1, 1))))
