require("aoc_2024/libaoc")

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

local Operation = aoc.Enum("PLUS", "MULTIPLY", "CONCAT")

local input_lines = aoc.read_lines(arg[1])
local input_equations = {}
for _, line in pairs(input_lines) do
	local split = aoc.string_split_space(line)
	local result = str_to_uint64(split[1]:sub(1, -2))
	split[1] = nil
	local parameters = aoc.map(str_to_uint64, aoc.values(split))
	table.insert(input_equations, { result = result, parameters = parameters })
end

local function _make_evaluator(ops_len, params_len)	
	local function make_leaf(expected_result, previous_result)
		return quote
			-- C.printf("Leaf %llu vs %llu\n", [ expected_result ], [ previous_result ])
			if [ expected_result ] == [ previous_result ] then
				return [ expected_result ]
			end
		end
	end
	local function make_intermediate(param, expected_result, previous_result, next_fn)
		return quote
			var actual_result: uint64
			for i = 1, [ ops_len + 1 ] do
				switch i do
					case [ Operation.PLUS ] then
						actual_result = [ previous_result ] + [ param ]
					end
					case [ Operation.MULTIPLY ] then
						actual_result = [ previous_result ] * [ param ]
					end
					case [ Operation.CONCAT ] then
						var actual_result_concat = [ previous_result ]
						for i = 0, uint64_log10([ param ]) do
							actual_result_concat = actual_result_concat * 10ULL
						end
						actual_result = actual_result_concat + [ param ]
					end
				end
				[ next_fn(expected_result, actual_result) ]
			end
		end
	end

	local expected_result = symbol(uint64)
	local params = {}
	for i = 1, params_len do
		table.insert(params, symbol(uint64))
	end

	local eval_fn = make_leaf
	for i = 0, #params - 2 do
		local next_fn = eval_fn
		eval_fn = function(expected_result, previous_result)
			return make_intermediate(params[#params - i], expected_result, previous_result, next_fn)
		end
	end

	local terra evaluate([ expected_result ], [ params ])
		[ eval_fn(expected_result, params[1]) ]

		return 0
	end
	-- print(evaluate:printpretty())
	return evaluate
end
local make_evaluator = terralib.memoize(_make_evaluator)

-- Part 1
local plausible_sum = 0ULL
for i, eq in ipairs(input_equations) do
	local evaluate = make_evaluator(2, #eq.parameters)
	plausible_sum = plausible_sum + evaluate(eq.result, unpack(eq.parameters))
end
print("plausible_sum", plausible_sum)

-- Part 2
local plausible_sum2 = 0ULL
for i, eq in ipairs(input_equations) do
	-- print(i .. "/" .. #input_equations, plausible_sum2, #eq.parameters)
	local evaluate = make_evaluator(3, #eq.parameters)
	plausible_sum2 = plausible_sum2 + evaluate(eq.result, unpack(eq.parameters))
end
print("plausible_sum2", plausible_sum2)
