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
	local copy = map:shallow_copy()
	for _, t in pairs(best_tiles) do
		copy:set(t.x, t.y, Cell.BEST)
	end
	print(copy:dump(Cell.dump))
end

local function seen_key(pos, dir)
	return aoc.dump(pos) .. ";" .. aoc.dump(dir)
end
local function attempt_move(map, seen_tiles, heads, next_pos, next_dir, next_cost, next_prev)
	if map:get(next_pos.x, next_pos.y) == Cell.EMPTY then
		local next_seen_tile = aoc.map_get_default(seen_tiles, next_pos, {})
		local next_seen = aoc.map_get(next_seen_tile, next_dir)
		if next_seen == nil or next_seen.cost > next_cost then
			table.insert(heads, { pos = next_pos, dir = next_dir, cost = next_cost })
			aoc.map_insert(next_seen_tile, next_dir, { cost = next_cost, prev = next_prev })
		elseif next_seen.cost == next_cost then
			for _, e in pairs(next_prev) do
				table.insert(next_seen.prev, e)
			end
		end
	end
end

-- Part 1
local cost_move = 1
local cost_turn = 1000
local start_dir = Vector2.new(1, 0)

local heads = { { pos = start_pos, dir = start_dir, cost = 0 } }
local seen_tiles = {}
aoc.map_insert(seen_tiles, start_pos, { [aoc.dump(start_dir)] = { cost = 0, prev = {} } })
while #heads > 0 do
	local head = table.remove(heads)

	-- attempt a move
	attempt_move(map, seen_tiles, heads, head.pos:add(head.dir), head.dir, head.cost + cost_move, { head.pos })

	-- attempt rotate
	local next_prev = aoc.values(aoc.map_get(aoc.map_get(seen_tiles, head.pos), head.dir).prev)
	attempt_move(map, seen_tiles, heads, head.pos, head.dir:rot_90(), head.cost + cost_turn, next_prev)
	attempt_move(map, seen_tiles, heads, head.pos, head.dir:rot_270(), head.cost + cost_turn, next_prev)
end

local ends = aoc.map_get(seen_tiles, end_pos)
local best_ends = {}
for _, seen in pairs(ends) do
	if #best_ends == 0 or best_ends[1].cost > seen.cost then
		best_ends = { seen }
	elseif best_ends[1].cost == seen.cost then
		table.insert(best_ends, seen)
	end
end
print("min score", best_ends[1].cost)

-- Part 2
local seen_best_path = {}
aoc.map_insert(seen_best_path, end_pos, end_pos)
local tails = {}
for _, best_end in pairs(best_ends) do
	for _, prev in pairs(best_end.prev) do
		table.insert(tails, { pos = prev, dir = end_pos:sub(prev) })
	end
end
while #tails > 0 do
	local tail = table.remove(tails)
	aoc.map_insert(seen_best_path, tail.pos, tail.pos)
	local tail_prev = aoc.map_get(aoc.map_get(seen_tiles, tail.pos), tail.dir).prev

	for _, prev in pairs(tail_prev) do
		table.insert(tails, { pos = prev, dir = tail.pos:sub(prev) })
	end
end
-- print_state(map, aoc.values(seen_best_path))
print("total best tiles", #aoc.keys(seen_best_path))
