require("aoc_2024/libaoc")
local Vector2 = aoc.Vector2(double)
local Vector2i64 = aoc.Vector2(int64)

local function input_parse_vec2(str)
	local x, y = unpack(aoc.string_split(", ", str))
	x = tonumber(x:sub(3))
	y = tonumber(y:sub(3))
	return Vector2.new(x, y)
end

local input_lines = aoc.read_lines(arg[1])

local claw_machines = {}
local line_index = 1
while line_index < #input_lines do
	local button_a = input_parse_vec2(
		aoc.string_strip_prefix("Button A: ", input_lines[line_index])
	)
	local button_b = input_parse_vec2(
		aoc.string_strip_prefix("Button B: ", input_lines[line_index + 1])
	)
	local prize = input_parse_vec2(
		aoc.string_strip_prefix("Prize:", input_lines[line_index + 2])
	)

	table.insert(claw_machines, { a = button_a, b = button_b, p = prize })
	
	line_index = line_index + 4
end

-- minimize [3, 1] * x
-- subject to
--   Ax = b
--   x >= 0
--
-- A = [button_a_x, button_b_x; button_a_y, button_b_y], b = [prize_x, prize_y]
local function solve_exhaustive(machine, c, search_range)
	local A = { Vector2.new(machine.a.x, machine.b.x), Vector2.new(machine.a.y, machine.b.y) }
	local b = machine.p

	local min_solution = nil -- { x = Vector2, value = number }
	for x_pos = search_range[1], search_range[2] do
		for y_pos = search_range[1], search_range[2] do
			local x = Vector2.new(x_pos, y_pos)
			local b_calc = Vector2.new(A[1]:dot(x), A[2]:dot(x))
			if b_calc:eq(b) then
				local value = c:dot(x)
				if min_solution == nil or value < min_solution.value then
					min_solution = { x = x, value = value }
				end
			end
		end
	end

	if min_solution == nil then
		return nil
	else
		return min_solution.x
	end
end

local function extended_gcd(a, b)
	local b0 = b
	local s0 = 1
	local s1 = 0
	local t0 = 0
	local t1 = 1
	
	while b > 0 do
		local q = math.floor(a / b)
		b0 = b
	
		a, b = b, a - q * b
		s0, s1 = s1, s0 - q * s1
		t0, t1 = t1, t0 - q * t1
	end

	return { s = s0, t = t0, gcd = b0 }
end
local function solve_euclid_dim(a, b, p)
	local e = extended_gcd(a, b)
	local m = p / e.gcd
	if math.floor(m) ~= m then
		return nil
	end

	local r = {
		pos = Vector2i64.new(e.s, e.t):mul(m),
		dir = Vector2i64.new(b / e.gcd, -a / e.gcd)
	}
	return r
end
local function line_insersection(a, b)
	local a0 = a.pos
	local a1 = a.pos:add(a.dir)

	local b0 = b.pos
	local b1 = b.pos:add(b.dir)

	local p_div = (a0.x - a1.x) * (b0.y - b1.y) - (a0.y - a1.y) * (b0.x - b1.x)
	if p_div == 0 then
		return nil
	end

	local px = (a0.x*a1.y - a0.y*a1.x) * (b0.x - b1.x) - (a0.x - a1.x) * (b0.x*b1.y - b0.y*b1.x)
	local py = (a0.x*a1.y - a0.y*a1.x) * (b0.y - b1.y) - (a0.y - a1.y) * (b0.x*b1.y - b0.y*b1.x)
	if px % p_div ~= 0 or py % p_div ~= 0 then
		return nil
	end

	return Vector2i64.new(px / p_div, py / p_div)
end
local function solve_euclid_lines(machine)
	local dim_x = solve_euclid_dim(machine.a.x, machine.b.x, machine.p.x)
	local dim_y = solve_euclid_dim(machine.a.y, machine.b.y, machine.p.y)
	if dim_x == nil or dim_y == nil then
		return nil
	end

	local intersect = line_insersection(dim_x, dim_y)
	if intersect == nil then
		return nil
	end

	return intersect
end

local function solve_euclid_search(machine, c)
	local e = extended_gcd(machine.a.x, machine.b.x)
	local m = machine.p.x / e.gcd

	if math.floor(m) ~= m then
		return nil
	end

	local solutions = {}

	-- the start k, where x_x is the minimum positive integer
	local k = math.floor(e.s * m / (machine.b.x / e.gcd))
	local k_dir = -1
	while true do
		-- feasible solution - button_a_x * sol.x + button_b_x * sol.y == prize.x
		local sol = Vector2.new(
			e.s * m - k * (machine.b.x / e.gcd),
			e.t * m + k * (machine.a.x / e.gcd)
		)

		if sol.x < 0 or sol.y < 0 then
			break
		end
		-- print(aoc.dump(sol))
		-- check if the solution is feasible for y as well
		if sol:dot(Vector2.new(machine.a.y, machine.b.y)) == machine.p.y then
			table.insert(solutions, sol)
		end

		k = k + k_dir
	end

	if #solutions == 0 then
		return nil
	end

	local best_solution = solutions[1]
	local best_cost = c:dot(solutions[1])
	for _, sol in pairs(solutions) do
		local cost = c:dot(sol)
		if cost < best_cost then
			best_solution = sol
			best_cost = cost
		end
	end

	return best_solution
end

local button_cost = Vector2i64.new(3, 1)
-- Part 1
local token_cost = 0
for _, machine in pairs(claw_machines) do
	local solution = solve_euclid_lines(machine)
	if solution ~= nil then
		token_cost = token_cost + solution:dot(button_cost)
	end
end
print("token_cost", token_cost)

-- Part 2

local token_cost2 = 0
for _, machine in pairs(claw_machines) do
	local big_machine = {
		a = machine.a, b = machine.b,
		p = Vector2.new(machine.p.x + 10000000000000, machine.p.y + 10000000000000)
	}
	local solution = solve_euclid_lines(big_machine)
	if solution ~= nil then
		token_cost2 = token_cost2 + solution:dot(button_cost)
	end
end
print("token_cost2", token_cost2)
