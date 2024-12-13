require("aoc_2024/libaoc")
local Vector2 = aoc.Vector2
local Matrix = aoc.Matrix

local input_lines = aoc.read_lines(arg[1])
local map = Matrix.new(#input_lines[1], #input_lines)
for y = 1, map.height do
	for x = 1, map.width do
		local input_cell = input_lines[y]:sub(x, x)
		map:set(x, y, input_cell)
	end
end
-- print(aoc.dump(map))

local dirs = { Vector2.new(1, 0), Vector2.new(-1, 0), Vector2.new(0, 1), Vector2.new(0, -1) }
local function flood_search(map, start)
	local area = 0
	local perimeter = 0
	local sides = 0

	local seen = {}
	aoc.set_insert(seen, start)
	local heads = { start }
	while #heads > 0 do
		local head = table.remove(heads)
		local head_value = map:get(head.x, head.y)

		area = area + 1
		for _, dir in pairs(dirs) do
			local next = head:add(dir)

			local next_value = map:get(next.x, next.y)
			if next_value == head_value then
				if not aoc.set_has(seen, next) then
					aoc.set_insert(seen, next)
					table.insert(heads, next)
				end
			else
				-- increase perimeter, do not flood into
				perimeter = perimeter + 1

				-- decide if this is an edge of a side
				-- heuristic: given this situation:
				-- YN
				-- XH
				--
				-- when deciding whether the perimeter between H (head) and N (next) is a side
				-- we only say it is a side if X ~= H or X == H == Y
				local check_side_left = head:add(dir:rot_270())
				local check_side_left_value = map:get(check_side_left.x, check_side_left.y)
				local check_side_left_up = check_side_left:add(dir)
				if check_side_left_value ~= head_value or check_side_left_value == map:get(check_side_left_up.x, check_side_left_up.y) then
					sides = sides + 1
				end
			end
		end
	end

	return { area = area, perimeter = perimeter, sides = sides, seen = seen }
end

-- Part 1 & Part 2
local seen_all = {}
local total_price = 0 
local total_price_discount = 0
for y = 1, map.height do
	for x = 1, map.width do
		local pos = Vector2.new(x, y)
		if not aoc.set_has(seen_all, pos) then
			local flood = flood_search(map, pos)
			aoc.merge_into(seen_all, flood.seen)
			total_price = total_price + flood.area * flood.perimeter
			total_price_discount = total_price_discount + flood.area * flood.sides
		end
	end
end
print("total_price", total_price)
print("total_price_discount", total_price_discount)
