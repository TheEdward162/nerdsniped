function aoc_dump(val)
	local res = ""
	if type(val) == "table" then
		res = res .. "{"
		for k, v in pairs(val) do
			res = res .. " " .. k .. "=" .. aoc_dump(v) .. ","
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

function aoc_string_split_space(str)
	local res = {}
	for sub_str in string.gmatch(str, "([^ ]+)") do
		table.insert(res, sub_str)
	end
	return res
end

function aoc_map(f, tab)
	local res = {}
	for k, v in pairs(tab) do
		res[k] = f(v)
	end
	return res
end

function aoc_splice(tab, del_start, del_end)
	local res = {}
	for i, v in ipairs(tab) do
		if i < del_start or i >= del_end then
			table.insert(res, v)
		end
	end
	return res
end
