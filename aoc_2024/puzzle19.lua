require("aoc_2024/libaoc")
local input_lines = aoc.read_lines(arg[1])

local towel_kinds = aoc.string_split(", ", input_lines[1])
local wanted_patterns = {}
for i = 3, #input_lines do
	table.insert(wanted_patterns, input_lines[i])
end

table.sort(towel_kinds, function(a, b) return #a > #b end)
-- print(aoc.dump(towel_kinds))
-- print(aoc.dump(wanted_patterns))

local function can_construct_count(towel_kinds, cache, wanted_pattern)
	if cache[wanted_pattern] ~= nil then
		return cache[wanted_pattern]
	end

	local count = 0
	for _, towel in pairs(towel_kinds) do
		if towel == wanted_pattern then
			count = count + 1
		elseif #towel < #wanted_pattern then
			local sub_pattern = aoc.string_strip_prefix(towel, wanted_pattern)
			if sub_pattern ~= nil then
				local sub_count = can_construct_count(towel_kinds, cache, sub_pattern)
				cache[wanted_pattern] = sub_count
				count = count + sub_count
			end
		end
	end

	cache[wanted_pattern] = count
	return count
end

-- Part 1 & Part 2
local construct_cache = {}
local possible_patterns = 0
local possible_variants = 0ULL
for i, wanted_pattern in ipairs(wanted_patterns) do
	local count = can_construct_count(towel_kinds, construct_cache, wanted_pattern)
	if count > 0 then
		possible_patterns = possible_patterns + 1
		possible_variants = possible_variants + count
	end
end
print("possible_patterns", possible_patterns)
print("possible_variants", possible_variants)
