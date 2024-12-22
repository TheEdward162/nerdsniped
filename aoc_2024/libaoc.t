local C = terralib.includecstring [[
    #include<stdlib.h>
	#include<stdio.h>
	#include<math.h>
]]

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

function aoc.string_strip_prefix(prefix, str)
	if str:sub(1, #prefix) == prefix then
		return str:sub(#prefix + 1)
	else
		return nil
	end
end

-- Numbers

terra aoc.tonumber_u64(v: rawstring)
	var res: uint64
	C.sscanf(v, "%llu", &res)
	return res
end
terra aoc.tonumber_i64(v: rawstring)
	var res: int64
	C.sscanf(v, "%lld", &res)
	return res
end
uint64.dump = function(self)
	return tostring(self)
end
int64.dump = function(self)
	return tostring(self)
end
-- stolen from https://doc.rust-lang.org/src/core/num/int_log10.rs.html
terra uint64.log10(val: uint64)
	var log = 0
    if val >= 10000000000ULL then
        val = val / 10000000000ULL
        log = log + 10
    end
    if val >= 100000ULL then
        val = val / 100000ULL
        log = log + 5
    end

    var C1: uint32 = 393206
    var C2: uint32 = 524188
    var C3: uint32 = 916504
    var C4: uint32 = 514288

	var val32: uint32 = val
    var log32 = (
		((val32 + C1) and (val32 + C2)) ^ ((val32 + C3) and (val32 + C4))
	) >> 17
	-- C.printf("val=%llu log=%llu log32=%llu\n", val, log, log32)
	return log + log32
end
-- print(uint64.log10:disas())
-- Loses precision, but log10l does not work with terra
terra uint64.log10d(val: uint64)
	return C.log10(val)
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

function aoc.values(tab)
	local res = {}
	for _, v in pairs(tab) do
		table.insert(res, v)
	end
	return res
end

function aoc.reverse(tab)
	local res = {}
	for i = #tab, 1, -1 do
		table.insert(res, tab[i])
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

function aoc.iter(tab)
	return coroutine.wrap(function()
		for _, e in pairs(tab) do
			coroutine.yield(e)
		end
	end)
end
function aoc.iter_count(iter)
	local res = 0
	for _ in iter do
		res = res + 1
	end
	return res
end

function aoc.windows(tab, size)
	return coroutine.wrap(function()
		for i = size, #tab do
			local window = {}
			for wi = i - size + 1, i do
				table.insert(window, tab[wi])
			end
			coroutine.yield(window, i)
		end
	end)
end

function aoc.collect(iter)
	local res = {}
	for e in iter do
		table.insert(res, e)
	end
	return res
end
function aoc.append(tab, ...)
	local copy = aoc.copy_shallow(tab)
	for _, elem in pairs({...}) do
		table.insert(copy, elem)
	end
	return copy
end

function aoc.index_of(tab, needle, eq_fn)
	if eq_fn == nil then
		eq_fn = function(a, b) return a == b end
	end
	
	for i, e in ipairs(tab) do
		if eq_fn(e, needle) then
			return i

		end
	end
	return nil
end
function aoc.contains(tab, needle, eq_fn)
	return aoc.index_of(tab, needle, eq_fn) ~= nil
end

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
function aoc.sum(tab)
	return aoc.fold(
		function(acc, curr) return acc + curr end,
		tab
	)
end
function aoc.product(tab)
	return aoc.fold(
		function(acc, curr) return acc * curr end,
		tab
	)
end

function aoc.flatten(tab)
	local res = {}
	for _, v in pairs(tab) do
		for _, inner in pairs(v) do
			table.insert(res, inner)
		end
	end
	return res
end

-- Set and Map

function aoc.map_insert(tab, key, val)
	tab[aoc.dump(key)] = val
end
function aoc.map_remove(tab, key)
	tab[aoc.dump(key)] = nil
end
function aoc.map_has(tab, key)
	return tab[aoc.dump(key)] ~= nil
end
function aoc.map_get(tab, key)
	return tab[aoc.dump(key)]
end
function aoc.map_get_default(tab, key, default)
	local k = aoc.dump(key)
	if tab[k] == nil then
		tab[k] = default
	end
	return tab[k]
end
function aoc.map_len(tab)
	return #aoc.keys(tab)
end

function aoc.set_insert(tab, val)
	aoc.map_insert(tab, val, true)
end
function aoc.set_remove(tab, val)
	aoc.map_remove(tab, val)
end
function aoc.set_has(tab, val)
	return aoc.map_has(tab, val)
end
function aoc.set_len(tab)
	return aoc.map_len(tab)
end

-- Vectors and matrices

function aoc.Vector2(T)
	local struct Vector2 {
		x: T
		y: T
	}
	terra Vector2:add(rhs: Vector2)
		return Vector2 { x = self.x + rhs.x, y = self.y + rhs.y }
	end
	terra Vector2:sub(rhs: Vector2)
		return Vector2 { x = self.x - rhs.x, y = self.y - rhs.y }
	end
	terra Vector2:length()
		return C.sqrt(self.x * self.x + self.y * self.y)
	end
	terra Vector2:length_manhattan()
		return C.abs(self.x) + C.abs(self.y)
	end
	terra Vector2:distance(rhs: Vector2)
		var x = self.x - rhs.x
		var y = self.y - rhs.y
		return C.sqrt(x * x + y * y)
	end
	terra Vector2:mul(a: T)
		return Vector2 { x = self.x * a, y = self.y * a }
	end
	terra Vector2:div(a: T)
		return Vector2 { x = self.x / a, y = self.y / a }
	end
	terra Vector2:dot(rhs: Vector2)
		return self.x * rhs.x + self.y * rhs.y
	end
	terra Vector2:neg()
		return self:mul(-1)
	end
	terra Vector2:eq(rhs: Vector2)
		return self.x == rhs.x and self.y == rhs.y
	end
	terra Vector2:rot_90()
		return Vector2 { x = -self.y, y = self.x }
	end
	terra Vector2:rot_180()
		return Vector2 { x = -self.x, y = -self.y }
	end
	terra Vector2:rot_270()
		return Vector2 { x = self.y, y = -self.x }
	end
	Vector2.new = function(x, y)
		local vec = terralib.new(Vector2)
		vec.x = x
		vec.y = y
		return vec
	end
	Vector2.from_array = function(arr)
		return Vector2.new(arr[1], arr[2])
	end
	Vector2.dump = function(self)
		return aoc.dump(self.x) .. "," .. aoc.dump(self.y)
	end

	return Vector2
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
	copy_shallow = function(self)
		local new_m = aoc.Matrix.new(self.width, self.height)
		for y = 1, self.height do
			for x = 1, self.width do
				new_m:set(x, y, self:get(x, y))
			end
		end
		return new_m
	end,
	dump = function(self, cell_dump_fn)
		return aoc.Matrix.dump(self, cell_dump_fn)
	end
}
aoc.Matrix.dump = function(self, cell_dump_fn)
	if cell_dump_fn == nil then
		cell_dump_fn = aoc.dump
	end
	
	local res = ""
	for y = 1, self.height do
		for x = 1, self.width do
			local c = self:get(x, y)
			if c == nil then
				res = res .. "_"
			else
				res = res .. cell_dump_fn(c)
			end
		end
		res = res .. "\n"
	end
	return res
end
aoc.Matrix.new = function(width, height)
	local m = { width=width, height=height, cells={} }
	aoc.merge_into(m, aoc_Matrix_prototype)
	return m
end
aoc.Matrix.iter = function(m)
	return coroutine.wrap(function()
		for y = 1, m.height do
			for x = 1, m.width do
				coroutine.yield(x, y, m:get(x, y))
			end
		end
	end)
end
--[[
	trans_fn - function(head: { pos, state, cost, path }, next_pos): { cost, state } | nil
	end_fn - function(head: { pos, state, cost, path }): bool
]]--
local aoc_find_2d_paths = function(Vector2, start, end_fn, trans_fn)
	local DIRS_2D = { Vector2.new(1, 0), Vector2.new(-1, 0), Vector2.new(0, 1), Vector2.new(0, -1) }
	
	local heads = aoc.PriorityQueue.new(function(a, b)
		return a.cost - b.cost
	end)
	heads:insert({ pos = start.pos, cost = 0, path = { start.pos }, state = start.state })

	while heads:len() > 0 do
		local head = heads:pop()

		if end_fn(head) then
			coroutine.yield({ path = head.path, cost = head.cost })
		else
			for _, dir in pairs(DIRS_2D) do
				local next_pos = head.pos:add(dir)
				local next_trans = trans_fn(head, next_pos)
				if next_trans ~= nil then
					local next_path = aoc.append(head.path, next_pos)
					heads:insert(
						{ pos = next_pos, cost = next_trans.cost, path = next_path, state = next_trans.state }
					)
				end
			end
		end
	end
end
aoc.find_2d_paths = function(map, start, end_fn, trans_fn)
	return coroutine.wrap(function() return aoc_find_2d_paths(map, start, end_fn, trans_fn) end)
end

-- The dumbest priority queue you've seen!
aoc.PriorityQueue = {}
local aoc_PriorityQueue_prototype = {
	insert = function(self, elem)
		table.insert(self.data, elem)
	end,
	pop = function(self)
		if #self.data == 0 then
			return nil
		end

		local min = 1
		for i, elem in ipairs(self.data) do
			if self.cmp_fn(elem, self.data[min]) < 0 then
				min = i
			end
		end

		return table.remove(self.data, min)
	end,
	len = function(self)
		return #self.data
	end,
	dump = function(self)
		return aoc.dump(self.data)
	end
}
aoc.PriorityQueue.new = function(cmp_fn)
	local pq = { cmp_fn = cmp_fn, data = {} }
	aoc.merge_into(pq, aoc_PriorityQueue_prototype)
	return pq
end

-- Meta types

function aoc.Enum(...)
	local t = {}
	local inverse = {}
	local parse = {}
	local parse_inverse = {}

	for i, variant in ipairs({...}) do
		if type(variant) == "table" then
			local name = variant[1]
			local parse_char = variant[2]
			t[name] = i
			inverse[i] = name
			parse[parse_char] = i
			parse_inverse[i] = parse_char
		else
			local name = variant
			t[name] = i
			inverse[i] = name
		end
	end

	-- t["_inverse"] = inverse
	-- t["_parse"] = parse
	-- t["dump"] = function(self)
	-- 	return tostring(inverse[self])
	-- end
	t["try_parse"] = function(char, default)
		local v = parse[char]
		if v == nil then
			v = default
		end
		return v
	end
	t["dump"] = function(self)
		return tostring(parse_inverse[self])
	end

	return t
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
		res = res .. val
	elseif type(val) == "boolean" then
		res = res .. (res and "true" or "false")
	elseif type(val) == "cdata" then
		local ctype = terralib.typeof(val)
		if type(ctype["dump"]) == "function" then
			res = ctype.dump(val)
		else
			res = res .. "<" .. tostring(ctype) .. ">"
		end
	else
		res = res .. "<" .. type(val) .. ">"
	end
	return res
end
