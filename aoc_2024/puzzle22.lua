require("aoc_2024/libaoc")

local input_lines = aoc.read_lines(arg[1])
local buyer_starts = {}
for line in aoc.iter(input_lines) do
	table.insert(buyer_starts, aoc.tonumber_i64(line))
end
-- print(aoc.dump(buyer_starts))

local PRNG_MODULO = 16777216
local terra prng_step(start: int64)
	var secret = start
	secret = (secret ^ (secret * 64)) % PRNG_MODULO
	secret = (secret ^ (secret / 32)) % PRNG_MODULO
	secret = (secret ^ (secret * 2048)) % PRNG_MODULO

	return secret
end
local function price_prng(start, steps)
	local res = {}
	
	local current = start
	for i = 1, steps do
		local next = prng_step(current)
		local price = next % 10
		table.insert(res, { value = next, price = price, change = price - (current % 10) })
		current = next
	end

	return res
end

local function changes_key(changes)
	local key = ""
	for _, change in pairs(changes) do
		local str = tostring(change)
		key = key .. str:sub(1, #str - 2)
	end
	return key
end

-- buyer_starts = { 1, 2, 3, 2024 }

-- Part 1
local buyers = {}
local all_sequenes = {}

local part1_sum = 0
for i, start in ipairs(buyer_starts) do
	if i % 100 == 0 then
		print(i .. "/" .. #buyer_starts)
	end

	local prices = price_prng(start, 2000)
	part1_sum = part1_sum + prices[#prices].value

	local sequences = {}
	for w, wi in aoc.windows(prices, 4) do
		local key = changes_key(aoc.map(function(p) return p.change end, w))
		local price = prices[wi].price

		if not aoc.map_has(sequences, key) then
			-- print(start, key, price)
			aoc.map_insert(sequences, key, price)
			aoc.set_insert(all_sequenes, key)
		end
	end
	table.insert(buyers, sequences)
end
print("part1_sum", part1_sum)

-- Part 2
local most_bananas = 0
for sequence in aoc.iter(aoc.keys(all_sequenes)) do
	local sequence_bananas = 0
	for buyer in aoc.iter(buyers) do
		local buyer_price = aoc.map_get(buyer, sequence)
		if buyer_price ~= nil then
			sequence_bananas = sequence_bananas + buyer_price
		end
	end
	if sequence_bananas > most_bananas then
		most_bananas = sequence_bananas
		print("new best", sequence_bananas)
	end
end
print("most_bananas", most_bananas)
