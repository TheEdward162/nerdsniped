require("aoc_2024/libaoc")
local Vector2 = aoc.Vector2(int32)
local Matrix = aoc.Matrix

local Cell = aoc.Enum({"EMPTY", "."}, {"WALL", "#"}, {"PATH", "O"}, {"START", "S"}, {"END", "E"})

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

local function print_path(map, path)
	local m = map:copy_shallow()
	for _, pos in pairs(path) do
		m:set(pos.x, pos.y, Cell.PATH)
	end
	print(Matrix.dump(m, Cell.dump))
end

local function find_min_path(map, start_pos, end_pos)
	local seen = {}
	aoc.set_insert(seen, start_pos)
	
	local function trans_fn(head, next_pos)
		local next_cell = map:get(next_pos.x, next_pos.y)
	
		if next_cell ~= Cell.EMPTY then
			return nil
		end

		if aoc.set_has(seen, next_pos) then
			return nil
		end
		aoc.set_insert(seen, next_pos)

		local next_cost = head.cost + 1
		return {
			cost = next_cost,
			state = nil
		}
	end
	local function end_fn(head)
		return head.pos:eq(end_pos)
	end

	local gen = aoc.find_2d_paths(Vector2, { pos = start_pos, state = nil }, end_fn, trans_fn)
	return gen()
end

local function manhattan_radius(center, radius)
	return coroutine.wrap(function()
		for y_off = -radius, radius do
			for x_off = -radius, radius do
				local off = Vector2.new(x_off, y_off)
				if off:length_manhattan() <= radius then
					coroutine.yield(center:add(off))
				end
			end
		end
	end)
end

local function find_cheat_saved_costs(map, main_path, max_cheat_time, minimum_saved_cost)
	local main_path_index_lookup = {}
	for i, pos in ipairs(main_path) do
		aoc.map_insert(main_path_index_lookup, pos, i)
	end
	
	local saved_costs = {}
	for start_index = 1, #main_path do
		-- print(start_index .. "/" .. #main_path)
		-- if start_index % 100 == 0 then print(start_index .. "/" .. #main_path) end

		local start_pos = main_path[start_index]
		for reachable_pos in manhattan_radius(start_pos, max_cheat_time) do
			local end_index = aoc.map_get(main_path_index_lookup, reachable_pos)

			if end_index ~= nil then
				local saved_cost = (end_index - start_index) - start_pos:sub(reachable_pos):length_manhattan()
				if saved_cost >= minimum_saved_cost then
					table.insert(saved_costs, saved_cost)
				end
			end
		end
	end
	return saved_costs
end

local main_path = find_min_path(map, start_pos, end_pos)
print_path(map, main_path.path)

local min_saved_cost1 = 100
local min_saved_cost2 = 100
if map.width < 100 then
	min_saved_cost1 = 1 -- example
	min_saved_cost2 = 50
end

-- Part 1
local saved_costs = find_cheat_saved_costs(map, main_path.path, 2, min_saved_cost1)
print("saved_costs", #saved_costs)

-- Part 2
local saved_costs2 = find_cheat_saved_costs(map, main_path.path, 20, min_saved_cost2)
print("saved_costs2", #saved_costs2)
