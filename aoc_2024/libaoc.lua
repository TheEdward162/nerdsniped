aoc = {}

function aoc.dump(val, max_depth)
	if max_depth == nil then
		max_depth = 10
	elseif max_depth <= 0 then
		return "<recursion limit reached>"
	end
	
	local res = ""
	if type(val) == "table" then
		res = res .. "{"
		for k, v in pairs(val) do
			res = res .. " " .. k .. "=" .. aoc.dump(v, max_depth - 1) .. ","
		end
		res = res .. " }"
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

function aoc.string_split_space(str)
	local res = {}
	for sub_str in string.gmatch(str, "([^ ]+)") do
		table.insert(res, sub_str)
	end
	return res
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
