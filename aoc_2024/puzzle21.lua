require("aoc_2024/libaoc")
local Vector2 = aoc.Vector2(int32)

local function string_split_chars(str)
	local res = {}
	for i = 1, #str do
		table.insert(res, str:sub(i, i))
	end
	return res
end

local input_lines = aoc.read_lines(arg[1])
local codes = {}
for _, line in pairs(input_lines) do
	table.insert(codes, {
		code = string_split_chars(line),
		numeric = tonumber(line:sub(1, #line - 1))
	})
end
-- print(aoc.dump(codes))

local KEYPAD_NUMERIC = {
	["7"] = Vector2.new(1, 1),
	["8"] = Vector2.new(2, 1),
	["9"] = Vector2.new(3, 1),
	["4"] = Vector2.new(1, 2),
	["5"] = Vector2.new(2, 2),
	["6"] = Vector2.new(3, 2),
	["1"] = Vector2.new(1, 3),
	["2"] = Vector2.new(2, 3),
	["3"] = Vector2.new(3, 3),
	["0"] = Vector2.new(2, 4),
	["A"] = Vector2.new(3, 4),
	_gap = Vector2.new(1, 4)
}
local KEYPAD_DIRECTIONAL = {
	["^"] = Vector2.new(2, 1),
	["A"] = Vector2.new(3, 1),
	["<"] = Vector2.new(1, 2),
	["v"] = Vector2.new(2, 2),
	[">"] = Vector2.new(3, 2),
	_gap = Vector2.new(1, 1)
}

local function print_code(type_code)
	local res = ""
	for _, char in pairs(type_code) do
		res = res .. char
	end
	print(res)
end

local function type_keypad(keypad, start_char, code_iter)
	return coroutine.wrap(function()
		local current_pos = keypad[start_char]
		for end_char in code_iter do
			local next_pos = keypad[end_char]
			local to_next = next_pos:sub(current_pos)

			local x_char = ">"
			if to_next.x < 0 then
				x_char = "<"
			end
			local y_char = "v"
			if to_next.y < 0 then
				y_char = "^"
			end

			local x_first = (current_pos.x == keypad._gap.x and next_pos.y == keypad._gap.y)
				or (to_next.x < 0 and not (current_pos.y == keypad._gap.y and next_pos.x == keypad._gap.x))

			if x_first then
				for i = 1, math.abs(to_next.x) do
					coroutine.yield(x_char)
				end
				for i = 1, math.abs(to_next.y) do
					coroutine.yield(y_char)
				end
			else
				for i = 1, math.abs(to_next.y) do
					coroutine.yield(y_char)
				end
				for i = 1, math.abs(to_next.x) do
					coroutine.yield(x_char)
				end
			end
			coroutine.yield("A")

			current_pos = next_pos
		end
	end)
end

local COMPLEXITY_RECURSIVE_CACHE = {}
local function complexity_recursive(code_iter, count)
	if count == 0 then
		return aoc.iter_count(code_iter)
	end

	local result = 0ULL

	local last_char = "A"
	for char in code_iter do
		local cache_key = last_char .. char .. (count - 1 + 100)
		local cache_value = aoc.map_get(COMPLEXITY_RECURSIVE_CACHE, cache_key)
		if cache_value ~= nil then
			result = result + cache_value
		else
			local c = type_keypad(KEYPAD_DIRECTIONAL, last_char, aoc.iter({ char }))
			local sub_res = complexity_recursive(c, count - 1)
			aoc.map_insert(COMPLEXITY_RECURSIVE_CACHE, cache_key, sub_res)

			result = result + sub_res
		end
		last_char = char
	end

	return result
end

-- Part 1
local complexities = 0
for _, c in pairs(codes) do
	local start_code = type_keypad(KEYPAD_NUMERIC, "A", aoc.iter(c.code))
	complexities = complexities + complexity_recursive(start_code, 2) * c.numeric
end
print("complexities", complexities)

-- Part 2
local complexities2 = 0
for _, c in pairs(codes) do
	local start_code = type_keypad(KEYPAD_NUMERIC, "A", aoc.iter(c.code))
	complexities2 = complexities2 + complexity_recursive(start_code, 25) * c.numeric
end
print("complexities2", complexities2)
