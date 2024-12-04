require("aoc_2024/libaoc")

-- table of rows
local input_matrix = {}

local input_file = io.open(arg[1], "r")
while true do
	line = input_file:read("l")
	if line == nil then
		break
	end

	table.insert(input_matrix, line)
end
input_file:close()

local height = #input_matrix
local width = #input_matrix[1]

struct Vector2 {
	x: double
	y: double
}
terra Vector2:add(rhs: Vector2)
	return Vector2 { x = self.x + rhs.x, y = self.y + rhs.y }
end
terra Vector2:sub(rhs: Vector2)
	return Vector2 { x = self.x - rhs.x, y = self.y - rhs.y }
end
terra Vector2:mul(a: double)
	return Vector2 { x = self.x * a, y = self.y * a }
end
terra Vector2:neg()
	return self:mul(-1)
end
Vector2.new = function(x, y)
	local v = terralib.new(Vector2)
	v.x = x
	v.y = y
	return v
end
Vector2.from_array = function(arr)
	return Vector2.new(arr[1], arr[2])
end

local function search_dir(target, matrix, pos, dir)
	local i = 1
	while i <= #target do
		local row = matrix[pos.y]
		if row == nil then
			return false
		end

		local cell = row:sub(pos.x, pos.x)
		if #cell == 0 or cell ~= target:sub(i, i) then
			return false
		end

		i = i + 1
		pos = pos:add(dir)
	end
	return true
end

-- Part 1
local target_count = 0
local target = "XMAS"
local dirs = aoc.map(Vector2.from_array, { {1, 0}, {-1, 0}, {0, 1}, {0, -1}, {1, 1}, {1, -1}, {-1, 1}, {-1, -1} })
for y = 1, width do
	for x = 1, height do
		for _, dir in pairs(dirs) do
			if search_dir(target, input_matrix, Vector2.new(x, y), dir) then
				target_count = target_count + 1
			end
		end
	end
end
print("target_count", target_count)

-- Part 2
local cross_count = 0
local cross_target = "MAS"
local cross_dirs = aoc.map(Vector2.from_array, { {1, 1}, {1, -1} })
for y = 1, width do
	for x = 1, height do
		local pos = Vector2.new(x, y)
		local all_cross_match = true
		for _, dir in pairs(cross_dirs) do
			local cross_match = search_dir(cross_target, input_matrix, pos:sub(dir), dir) 
				or search_dir(cross_target, input_matrix, pos:add(dir), dir:neg())
			if not cross_match then
				all_cross_match = false
				break
			end
		end
		if all_cross_match then
			cross_count = cross_count + 1
		end
	end
end
print("cross_count", cross_count)
