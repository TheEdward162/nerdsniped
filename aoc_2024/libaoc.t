aoc = {}

function aoc.read_lines(path)
	local lines = {}
	
	local input_file = io.open(arg[1], "r")
	while true do
		local line = input_file:read("l")
		if line == nil then
			break
		end

		table.insert(lines, line)
	end
	input_file:close()

	return lines
end

-- Strings

function aoc.string_split(delim, str)
	local res = {}
	for sub_str in string.gmatch(str, "([^" ..delim .. "]+)") do
		table.insert(res, sub_str)
	end
	return res
end

function aoc.string_split_space(str)
	return aoc.string_split(" ", str)
end

-- Tables

function aoc.map(f, tab)
	local res = {}
	for k, v in pairs(tab) do
		res[k] = f(v)
	end
	return res
end

function aoc.filter(f, tab)
	local res = {}
	for k, v in pairs(tab) do
		if f(v) then
			res[k] = v
		end
	end
	return res
end

function aoc.split(f, tab)
	local res_pos = {}
	local res_neg = {}
	for k, v in pairs(tab) do
		if f(v) then
			res_pos[k] = v
		else
			res_neg[k] = v
		end
	end
	return res_pos, res_neg
end

function aoc.keys(tab)
	local res = {}
	for k, _ in pairs(tab) do
		table.insert(res, k)
	end
	return res
end

function aoc.invert(tab)
	local res = {}
	for k, v in pairs(tab) do
		res[v] = k
	end
	return res
end

function aoc.copy_shallow(tab)
	local res = {}
	for k, v in pairs(tab) do
		res[k] = v
	end
	return res
end

function aoc.merge_into(left, right)
	for k, v in pairs(right) do
		left[k] = v
	end
end

-- Arrays

function aoc.splice(tab, del_start, del_end)
	local res = {}
	for i, v in ipairs(tab) do
		if i < del_start or i >= del_end then
			table.insert(res, v)
		end
	end
	return res
end

function aoc.fold(f, tab)
	if #tab == 0 then
		return nil
	end
	local acc = tab[1]

	local current_index = next(tab, 1)
	while current_index ~= nil do
		acc = f(acc, tab[current_index])
		current_index = next(tab, current_index)
	end
	return acc
end
function aoc.min(tab)
	return aoc.fold(
		function(acc, curr) return math.min(acc, curr) end,
		tab
	)
end
function aoc.max(tab)
	return aoc.fold(
		function(acc, curr) return math.max(acc, curr) end,
		tab
	)
end

-- Vectors and matrices

struct aoc.Vector2 {
	x: double
	y: double
}
terra aoc.Vector2:add(rhs: aoc.Vector2)
	return aoc.Vector2 { x = self.x + rhs.x, y = self.y + rhs.y }
end
terra aoc.Vector2:sub(rhs: aoc.Vector2)
	return aoc.Vector2 { x = self.x - rhs.x, y = self.y - rhs.y }
end
terra aoc.Vector2:mul(a: double)
	return aoc.Vector2 { x = self.x * a, y = self.y * a }
end
terra aoc.Vector2:neg()
	return self:mul(-1)
end
terra aoc.Vector2:eq(rhs: aoc.Vector2)
	return self.x == rhs.x and self.y == rhs.y
end
terra aoc.Vector2:rot_90()
	return aoc.Vector2 { x = -self.y, y = self.x }
end
aoc.Vector2.new = function(x, y)
	local v = terralib.new(aoc.Vector2)
	v.x = x
	v.y = y
	return v
end
aoc.Vector2.from_array = function(arr)
	return aoc.Vector2.new(arr[1], arr[2])
end
aoc.Vector2.dump = function(self)
	return self.x .. "," .. self.y
end

-- Meta types

function aoc.Enum(...)
	local t = { type = int }
	for i,name in ipairs({...}) do
		t[name] = i
	end
	return t
end

--[[
function aoc.Matrix(T)
	struct Matrix {
		width: uint
		height: uint
		cells: &T
	}
	terra Matrix:init(width: uint, height: uint)
		self.width = width
		self.height = height
		self.cells = [&T](C.malloc(sizeof(T) * width * height))
	end
	terra Matrix:index(x: uint, y: uint)
		val index = (y - 1) * width + (x - 1)
		if index > self.width * self.height then
			error()
		end
		return index
	end
	terra Matrix:get(x: uint, y: uint)
		return self.cells[self:index(x, y)]
	end
	terra Matrix:set(x: uint, y: uint, value: T)
		self.cells[self:index(x, y)] = value
	end
	Matrix.new = function(width, height)
		local m = terralib.new(Matrix)
		m:init(width, height)
		return m
	end

	return Matrix
end
]]--

aoc.Matrix = {}
local aoc_Matrix_prototype = {
	index = function(self, x, y)
		if x < 1 or x > self.width then
			return nil
		end
		if y < 1 or y > self.height then
			return nil
		end
		
		return (y - 1) * self.width + (x - 1) + 1
	end,
	get = function(self, x, y)
		return self.cells[self:index(x, y)]
	end,
	set = function(self, x, y, v)
		self.cells[self:index(x, y)] = v
	end,
	shallow_copy = function(self)
		local new_m = aoc.Matrix.new(self.width, self.height)
		for y = 1, self.height do
			for x = 1, self.width do
				new_m:set(x, y, self:get(x, y))
			end
		end
		return new_m
	end,
	dump = function(self)
		local res = ""
		for y = 1, self.height do
			for x = 1, self.width do
				local c = self:get(x, y)
				if c == nil then
					res = res .. "_"
				else
					res = res .. aoc.dump(c)
				end
			end
			res = res .. "\n"
		end
		return res
	end
}
aoc.Matrix.new = function(width, height)
	local m = { width=width, height=height, cells={} }
	for k, v in pairs(aoc_Matrix_prototype) do
		m[k] = v
	end
	return m
end

-- Debug

function aoc.dump(val, max_depth)
	if max_depth == nil then
		max_depth = 10
	elseif max_depth <= 0 then
		return "<recursion limit reached>"
	end
	
	local res = ""
	if type(val) == "table" then
		if type(val["dump"]) == "function" then
			res = val:dump()
		else
			res = res .. "{"
			for k, v in pairs(val) do
				res = res .. " " .. k .. "=" .. aoc.dump(v, max_depth - 1) .. ","
			end
			res = res .. " }"
		end
	elseif type(val) == "number" then
		res = res .. tostring(val)
	elseif type(val) == "string" then
		res = res .. string.format("%q", val)
	elseif type(val) == "boolean" then
		res = res .. (res and "true" or "false")
	else
		res = res .. "<" .. type(val) .. ">"
	end
	return res
end
