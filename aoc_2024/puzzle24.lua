require("aoc_2024/libaoc")

local Op = aoc.Enum("AND", "OR", "XOR")
local WireLevel = aoc.Enum("OFF", "ON", "UNDEF")

local input_lines = aoc.iter(aoc.read_lines(arg[1]))

local wires = {}
local gates = {}

for line in input_lines do
	if line == "" then
		break
	end
	local name, value_str = unpack(aoc.string_split(": ", line))

	local value = WireLevel.OFF
	if value_str == "1" then
		value = WireLevel.ON
	end

	aoc.map_insert(wires, name, value)
end
for line in input_lines do
	local gate, output = unpack(aoc.string_split(" -> ", line))
	local left, op_str, right = unpack(aoc.string_split(" ", gate))

	local op = Op.try_parse(op_str)
	if op == nil then
		error("Invalid op \"" .. op_str .. "\"")
	end

	aoc.map_insert(wires, output, WireLevel.UNDEF)
	table.insert(gates, { op = op, left = left, right = right, output = output })
end

--[[
L ← Empty list that will contain the sorted elements
S ← Set of all nodes with no incoming edge

while S is not empty do
    remove a node n from S
    add n to L
    for each node m with an edge e from n to m do
        remove edge e from the graph
        if m has no other incoming edges then
            insert m into S

if graph has edges then
    return error   (graph has at least one cycle)
else 
    return L   (a topologically sorted order)
]]--
local function sort_wires(wires, gates)
	local wire_dependencies = {}
	for gate in aoc.iter(gates) do
		aoc.map_insert(wire_dependencies, gate.output, { gate.left, gate.right })
	end
	
	local heads = {}
	for name, value in pairs(wires) do
		if value ~= WireLevel.UNDEF then
			aoc.set_insert(heads, name)
		end
	end

	local sorted = {}
	while aoc.set_len(heads) > 0 do
		local wire = aoc.set_pop(heads)
		table.insert(sorted, wire)
		-- linear search go brr
		for other, deps in pairs(wire_dependencies) do
			local dep_index = aoc.index_of(deps, wire)
			if dep_index ~= nil then
				table.remove(deps, dep_index)
				if #deps == 0 then
					aoc.set_insert(heads, other)
				end
			end
		end
	end

	return sorted
end
local function sort_gates(wires, gates)
	local sorted_wires = aoc.invert(sort_wires(wires, gates))
	local sorted_gates = aoc.copy_shallow(gates)
	table.sort(sorted_gates, function(a, b)
		return sorted_wires[a.output] < sorted_wires[b.output]
	end)

	return sorted_gates
end

gates = sort_gates(wires, gates)
-- print(aoc.dump(wires))
-- print(aoc.dump(gates))

local function prefixed_wire_names(wires, prefix)
	local res = {}
	for name, value in pairs(wires) do
		if aoc.string_strip_prefix(prefix, name) ~= nil then
			table.insert(res, name)
		end
	end
	table.sort(res)

	return res
end
local function map_wire_level(level)
	if level == WireLevel.OFF then
		return 0ULL
	elseif level == WireLevel.ON then
		return 1ULL
	else
		return 0xEEEEEEEEEEEEEEEEULL
	end
end
local terra fold_bit(acc: uint64, curr: uint64, shift: uint32)
	return acc or (curr << shift)
end
local function extract_wires_bits(wires, names)
	local res = 0ULL
	for i, wire in ipairs(names) do
		res = fold_bit(res, map_wire_level(aoc.map_get(wires, wire)), i - 1)
	end

	return res
end

local function make_simulator(wires, gates)
	local function generate_op(ctx, gate)
		if gate.op == Op.AND then
			return quote
				[ aoc.map_get(ctx.wires, gate.output) ] = [ aoc.map_get(ctx.wires, gate.left) ] and [ aoc.map_get(ctx.wires, gate.right) ]
			end
		elseif gate.op == Op.OR then
			return quote
				[ aoc.map_get(ctx.wires, gate.output) ] = [ aoc.map_get(ctx.wires, gate.left) ] or [ aoc.map_get(ctx.wires, gate.right) ]
			end
		elseif gate.op == Op.XOR then
			return quote
				[ aoc.map_get(ctx.wires, gate.output) ] = [ aoc.map_get(ctx.wires, gate.left) ] ^ [ aoc.map_get(ctx.wires, gate.right) ]
			end
		end
	end
	
	local ctx = {
		wires = {}
	}
	for name, value in pairs(wires) do
		aoc.map_insert(ctx.wires, name, symbol(uint64))
	end
	local wires_x = prefixed_wire_names(wires, "x")
	local wires_y = prefixed_wire_names(wires, "y")
	local wires_z = prefixed_wire_names(wires, "z")
	
	local terra simulate(x: uint64, y: uint64)
		escape
			for name, sym in pairs(ctx.wires) do
				emit quote
					var [ sym ] = [ map_wire_level(aoc.map_get(wires, name)) ]
				end
			end
		end

		escape
			for i, wire in ipairs(wires_x) do
				emit quote
					[ aoc.map_get(ctx.wires, wire) ] = ([ x ] >> [ i - 1 ]) and 1
				end
			end
		end
		escape
			for i, wire in ipairs(wires_y) do
				emit quote
					[ aoc.map_get(ctx.wires, wire) ] = ([ y ] >> [ i - 1 ]) and 1
				end
			end
		end

		escape
			for gate in aoc.iter(gates) do
				emit quote [ generate_op(ctx, gate) ] end
			end
		end
		var output: uint64 = 0
		escape
			for i, wire in ipairs(wires_z) do
				emit quote
					[ output ] = [ output ] or [ aoc.map_get(ctx.wires, wire) ] << [ i - 1 ]
				end
			end
		end
		return output
	end
	-- print(simulate:printpretty())
	-- print(simulate:disas())
	return simulate
