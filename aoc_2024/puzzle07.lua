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
	local result = split[1]:sub(1, -2)
	split[1] = nil
	local parameters = aoc.values(split)
	table.insert(input_equations, { result = result, parameters = parameters })
end

local function make_evaluator(eq, ops)
	local function make_leaf(expected_result, previous_result)
		return quote
			-- C.printf("Leaf %llu vs %llu\n", [ expected_result ], [ previous_result ])
			if [ expected_result ] == [ previous_result ] then
				return [ expected_result ]
			end
		end
	end
	local function make_intermediate(param, expected_result, previous_result, next_fn)
		local statements = {}
		for _, op in pairs(ops) do
			if op == Operation.PLUS then
				table.insert(statements, quote
					var actual_result_plus = [ previous_result ] + [ param ]
					-- C.printf("Plus %llu + %llu = %llu\n", [ previous_result ], [ param ], actual_result_plus)
					[ next_fn(expected_result, actual_result_plus) ]
				end)
			elseif op == Operation.MULTIPLY then
				table.insert(statements, quote
					var actual_result_mult = [ previous_result ] * [ param ]
					-- C.printf("Mul %llu * %llu = %llu\n", [ previous_result ], [ param ], actual_result_mult)
					[ next_fn(expected_result, actual_result_mult) ]
				end)
			elseif op == Operation.CONCAT then
				table.insert(statements, quote
					var actual_result_concat = [ previous_result ]
					for i = 0, uint64_log10([ param ]) do
						actual_result_concat = actual_result_concat * 10ULL
					end
					actual_result_concat = actual_result_concat + [ param ]
					-- C.printf("Concat %llu concat %llu = %llu\n", [ previous_result ], [ param ], actual_result_concat)
					[ next_fn(expected_result, actual_result_concat) ]
				end)
			end
		end

		return statements
	end

	local expected_result = { ident = symbol(uint64), value = str_to_uint64(eq.result) }
	local params = {}
	for _, param in pairs(eq.parameters) do
		table.insert(params, { ident = symbol(uint64), value = str_to_uint64(param) })
	end

	local eval_fn = make_leaf
	for i = 0, #params - 2 do
		local next_fn = eval_fn
		eval_fn = function(expected_result, previous_result)
			return make_intermediate(params[#params - i].ident, expected_result, previous_result, next_fn)
		end
	end

	local terra evaluate()
		var [ expected_result.ident ] = [ expected_result.value ]
		escape
			for _, param in pairs(params) do
				emit quote
					var [ param.ident ] = [ param.value ]
				end
			end
		end

		[ eval_fn(expected_result.ident, params[1].ident) ]

		return 0
	end
	-- print(evaluate:printpretty())
	return evaluate
end

-- Part 1
local plausible_sum = 0ULL
for i, eq in ipairs(input_equations) do
	local evaluate = make_evaluator(eq, { Operation.PLUS, Operation.MULTIPLY })
	plausible_sum = plausible_sum + evaluate()
end
print("plausible_sum", plausible_sum)

-- Part 2
local plausible_sum2 = 0ULL
for _, eq in pairs(input_equations) do
	local evaluate = make_evaluator(eq, { Operation.PLUS, Operation.MULTIPLY, Operation.CONCAT })
	plausible_sum2 = plausible_sum2 + evaluate()
end
print("plausible_sum2", plausible_sum2)
