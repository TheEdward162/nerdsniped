require("aoc_2024/libaoc")
ffi = require("ffi")

local C = terralib.includecstring [[
    #include<stdio.h>
]]
terra str_to_uint64(v: rawstring)
	var res: uint64
	C.sscanf(v, "%llu", &res)
	return res
end
terra uint64_log10(v: uint64)
	var res = 0
	while v > 0 do
		res = res +  1
		v = v / 10
	end
	return res
end

local input_lines = aoc.read_lines(arg[1])
local input_equations = {}
for _, line in pairs(input_lines) do
	local split = aoc.string_split_space(line)
	local result = str_to_uint64(split[1]:sub(1, -2))
	split[1] = nil
	local parameters = aoc.map(str_to_uint64, aoc.values(split))
	table.insert(input_equations, { result = result, parameters = parameters })
end

local Operation = aoc.Enum("PLUS", "MULTIPLY", "CONCAT")

function co_gen_ops(symbols, prefix, n)
	if n == 0 then
		coroutine.yield(prefix)
	else
		for _, symbol in pairs(symbols) do
			table.insert(prefix, symbol)
			co_gen_ops(symbols, prefix, n - 1)
			table.remove(prefix)
		end
	end
end
local function gen_ops(symbols, len)
	return coroutine.wrap(function() return co_gen_ops(symbols, {}, len) end)
end

local function evaluate(parameters, operations)	
	local result = parameters[1]
	for i = 2, #parameters do
		local op = operations[i - 1]
		local b = result
		if op == Operation.PLUS then
			result = result + parameters[i]
		elseif op == Operation.MULTIPLY then
			result = result * parameters[i]
		elseif op == Operation.CONCAT then
			for i = 1, uint64_log10(parameters[i]) do
				result = result * 10ULL
			end
			result = result + parameters[i]
		end
		-- print("evaluate", b, op, parameters[i], result)
	end
	return result
end

-- Part 1
local correct_sum = 0ULL
local symbols = { Operation.PLUS, Operation.MULTIPLY }
for i, eq in ipairs(input_equations) do
	for ops in gen_ops(symbols, #eq.parameters - 1) do
		local evaled = evaluate(eq.parameters, ops)
		if evaled == eq.result then
			correct_sum = correct_sum + evaled
			break
		end
	end
end
print("correct_sum", correct_sum)

-- Part 2
local correct_sum2 = 0ULL
local symbols2 = { Operation.PLUS, Operation.MULTIPLY, Operation.CONCAT }
for i, eq in ipairs(input_equations) do
	print(i .. "/" .. #input_equations)
	for ops in gen_ops(symbols2, #eq.parameters - 1) do
		local evaled = evaluate(eq.parameters, ops)
		if evaled == eq.result then
			-- print("evaled", evaled == eq.result, aoc.dump(eq.parameters), aoc.dump(ops), evaled)
			correct_sum2 = correct_sum2 + evaled
			break
		end
	end
end
print("correct_sum2", correct_sum2)