end
local simulator = make_simulator(wires, gates)

local wires_x = prefixed_wire_names(wires, "x")
local input_x = extract_wires_bits(wires, wires_x)
local wires_y = prefixed_wire_names(wires, "y")
local input_y = extract_wires_bits(wires, wires_y)

-- Part 1
local result = simulator(input_x, input_y)
print("result", result)

-- Part 2
local wires_z = prefixed_wire_names(wires, "z")

local function find_gate(gates, output)
	for i, gate in ipairs(gates) do
		if gate.output == output then
			return gate, i
		end
	end
end
local function swap_gates(wires, gates, a, b)
	local gates_copy = aoc.copy_shallow(gates)
	local gate_a = find_gate(gates_copy, a)
	local gate_b = find_gate(gates_copy, b)

	local tmp = gate_a.output
	gate_a.output = gate_b.output
	gate_b.output = tmp

	sort_gates(wires, gates_copy)
	return gates_copy
end

local function dump_gate(wires, gates, gate, depth)
	local dump_left = nil
	if aoc.map_get(wires, gate.left) ~= WireLevel.UNDEF or depth == 0 then
		dump_left = gate.left
	else
		dump_left = "[" .. gate.left .. "]" .. "(" .. dump_gate(wires, gates, find_gate(gates, gate.left), depth - 1) .. ")"
	end

	local dump_right = nil
	if aoc.map_get(wires, gate.right) ~= WireLevel.UNDEF or depth == 0 then
		dump_right = gate.right
	else
		dump_right = "[" .. gate.right .. "]" .. "(" .. dump_gate(wires, gates, find_gate(gates, gate.right), depth - 1) .. ")"
	end

	return dump_left .. " " .. Op.dump(gate.op) .. " " .. dump_right
end
local function print_gates(wires, gates, wires_z)
	for wire in aoc.iter(wires_z) do
		local g = find_gate(gates, wire)
		print(wire .. " = " .. dump_gate(wires, gates, g, 100))
	end
end

local function dump_gate_graphviz(gates)
	local dot = "digraph G {\n"
	for gate in aoc.iter(gates) do
		local color = "black"
		if gate.op == Op.AND then
			color = "red"
		elseif gate.op == Op.OR then
			color = "green"
		elseif gate.op == Op.XOR then
			color = "blue"
		end

		dot = dot .. gate.left .. "->" .. gate.output .. "[color=" .. color .. "]" .. ";\n"
		dot = dot .. gate.right .. "->" ..  gate.output .. "[color=" .. color .. "]" .. ";\n"
	end
	dot = dot .. "}"
	print(dot)
end


if #wires_x ~= #wires_y then
	error("assertion failed - inputs are the same bit length")
end
local bit_length = #wires_x

local terra compute_expected(x: uint64, y: uint64)
	var mask_x: uint64 = (1ULL << [ #wires_x ]) - 1
	var mask_y: uint64 = (1ULL << [ #wires_y ]) - 1
	var mask_z: uint64 = (1ULL << [ #wires_z ]) - 1

	return ((x and mask_x) + (y and mask_y)) and mask_z
end
local function find_wrong_bits(wires, gates, input_bits, simulator)
	local wrong_bits = {}
	for x = 1, input_bits do
		local test_x = fold_bit(0, 1, x - 1)
		local test_y = 0
		local res = simulator(test_x, test_y)
		local expected = compute_expected(test_x, test_y)

		if res ~= expected then
			aoc.set_insert(wrong_bits, x)
		end
	end
	return wrong_bits
end

-- solved manually
local swaps = {
	{ "z05", "jst" },
	{ "gdf", "mcm" },
	{ "z15", "dnt" },
	{ "z30", "gwc" }
}
for swap in aoc.iter(swaps) do
	gates = swap_gates(wires, gates, swap[1], swap[2])
end

print_gates(wires, gates, wires_z)
dump_gate_graphviz(gates)
print(aoc.dump(
	find_wrong_bits(wires, gates, bit_length, make_simulator(wires, gates))
))

local answer = aoc.flatten(swaps)
table.sort(answer)
print(aoc.join(",", answer))
