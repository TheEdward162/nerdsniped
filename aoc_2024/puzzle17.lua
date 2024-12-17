require("aoc_2024/libaoc")
local ffi = require("ffi")
local C = terralib.includecstring [[
    #include<stdlib.h>
	#include<stdio.h>
]]

--[[
registers:
* A
* B
* C
* IP

IO:
* output

operands:
* literal 0-7
* combo 0-3, regA, regB, regC, invalid

instructions:
0 = adv: A = int(A div 2**combo_op)
1 = bxl: B = B ^ literal_op
2 = bst: B = combo_op % 8
3 = jnz: if A ~= 0 { IP = literal_op } else { noop }
4 = bxc: B = B ^ C -- (still reads operand)
5 = out: output(combo_op % 8)
6 = bdv: B = int(A div 2**combo_op)
7 = cdv: C = int(A div 2**combo_op)
]]--

local input_lines = aoc.read_lines(arg[1])
local input_registers = {
	A = aoc.tonumber_u64(aoc.string_strip_prefix("Register A:", input_lines[1])),
	B = aoc.tonumber_u64(aoc.string_strip_prefix("Register B:", input_lines[2])),
	C = aoc.tonumber_u64(aoc.string_strip_prefix("Register C:", input_lines[3]))
}
local input_program_str = aoc.string_strip_prefix("Program: ", input_lines[5])
local input_program = aoc.map(tonumber, aoc.string_split(",", input_program_str))

-- print(aoc.dump(input_registers))
-- print(aoc.dump(input_program))

local function make_evaluator(program)
	local struct EvalOutput {
		output: &int8
		output_len: uint32
		output_cap: uint32
	}
	terra EvalOutput:reserve(additional: uint32)
		if self.output_cap - self.output_len < additional then
			self.output_cap = self.output_cap + additional
			self.output = [rawstring](C.realloc(self.output, self.output_cap))
		end
	end
	-- terra EvalOutput:push_str(str: rawstring, len: uint32)
	-- 	self:reserve(len)

	-- 	for i = 0, len do
	-- 		self.output[self.output_len + i] = str[i]
	-- 	end
	-- 	self.output_len = self.output_len + len
	-- end
	terra EvalOutput:append_output(v: int8)
		self:reserve(1)
		self.output[self.output_len] = v
		self.output_len = self.output_len + 1
	end
	terra EvalOutput:drop()
		C.free(self.output)
	end

	local terra ipow2(exp: int64)
		return 1 << exp
	end

	local function combo_operand(ctx, operand)
		if operand == 7 then
			error("Invalid combo operand 7")
		elseif operand == 6 then
			return ctx.syms.C
		elseif operand == 5 then
			return ctx.syms.B
		elseif operand == 4 then
			return ctx.syms.A
		else
			return operand
		end
	end
	local opcode_gens = {
		-- 0 = adv: A = int(A div 2**combo_op)
		[0] = function(ctx, operand)
			return quote
				[ ctx.syms.A ] = [ ctx.syms.A ] / ipow2([ combo_operand(ctx, operand) ])
			end
		end,
		-- 1 = bxl: B = B ^ literal_op
		[1] = function(ctx, operand)
			return quote
				[ ctx.syms.B ] = [ ctx.syms.B ] ^ [ operand ]
			end
		end,
		-- 2 = bst: B = combo_op % 8
		[2] = function(ctx, operand)
			return quote
				[ ctx.syms.B ] = [ combo_operand(ctx, operand) ] % 8
			end
		end,
		-- 3 = jnz: if A ~= 0 { IP = literal_op } else { noop }
		[3] = function(ctx, operand)
			return quote
				if [ ctx.syms.A ] ~= 0 then
					goto [ ctx.labels[operand + 1] ]
				end
			end
		end,
		-- 4 = bxc: B = B ^ C -- (still reads operand)
		[4] = function(ctx, operand)
			return quote
				[ ctx.syms.B ] = [ ctx.syms.B ] ^ [ ctx.syms.C ]
			end
		end,
		-- 5 = out: output(combo_op % 8)
		[5] = function(ctx, operand)
			return quote
				var value = [ combo_operand(ctx, operand) ] % 8
				[ ctx.syms.output ]:append_output(value)
			end
		end,
		-- 6 = bdv: B = int(A div 2**combo_op)
		[6] = function(ctx, operand)
			return quote
				[ ctx.syms.B ] = [ ctx.syms.A ] / ipow2([ combo_operand(ctx, operand) ])
			end
		end,
		-- 7 = cdv: C = int(A div 2**combo_op)
		[7] = function(ctx, operand)
			return quote
				[ ctx.syms.C ] = [ ctx.syms.A ] / ipow2([ combo_operand(ctx, operand) ])
			end
		end,
	}
	local function emit_instruction(ctx, index)
		local op_label = ctx.labels[index]
		local opcode = ctx.program[(index - 1) * 2 + 1]
		local operand = ctx.program[(index - 1) * 2 + 2]

		return quote
			::[op_label]::
			[ opcode_gens[opcode](ctx, operand) ]
		end
	end

	local ctx = {
		program = program,
		labels = {},
		syms = {
			A = symbol(int64),
			B = symbol(int64),
			C = symbol(int64),
			output = symbol(EvalOutput)
		},
	}
	for i = 1, #program / 2 do
		table.insert(ctx.labels, label("op" .. i))
	end
	
	local terra evaluate(regA: int64, regB: int64, regC: int64)
		var [ ctx.syms.A ] = regA
		var [ ctx.syms.B ] = regB
		var [ ctx.syms.C ] = regC

		var [ ctx.syms.output ] = EvalOutput { nil, 0, 0 }

		escape
			for i = 1, #program / 2 do
				emit quote [ emit_instruction(ctx, i) ] end
			end
		end

		return [ ctx.syms.output ]
	end
	-- print(evaluate:disas())

	return function(a, b, c)
		local eval = evaluate(a, b, c)

		local output = {}
		for i = 0, eval.output_len - 1 do
			table.insert(output, eval.output[i])
		end
		eval:drop()

		return output
	end
