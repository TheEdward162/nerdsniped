require("aoc_2024/libaoc")

local dependency_rules = {}
local updates = {}

local input_file = io.open(arg[1], "r")
while true do
	line = input_file:read("l")
	if line == nil then
		break
	end
	if line == "" then
		break
	end

	local dep = aoc.string_split("|", line)
	table.insert(dependency_rules, dep)
end
while true do
	line = input_file:read("l")
	if line == nil then
		break
	end
	table.insert(
		updates,
		aoc.string_split(",", line)
	)
end
input_file:close()


-- Part 1
local depends_on = {} -- map[page, list[page]], describes what pages a given page depends on
for _, dep in pairs(dependency_rules) do
	if depends_on[dep[2]] == nil then
		depends_on[dep[2]] = {}
	end

	table.insert(depends_on[dep[2]], dep[1])
end
depends_on = aoc.map(aoc.invert, depends_on)

local function is_valid_update(update)
	local cant_appear = {}

	for _, page in pairs(update) do
		if cant_appear[page] ~= nil then
			return false
		end

		local page_deps = depends_on[page]
		if page_deps ~= nil then
			aoc.merge_into(cant_appear, page_deps)
		end
	end

	return true
end
local function middle_page(update)
	local middle_index = math.floor(#update / 2) + 1
	return update[middle_index]
end

-- Part 2
local function update_sort_fn(a, b)
	-- must return true if the first argument should come first in the sorted array
	local page_deps = depends_on[b]
	if page_deps ~= nil and page_deps[a] ~= nil then
		return true
	end
	return false
end
local function fix_update(update)
	local fixed_update = aoc.copy_shallow(update)
	table.sort(fixed_update, update_sort_fn)
	return fixed_update
end

local middle_sum = 0
local fixed_middle_sum = 0
for _, update in pairs(updates) do
	if is_valid_update(update) then
		middle_sum = middle_sum + middle_page(update)
	else
		local fixed_update = fix_update(update)
		fixed_middle_sum = fixed_middle_sum + middle_page(fixed_update)
	end
end
print("middle_sum", middle_sum)
print("fixed_middle_sum", fixed_middle_sum)