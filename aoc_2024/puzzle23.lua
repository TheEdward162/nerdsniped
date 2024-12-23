require("aoc_2024/libaoc")
local Matrix = aoc.Matrix

local input_lines = aoc.read_lines(arg[1])
local edges = {}
for line in aoc.iter(input_lines) do
	local from, to = unpack(aoc.string_split("-", line))
	table.insert(edges, { from, to })
end
-- print(aoc.dump(edges))

local node_index = 1
local node_indices = {}
local node_names = {}
for edge in aoc.iter(edges) do
	if not aoc.map_has(node_indices, edge[1]) then
		aoc.map_insert(node_indices, edge[1], node_index)
		aoc.map_insert(node_names, node_index, edge[1])
		node_index = node_index + 1
	end
	if not aoc.map_has(node_indices, edge[2]) then
		aoc.map_insert(node_indices, edge[2], node_index)
		aoc.map_insert(node_names, node_index, edge[2])
		node_index = node_index + 1
	end
end
-- print(aoc.dump(node_indices))
-- print(aoc.dump(node_names))
local nodes = {}
for node in aoc.iter(aoc.map_values(node_indices)) do
	aoc.set_insert(nodes, node)
end

local adjacency_matrix = Matrix.new(aoc.map_len(node_indices), aoc.map_len(node_indices), 0)
for edge in aoc.iter(edges) do
	local from_i = aoc.map_get(node_indices, edge[1])
	local to_i = aoc.map_get(node_indices, edge[2])

	adjacency_matrix:set(from_i, to_i, 1)
	adjacency_matrix:set(to_i, from_i, 1)
end
-- print(aoc.dump(adjacency_matrix))

local function print_clique(clique)
	local names = aoc.map(
		function(n) return aoc.map_get(node_names, n) end,
		clique
	)
	print(aoc.join(",", names))
end

local function clique_name(clique)
	local names = aoc.map(
		function(n) return aoc.map_get(node_names, n) end,
		aoc.set_values(clique)
	)
	table.sort(names)
	return aoc.join(",", names)
end

--[[
algorithm BronKerbosch1(R, P, X) is
    if P and X are both empty then
        report R as a maximal clique
    for each vertex v in P do
        BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
        P := P \ {v}
        X := X ⋃ {v}
]]--
local function neighbor_set(adj, center)
	local nodes = {}
	for x = 1, adj.width do
		if adj:get(x, center) == 1 then
			aoc.set_insert(nodes, x)
		end
	end
	return nodes
end
local function bron_kerbosch_base(adj, partial, avail_nodes, rejected_nodes)
	if aoc.set_len(avail_nodes) == 0 and aoc.set_len(rejected_nodes) == 0 then
		coroutine.yield(partial)
	end

	local avail_values = aoc.set_values(avail_nodes)
	for node in aoc.iter(avail_values) do
		local next_partial = aoc.copy_shallow(partial)
		aoc.set_insert(next_partial, node)

		local node_neighbors = neighbor_set(adj, node)
		local next_avail = aoc.set_intersect(avail_nodes, node_neighbors)
		local next_rejected = aoc.set_intersect(rejected_nodes, node_neighbors)

		bron_kerbosch_base(adj, next_partial, next_avail, next_rejected)
		aoc.set_remove(avail_nodes, node)
		aoc.set_insert(rejected_nodes, node)
	end
end
local function find_cliques(adj, nodes)
	return coroutine.wrap(function()
		bron_kerbosch_base(adj, {}, aoc.copy_shallow(nodes), {})
	end)
end

-- Part 1 & Part 2
local global_max_clique = {}
local cliques_with_t = {}
for max_clique in find_cliques(adjacency_matrix, nodes) do
	local len = aoc.set_len(max_clique)

	if len > aoc.set_len(global_max_clique) then
		global_max_clique = max_clique
	end
	
	if len >= 3 then
		for clique in aoc.combinations(aoc.values(max_clique), 3) do
			local name = clique_name(clique)
			if name:sub(1, 1) == "t" or string.find(name, ",t") ~= nil then
				aoc.set_insert(cliques_with_t, name)
			end
		end
	end
end
print("cliques_with_start_t", aoc.set_len(cliques_with_t))
print("global_max_clique", clique_name(global_max_clique))
