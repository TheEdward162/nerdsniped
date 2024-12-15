require("aoc_2024/libaoc")
local Vector2 = aoc.Vector2(int32)
local Matrix = aoc.Matrix

local function parse_vec(str)
	local x, y = unpack(aoc.string_split(",", str))
	return Vector2.new(
		tonumber(x),
		tonumber(y)
	)
end

local input_lines = aoc.read_lines(arg[1])
local robots = {}
for _, line in pairs(input_lines) do
	local p_str, v_str = unpack(aoc.string_split_space(line))
	local p = parse_vec(aoc.string_strip_prefix("p=", p_str))
	local v = parse_vec(aoc.string_strip_prefix("v=", v_str))

	table.insert(robots, { start = p, pos = p, dir = v })
end
-- print(aoc.dump(robots))

local function print_robots(area_size, robots)
	local map = Matrix.new(area_size.x, area_size.y)
	for y = 1, map.height do
		for x = 1, map.width do
			map:set(x, y, 0)
		end
	end

	for _, robot in pairs(robots) do
		local v = map:get(robot.pos.x + 1, robot.pos.y + 1)
		map:set(robot.pos.x + 1, robot.pos.y + 1, v + 1)
	end

	for y = 1, map.height do
		for x = 1, map.width do
			local v = map:get(x, y)
			if v == 0 then
				map:set(x, y, " ")
			end
		end
	end
	print(aoc.dump(map))
end

-- Part 1
local area_size = Vector2.new(101, 103)
if #robots == 12 then
	area_size = Vector2.new(11, 7)
end
local area_middle = area_size:div(2)

local simulate_steps = 100
for i = 1, simulate_steps do
	for robot_i, robot in ipairs(robots) do
		robot.pos = Vector2.new(
			(robot.pos.x + robot.dir.x) % area_size.x,
			(robot.pos.y + robot.dir.y) % area_size.y
		)
		-- if robot.pos:eq(robot.start) then
		-- 	print("robot " .. robot_i .. " period " .. i)
		-- end
	end
end


local quadrants = { 0, 0, 0, 0 }
for _, robot in pairs(robots) do
	local quandrant_index = nil
	if robot.pos.x < area_middle.x and robot.pos.y < area_middle.y then
		quandrant_index = 1
	elseif robot.pos.x < area_middle.x and robot.pos.y > area_middle.y then
		quandrant_index = 3
	elseif robot.pos.x > area_middle.x and robot.pos.y < area_middle.y then
		quandrant_index = 2
	elseif robot.pos.x > area_middle.x and robot.pos.y > area_middle.y then
		quandrant_index = 4
	end

	if quandrant_index ~= nil then
		quadrants[quandrant_index] = quadrants[quandrant_index] + 1
	end
end
print("safety factor", aoc.product(quadrants))

local max_steps = 10403
local result = 8050
for i = simulate_steps + 1, result do
	for robot_i, robot in ipairs(robots) do
		robot.pos = Vector2.new(
			(robot.pos.x + robot.dir.x) % area_size.x,
			(robot.pos.y + robot.dir.y) % area_size.y
		)
	end

	-- for y = 1, area_size.y do
	-- 	local continuous_x = 0
	-- 	for x = 1, area_size.x do
	-- 		for _, robot in pairs(robots) do
	-- 			if robot.pos.x == x then
	-- 				continuous_x = continuous_x + 1
	-- 			else
	-- 				continuous_x = 0
	-- 			end
	-- 		end
	-- 	end

	-- 	if continuous_x == 31 then
	-- 		print_robots(area_size, robots)
	-- 		print(i)
	-- 	end
	-- end
end
print_robots(area_size, robots)
