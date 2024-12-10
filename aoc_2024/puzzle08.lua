require("aoc_2024/libaoc")
local Vector2 = aoc.Vector2

local input_nodes = {}

local input_lines = aoc.read_lines(arg[1])
local width = #input_lines[1]
local height = #input_lines
for y, line in ipairs(input_lines) do
	for x = 1, #line do
		local ch = line:sub(x, x)
		if ch ~= "." then
			if input_nodes[ch] == nil then
				input_nodes[ch] = {}
			end
			table.insert(input_nodes[ch], Vector2.new(x - 1, y - 1))
		end
	end
end

local function within_bounds(vec)
	return vec.x >= 0 and vec.x < width and vec.y >= 0 and vec.y < height
end

local function antinode(a, b, n)
	local diff2 = a:sub(b):mul(n)
	return b:add(diff2)
end

-- Part 1
local antinodes_count = 0
local antinodes = {}
for name, nodes in pairs(input_nodes) do
	for i = 1, #nodes do
		for j = i + 1, #nodes do
			local anti1 = antinode(nodes[i], nodes[j], 2.0)
			if within_bounds(anti1) then
				antinodes[aoc.dump(anti1)] = true
			end

			local anti2 = antinode(nodes[j], nodes[i], 2.0)
			if within_bounds(anti2) then
				antinodes[aoc.dump(anti2)] = true
			end
		end
	end
end
print("antinodes_count", #aoc.keys(antinodes))

-- Part 2
local antinodes2 = {}
for name, nodes in pairs(input_nodes) do
	for i = 1, #nodes do
		for j = i + 1, #nodes do
			local f = 1.0
			while true do
				local anti1 = antinode(nodes[i], nodes[j], f)
				if within_bounds(anti1) then
					antinodes2[aoc.dump(anti1)] = true
				else
					break
				end
				f = f + 1.0
			end

			local f = 1.0
			while true do
				local anti2 = antinode(nodes[j], nodes[i], f)
				if within_bounds(anti2) then
					antinodes2[aoc.dump(anti2)] = true
				else
					break
				end
				f = f + 1.0
			end
		end
	end
end
print("antinodes_count2", #aoc.keys(antinodes2))