require("aoc_2024/libaoc")
local Vector2 = aoc.Vector2(int32)
local Matrix = aoc.Matrix

local Cell = aoc.Enum({"EMPTY", "."}, {"WALL", "#"}, {"START", "S"}, {"END", "E"}, {"BEST", "O"})

local input_lines = aoc.read_lines(arg[1])

local map = Matrix.new(#input_lines[1], #input_lines)
local start_pos = Vector2.new(0, 0)
local end_pos = Vector2.new(0, 0)
for x, y, _ in Matrix.iter(map) do
	local char = input_lines[y]:sub(x, x)
	local cell = Cell.try_parse(char)

	if cell == Cell.START then
		cell = Cell.EMPTY
		start_pos = Vector2.new(x, y)
	elseif cell == Cell.END then
		cell = Cell.EMPTY
		end_pos = Vector2.new(x, y)
	end
	map:set(x, y, cell)
end

-- print(Matrix.dump(map, Cell.dump))
-- print(aoc.dump(start_pos), aoc.dump(end_pos))

local function print_state(map, best_tiles)
	local copy = map:copy_shallow()
	for _, t in pairs(best_tiles) do
		copy:set(t.x, t.y, Cell.BEST)
	end
	print(copy:dump(Cell.dump))
end

local function seen_key(pos, dir)
	return aoc.dump(pos) .. ";" .. aoc.dump(dir)
end
local function find_min_paths(map, start_pos, end_pos)
	local start_dir = Vector2.new(1, 0)
	
	local seen_at_cost = {}
	aoc.map_insert(seen_at_cost, seen_key(start_pos, start_dir), 0)
	
	local function trans_fn(head, next_pos)
		-- print_state(map, head.path)

		local next_cell = map:get(next_pos.x, next_pos.y)
		if next_cell ~= Cell.EMPTY then
			return nil
		end

		local next_cost = head.cost + 1
		local next_dir = next_pos:sub(head.pos)
		if next_dir:eq(head.state:rot_180()) then
			next_cost = next_cost + 2000
		elseif not head.state:eq(next_dir) then
			next_cost = next_cost + 1000
		end

		local next_seen_key = seen_key(next_pos, next_dir)
		local seen_cost = aoc.map_get(seen_at_cost, next_seen_key)
		if seen_cost ~= nil and seen_cost < next_cost then
			return nil
		end
		aoc.map_insert(seen_at_cost, next_seen_key, next_cost)

		return {
			cost = next_cost,
			state = next_dir
		}
	end
	local function end_fn(head)
		return head.pos:eq(end_pos)
	end

	local gen = aoc.find_2d_paths(Vector2, { pos = start_pos, state = start_dir }, end_fn, trans_fn)
	local min_paths = { gen() }
	for min_path in gen do
		if min_path.cost > min_paths[1].cost then
			break
		end
		table.insert(min_paths, min_path)
		-- print_state(map, min_path.path)
	end
	return min_paths
end
local min_paths = find_min_paths(map, start_pos, end_pos)
print("min score", min_paths[1].cost)

-- Part 2
local seen_best_path = {}
aoc.map_insert(seen_best_path, end_pos, end_pos)
for _, min_path in pairs(min_paths) do
	for _, pos in pairs(min_path.path) do
		aoc.map_insert(seen_best_path, pos, pos)
	end
end
-- print_state(map, aoc.values(seen_best_path))
print("total best tiles", #aoc.keys(seen_best_path))
