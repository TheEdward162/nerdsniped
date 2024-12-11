require("aoc_2024/libaoc")

local input_lines = aoc.read_lines(arg[1])

local stones = aoc.map(aoc.tonumber_u64, aoc.string_split_space(input_lines[1]))
local function apply_rules(stone)
	local res = {}

	local log10 = uint64.log10(stone)
	if stone == 0 then
		table.insert(res, 1)
	elseif log10 % 2 == 1 then
		local mask = 10^((log10 + 1) / 2)
		table.insert(res, stone / mask)
		table.insert(res, stone % mask)
	else
		-- print(aoc.dump(stone) .. " * 2024 = " .. aoc.dump(stone * 2024ULL))
		table.insert(res, stone * 2024ULL)
	end

	return res
end

local count_stones_lookup = {}
local function count_stones_n(stone, n)
	local cache_key = aoc.dump(stone) .. ":" .. n
	if count_stones_lookup[cache_key] ~= nil then
		return count_stones_lookup[cache_key]
	end

	if n == 1 then
		return #apply_rules(stone)
	end

	local count = 0ULL
	local new_stones = apply_rules(stone)
	for _, ns in pairs(new_stones) do
		count = count + count_stones_n(ns, n - 1)
	end

	count_stones_lookup[cache_key] = count
	return count
end

-- Part 1
local count_stones = aoc.sum(
	aoc.map(
		function(s) return count_stones_n(s, 25) end,
		stones
	)
)
print("count_stones", count_stones)

-- Part 2
local count_stones2 = aoc.sum(
	aoc.map(
		function(s) return count_stones_n(s, 75) end,
		stones
	)
)
print("count_stones2", count_stones2)