end

local device = make_evaluator(input_program)

-- Part 1
local part1 = device(input_registers.A, input_registers.B, input_registers.C)
local part1_output = ""
for _, n in pairs(part1) do
	if #part1_output > 0 then
		part1_output = part1_output .. ","
	end
	part1_output = part1_output .. tostring(n)
end
print(part1_output)

-- Part 2

--[[
Our specific program is:
do {
	B = A % 8
	B = B ^ 3
	C = A / 2**B
	B = B ^ 5
	A = A / 8
	B = B ^ C
	output(B % 8)
} while (A > 0)

output = ((((A % 8) ^ 3) ^ 5) ^ (A / 2**((A % 8) ^ 3))) % 8
(
	(
		((A % 8) ^ 0b110)
	) ^ (
		A >> ((A % 8) ^ 0b11)
	)
) % 8

0bJIHGFEDCBA
0b000 feD
0b001 edc -> ed1
0b010 dCB -> d01
0b011 cBa -> 110
0b100 JiH
0b101 Ihg
0b110 HGF
0b111 GFe

Conclusion:
* A gets shorter by 3 bits every iteration
* Output depends on the low 10 bits of A in every iteration
* Thus we can find the top 3 bits of A by emulating with that A and looking at the first output item
* Recursively we can find all bits of A
	* There might be paths that are infeasible
]]--

local function find_a_override(program_index, current_a)
	if program_index == 0 then
		return current_a
	end
	
	local expected_output = input_program[program_index]
	for i = 0, 7 do
		-- a = a << 3 | i
		local next_a = current_a * 8 + i
		local next_output = device(next_a, input_registers.B, input_registers.C)
		if next_output[1] == expected_output then
			local final = find_a_override(program_index - 1, next_a)
			if final ~= nil then
				return final
			end
		end
	end

	return nil
end

local a_override = find_a_override(#input_program, 0LL)
print("a_override", a_override)
