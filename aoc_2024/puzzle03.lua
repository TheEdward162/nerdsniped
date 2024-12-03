require("aoc_2024/libaoc")

local EMBED_DFA_MARKER_KEY = "__mark_embed_dfa"

--[[
Performs the merge of one DFA into another.

This is used for embedded DFAs.
]]--
function merge_dfa_into(host_states, dfa_source, state_renames)
	local merged_state_names_map = {}

	-- compute a new names for the states
	for state_name, _ in pairs(dfa_source.states) do
		local merged_state_name = state_renames[state_name]
		if merged_state_name == nil then
			merged_state_name = start_state_rename .. "__" .. state_name
		end

		merged_state_names_map[state_name] = merged_state_name
	end

	local function merge_dfa_action(host, embed)
		local host_is_empty = host == nil or host == false
		local embed_is_empty = embed == false
		
		if host_is_empty and embed_is_empty then
			return false
		elseif host_is_empty then
			host = {}
		elseif embed_is_empty then
			embed = {}
		end
	
		local res = {}
		for state_input, next_state_name in pairs(embed) do
			res[state_input] = merged_state_names_map[next_state_name]
		end
		-- let host override embedded
		for state_input, next_state_name in pairs(host) do
			res[state_input] = next_state_name
		end
		return res
	end

	for state_name, action in pairs(dfa_source.states) do
		local dest_state_name = merged_state_names_map[state_name]
		host_states[dest_state_name] = merge_dfa_action(host_states[dest_state_name], action)
	end
