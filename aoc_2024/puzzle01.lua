input_left = {}
input_right = {}

input_file = io.open(arg[1], "r")
while true do
	left = input_file:read("n")
	right = input_file:read("n")
	if left == nil or right == nil then
		break
	end

	table.insert(input_left, left)
	table.insert(input_right, right)
end
input_file:close()

-- Part 1
table.sort(input_left)
table.sort(input_right)

diff_sum = 0
for i = 1, #input_left do
	diff_sum = diff_sum + (math.abs(input_left[i] - input_right[i]))
end
print("diff_sum", diff_sum)

-- Part 2
right_freq = {}
for k, v in pairs(input_right) do
	if right_freq[v] == nil then
		right_freq[v] = 0
	end
	right_freq[v] = right_freq[v] + 1
end

similarity_score = 0
for k, v in pairs(input_left) do
	freq = right_freq[v]
	if freq ~= nil then
		similarity_score = similarity_score + v * freq
	end
end
print("similarity_score", similarity_score)
