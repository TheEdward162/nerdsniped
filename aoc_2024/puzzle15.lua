require("aoc_2024/libaoc")
local Vector2 = aoc.Vector2(int32)
local Matrix = aoc.Matrix

local Cell = aoc.Enum2({"EMPTY", "."}, {"WALL", "#"}, {"CRATE", "O"}, {"CRATE_L", "["}, {"CRATE_R", "]"}, {"ROBOT", "@"})
local Instruction = aoc.Enum2({"UP", "^"}, {"DOWN", "v"}, {"LEFT", "<"}, {"RIGHT", ">"})

local input_lines = aoc.read_lines(arg[1])
local matrix_height = 0
for y = 1, #input_lines do
	if input_lines[y] == "" then
		matrix_height = y - 1
		break
	end
end

local map_start = Matrix.new(#input_lines[1], matrix_height)
local robot_pos_start = Vector2.new(0, 0)
for y = 1, map_start.height do
	local line = input_lines[y]
	for x = 1, map_start.width do
		local char = line:sub(x, x)
		
		local cell = Cell.try_parse(char)
		if char == "@" then
			robot_pos_start = Vector2.new(x, y)
			cell = Cell.EMPTY
		end
		map_start:set(x, y, cell)
	end
end
-- print(map_start:dump(Cell.dump))
-- print(aoc.dump(robot_pos_start))

local instructions = {}
for i = matrix_height + 1, #input_lines do
	local line = input_lines[i]
	for j = 1, #line do
		local inst = Instruction.try_parse(line:sub(j, j))
		table.insert(instructions, inst)
	end
end
-- print(aoc.dump(instructions))

local instruction_dir = {
	[Instruction.UP] = Vector2.new(0, -1),
	[Instruction.DOWN] = Vector2.new(0, 1),
	[Instruction.LEFT] = Vector2.new(-1, 0),
	[Instruction.RIGHT] = Vector2.new(1, 0),
}

local function try_move_cell(map, pos, dir, commit)
	if commit == nil then
		commit = true
	end
	local cell = map:get(pos.x, pos.y)
	
	if cell == Cell.EMPTY then
		return true
	end
	if cell == Cell.WALL then
		return false
	end
	-- cell == Cell.CRATE or Cell.CRATE_L or Cell.CRATE_R

	local nexts = { pos:add(dir) }
	if dir.y ~= 0 and cell ~= Cell.CRATE then
		-- if dir is up or down we need to account for CRATE_L and CRATE_R
		local next_wide = nil
		if cell == Cell.CRATE_L then
			next_wide = pos:add(dir):add(Vector2.new(1, 0))
		elseif cell == Cell.CRATE_R then
			next_wide = pos:add(dir):add(Vector2.new(-1, 0))
		else
			error()
		end
		table.insert(nexts, next_wide)
	end

	-- simulate without commiting
	for _, next in pairs(nexts) do
		if not try_move_cell(map, next, dir, false) then
			return false
		end
	end
	if not commit then
		return true
	end

	-- actually commit
	for _, next in pairs(nexts) do
		try_move_cell(map, next, dir, true)
		local orig = next:sub(dir)
		local orig_cell = map:get(orig.x, orig.y)

		map:set(next.x, next.y, orig_cell)
		map:set(orig.x, orig.y, Cell.EMPTY)
	end

	return true
end

local function print_state(map, robot_pos)
	local tmp = map:get(robot_pos.x, robot_pos.y)
	map:set(robot_pos.x, robot_pos.y, Cell.ROBOT)
	print(map:dump(Cell.dump))
	map:set(robot_pos.x, robot_pos.y, tmp)
end

-- Part 1
local map = map_start:shallow_copy()
local robot_pos = robot_pos_start
for _, inst in pairs(instructions) do
	local dir = instruction_dir[inst]
	local next_pos = robot_pos:add(dir)
	
	local next_cell = map:get(next_pos.x, next_pos.y)
	if next_cell == Cell.EMPTY then
		robot_pos = next_pos
	elseif next_cell == Cell.CRATE then
		if try_move_cell(map, next_pos, dir) then
			robot_pos = next_pos
		end
	end
end

local sum_gps = 0
for y = 1, map.width do
	for x = 1, map.height do
		local cell = map:get(x, y)
		if cell == Cell.CRATE then
			sum_gps = sum_gps + (y - 1) * 100 + x - 1
		end
	end
end
print("sum_gps", sum_gps)

-- Part 2
local map_wide = Matrix.new(map_start.width * 2, map_start.height)
local robot_pos_wide = Vector2.new((robot_pos_start.x - 1) * 2 + 1, robot_pos_start.y)
for y = 1, map_start.height do
	for x = 1, map_start.width do
		local cell = map_start:get(x, y)
		local cell_wide = cell
		if cell == Cell.CRATE then
			cell = Cell.CRATE_L
			cell_wide = Cell.CRATE_R
		end

		local wide_x = (x - 1) * 2 + 1
		map_wide:set(wide_x, y, cell)
		map_wide:set(wide_x + 1, y, cell_wide)
	end
end
-- print_state(map_wide, robot_pos_wide)

for _, inst in pairs(instructions) do
	local dir = instruction_dir[inst]
	local next_pos = robot_pos_wide:add(dir)

	local next_cell = map_wide:get(next_pos.x, next_pos.y)
	if next_cell == Cell.EMPTY then
		robot_pos_wide = next_pos
	elseif next_cell ~= Cell.WALL then
		if try_move_cell(map_wide, next_pos, dir) then
			robot_pos_wide = next_pos
		end
	end

	-- print("inst", Instruction.dump(inst))
	-- print_state(map_wide, robot_pos_wide)
end

-- print_state(map_wide, robot_pos_wide)
local sum_gps2 = 0
for y = 1, map_wide.height do
	for x = 1, map_wide.width do
		local cell = map_wide:get(x, y)
		if cell == Cell.CRATE_L then
			sum_gps2 = sum_gps2 + (y - 1) * 100 + x - 1
		end
	end
end
print("sum_gps2", sum_gps2)
