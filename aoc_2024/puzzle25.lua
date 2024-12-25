require("aoc_2024/libaoc")

local function repeat_n(item, n)
	local res = {}
	for i = 1, n do
		table.insert(res, item)
	end
	return res
end

local lock_height = 7
local lock_width = 5
local lock_first_line = aoc.join("", repeat_n(".", lock_width))

local input_lines = aoc.read_lines(arg[1])

local keys = {}
local locks = {}
for i = 1, #input_lines, lock_height + 1 do
	local is_lock = input_lines[i] ==  lock_first_line

	local fold_fn = math.max
	local init_val = 0
	if is_lock then
		fold_fn = function(acc, curr) return math.max(acc, lock_height - curr + 1) end
	end

	local columns = repeat_n(0, lock_width)
	for y = 1, lock_height do
		local line = input_lines[i + y - 1]
		for x = 1, lock_width do
			local char = line:sub(x, x)
			if char == "#" then
				columns[x] = fold_fn(columns[x], y)
			end
		end
	end
	if is_lock then
		table.insert(locks, columns)
	else
		table.insert(keys, columns)
	end
end

-- print(aoc.dump(keys))
-- print(aoc.dump(locks))

-- Part 1
local fit_combinations = 0
for key in aoc.iter(keys) do
	for lock in aoc.iter(locks) do
		local columns = aoc.map(aoc.sum, aoc.zip(key, lock))
		if aoc.all(function(c) return c <= lock_height end, columns) then
			fit_combinations = fit_combinations + 1
		end
	end
end
print("fit_combinations", fit_combinations)