end
--[[
Generate a state machine (DFA)

Arguments:
states: table[state_name: str -> state_transition_function]
start_state: str
end_states: list[str]

Returns:
	A type which represents the state machine

The state machine has the following functions:
machine:init()
	Initializes the machine the first time.
machine:reset()
	Resets the machine to its start state.
machine:input(ch: uint32): int8
	Advances the state machine with given input.
	Returns:
		* -1 for error (no possible next state)
		* 0 for continue
		* 1 for end state reached (but can be continued)
]]--
function make_dfa(name, input_states, start_state, end_states)
	-- pre-process states to handle embedded DFAs
	local states_embed, states = aoc.split(
		function(action) return type(action) == "table" and action[EMBED_DFA_MARKER_KEY] == true end,
		input_states
	)
	for state_name, action in pairs(states_embed) do
		local state_renames = {
			[action.dfa.dfa_source.start_state] = state_name
		}
		for i, v in ipairs(action.end_states) do
			state_renames[action.dfa.dfa_source.end_states[i]] = v
		end
		merge_dfa_into(states, action.dfa.dfa_source, state_renames)
	end

	-- assign indices to states
	local index_to_state = aoc.keys(states)
	local state_to_index = aoc.invert(index_to_state)
	
	local struct Dfa {
		_state: int
	}
	Dfa.name = name
	-- preserve the source for embedding
	Dfa.dfa_source = {
		start_state = start_state,
		end_states = end_states,
		states = input_states,
		index_to_state = index_to_state
	}

	terra Dfa:init()
		self._state = [ state_to_index[start_state] ]
	end
	terra Dfa:reset()
		self:init()
	end
	terra Dfa:current_state()
		return self._state
	end

	-- ffi = require("ffi")
	-- ffi.string(dfa:current_state_str()
	-- local function generate_current_state_str(state_symbol)
	-- 	local cases = {}
	-- 	for state_index, state_name in pairs(index_to_state) do
	-- 		table.insert(
	-- 			cases,
	-- 			quote
	-- 				case [ state_index ] then
	-- 					return [ state_name ]
	-- 				end
	-- 			end
	-- 		)
	-- 	end
	-- 	return quote
	-- 		switch state_symbol do
	-- 			[ cases ]
	-- 		end
	-- 	end
	-- end
	-- terra Dfa:current_state_str()
	-- 	var current_state: int = self._state
	-- 	[ generate_current_state_str(current_state) ]
	-- end

	local function generate_input_action_map(action_map, state_to_index, ch_symbol, next_state_symbol)
		local cases = {}
		local else_block = quote end
		for ch_case, next_state_name in pairs(action_map) do
			if ch_case == "else" then
				else_block = quote
					next_state_symbol = [ state_to_index[next_state_name] ]
				end
			elseif #ch_case ~= 1 then
				error("Expected a single character or `else`, got " .. ch_case)
			else
				table.insert(
					cases,
					quote
						case [ string.byte(ch_case) ] then
							next_state_symbol = [ state_to_index[next_state_name] ]
						end
					end
				)
			end
		end
		return quote
			switch ch_symbol do
				[ cases ]
			else
				[ else_block ]
			end
		end
	end
	local function generate_input_action(action, state_to_index, ch_symbol, next_state_symbol)
		-- action always fails
		if action == false then
			return quote end
		end
		-- simple map
		if type(action) == "table" then
			return generate_input_action_map(action, state_to_index, ch_symbol, next_state_symbol)
		end

		return nil
	end
	local function generate_input_switch(state_symbol, ch_symbol, next_state_symbol)
		local cases = {}
		for state_name, action in pairs(states) do
			table.insert(
				cases,
				quote
					case [ state_to_index[state_name] ] then
						[ generate_input_action(action, state_to_index, ch_symbol, next_state_symbol) ]
					end
				end
			)
		end
		return quote
			switch state_symbol do
				[ cases ]
			end
		end
	end
	local function generate_end_state_switch(state_symbol)
		local cases = {}
		for _, end_state_name in pairs(end_states) do
			table.insert(
				cases,
				quote
					case [ state_to_index[end_state_name] ] then
						return 1
					end
				end
			)
		end
		return quote
			switch state_symbol do
				[ cases ]
			else
				return 0
			end
		end
	end
	terra Dfa:input(ch: uint32): int8
		var next_state: int = 0
		var current_state: int = self._state
		[ generate_input_switch(current_state, ch, next_state) ]

		if next_state == 0 then
			return -1
		end

		self._state = next_state
		[ generate_end_state_switch(next_state) ]
	end

	return Dfa
end
--[[
Marks the DFA into a marker table for embedding.

The `end_state` is what the end state of the existing DFA is mapped to.
The start state name is computed during `make_dfa` where the actual merging takes place.
]]--
function mark_embed_dfa(T, end_states)
	return { [EMBED_DFA_MARKER_KEY] = true, dfa = T, end_states = end_states }
end

-- groups is: { name = tuple(from_state, to_state) }
function apply_dfa(T, input, groups)
	if groups == nil then
		groups = {}
	end
	local checkpoints = {}
	for _, states in pairs(groups) do
		checkpoints[states[1]] = true
		checkpoints[states[2]] = true
	end

	local dfa = terralib.new(T)
	dfa:init()

	local last_match = 0
	local last_end_state = 0
	local checkpoint_matches = {}
	for i = 1, #input do
		local char = string.byte(input, i, i)
		local res = dfa:input(char)

		-- on error we have to revert checkpoints
		if res == -1 then
			local reverted_checkpoint_matches = {}
			for checkpoint, matches in pairs(checkpoint_matches) do
				local reverted_matches = aoc.filter(function(m) return m <= last_match end, matches)
				if #reverted_matches > 0 then
					reverted_checkpoint_matches[checkpoint] = reverted_matches
				end
			end
			checkpoint_matches = reverted_checkpoint_matches
			break
		end

		local current_state = T.dfa_source.index_to_state[dfa:current_state()]
		if res == 1 then
			last_match = i
			last_end_state = current_state
		end

		if checkpoints[current_state] ~= nil then
			if checkpoint_matches[current_state] == nil then
				checkpoint_matches[current_state] = {}
			end
			table.insert(checkpoint_matches[current_state], i + 1)
		end
	end

	local group_matches = {}
	for group_name, group_states in pairs(groups) do
		local checkpoint_from = checkpoint_matches[group_states[1]]
		local checkpoint_to = checkpoint_matches[group_states[2]]
		if checkpoint_from ~= nil and checkpoint_to ~= nil then
			group_matches[group_name] = input:sub(aoc.min(checkpoint_from), aoc.max(checkpoint_to) - 1)
		end
	end
	
	return last_end_state, last_match, group_matches
end

input_file = io.open(arg[1], "r")
input_string = input_file:read("a")
input_file:close()

-- Part 1
IntegerDfa = make_dfa(
	"IntegerDfa",
	{
		start = {
			["1"] = "digit",
			["2"] = "digit",
			["3"] = "digit",
			["4"] = "digit",
			["5"] = "digit",
			["6"] = "digit",
			["7"] = "digit",
			["8"] = "digit",
			["9"] = "digit",
		},
		digit = {
			["0"] = "digit",
			["1"] = "digit",
			["2"] = "digit",
			["3"] = "digit",
			["4"] = "digit",
			["5"] = "digit",
			["6"] = "digit",
			["7"] = "digit",
			["8"] = "digit",
			["9"] = "digit"
		}
	},
	"start", {"digit"}
)
MulExprDfa = make_dfa(
	"MulExprDfa",
	{
		start = { m = "m" },
		m = { u = "mu" },
		mu = { l = "mul" },
		mul = { ["("] = "mul(" },
		["mul("] = mark_embed_dfa(IntegerDfa, {"mul(op_a"}),
		["mul(op_a"] = { [","] = "mul(op_a," },
		["mul(op_a,"] = mark_embed_dfa(IntegerDfa, {"mul(op_a,op_b"}),
		["mul(op_a,op_b"] = { [")"] = "mul(op_a,op_b)" },
		["mul(op_a,op_b)"] = false
	},
	"start", {"mul(op_a,op_b)"}
)
DoDontDfa = make_dfa(
	"DoDontDfa",
	{
		start = { d = "d" },
		d = { o = "do" },
		["do"] = { n = "don", ["("] = "do(" },
		["do("] = { [")"] = "do()" },
		["do()"] = false,

		don = { ["'"] = "don'" },
		["don'"] = { t = "don't" },
		["don't"] = { ["("] = "don't(" },
		["don't("] = { [")"] = "don't()" },
		["don't()"] = false
	},
	"start", { "do()", "don't()" }
)

local part1_sum = 0
local part2_sum = 0

local current_input = input_string
local groups = { op_a = {"mul(", "mul(op_a"}, op_b = {"mul(op_a,", "mul(op_a,op_b"} }
local is_enabled = true

while #current_input > 0 do
	local do_match, match_len = apply_dfa(DoDontDfa, current_input)
	if do_match == "do()" then
		is_enabled = true
		current_input = current_input:sub(match_len)
	elseif do_match == "don't()" then
		is_enabled = false
		current_input = current_input:sub(match_len)
	else
		local _, match_len, group_matches = apply_dfa(MulExprDfa, current_input, groups)
		if match_len == 0 then
			current_input = current_input:sub(2)
		else
			local matched_str = current_input:sub(1, match_len)
			current_input = current_input:sub(match_len)

			op_a = tonumber(group_matches.op_a)
			op_b = tonumber(group_matches.op_b)
			part1_sum = part1_sum + op_a * op_b
			if is_enabled then
				part2_sum = part2_sum + op_a * op_b
			end
		end
	end
end
print("part1_sum", part1_sum)
print("part2_sum", part2_sum)
