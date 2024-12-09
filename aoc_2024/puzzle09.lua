require("aoc_2024/libaoc")

local input_file_blocks = {}
local input_file = io.open(arg[1], "r")
local input_on_file = true
local input_offset = 0
while true do
	local char = input_file:read(1)
	if char == nil then
		break
	end
	local block_len = tonumber(char)
	if block_len == nil then
		break
	end

	if input_on_file then
		table.insert(input_file_blocks, { offset = input_offset, len = block_len, file_id = #input_file_blocks })
	end
	input_on_file = not input_on_file
	input_offset = input_offset + block_len
end
input_file:close()

-- Part 1
local file_blocks = {}
for _, v in pairs(input_file_blocks) do
	table.insert(file_blocks, { offset = v.offset, len = v.len, file_id = v.file_id })
end

local checksum = 0
local function checksum_add(offset, file_id, dbg)
	checksum = checksum + offset * file_id
end

local current_block_left = 1
local current_block_right = #file_blocks
while current_block_left <= #file_blocks do
	local block_left = file_blocks[current_block_left]
	-- add left block to checksum
	for i = block_left.offset, block_left.offset + block_left.len - 1 do
		checksum = checksum + i * block_left.file_id
	end

	if current_block_left >= current_block_right or current_block_left == #file_blocks then
		break
	end

	-- move as many right blocks to left as possible and add them to checksum
	local empty_offset = block_left.offset + block_left.len
	local empty_offset_end = file_blocks[current_block_left + 1].offset
	while empty_offset < empty_offset_end do
		if current_block_right <= current_block_left then
			break
		end

		local block_right = file_blocks[current_block_right]
		-- TODO: breaks for 0101

		if empty_offset_end - empty_offset >= block_right.len then
			-- move all of the right block to the left
			for i = empty_offset, empty_offset + block_right.len - 1 do
				checksum = checksum + i * block_right.file_id
			end
			empty_offset = empty_offset + block_right.len
			block_right.len = 0
			current_block_right = current_block_right - 1
		else
			-- move part of the right block to the left
			for i = empty_offset, empty_offset_end - 1 do
				checksum = checksum + i * block_right.file_id
			end
			block_right.len = block_right.len - (empty_offset_end - empty_offset)
			break
		end
	end

	current_block_left = current_block_left + 1
end
print("checksum", checksum)

-- Part 2
local file_blocks = {}
for _, v in pairs(input_file_blocks) do
	table.insert(file_blocks, { offset = v.offset, len = v.len, file_id = v.file_id, moved = false })
end

local checksum2 = 0

local current_block_left = 1
local current_block_right = #file_blocks
while current_block_left <= #file_blocks do
	local block_left = file_blocks[current_block_left]

	if not block_left.moved then
		-- add left block to checksum
		for i = block_left.offset, block_left.offset + block_left.len - 1 do
			checksum2 = checksum2 + i * block_left.file_id
		end
	end

	if current_block_left == #file_blocks then
		break
	end

	-- check if we can move any block from right here
	local empty_offset = block_left.offset + block_left.len
	local empty_offset_end = file_blocks[current_block_left + 1].offset
	for current_block_right = #file_blocks, current_block_left+1, -1 do
		local block_right = file_blocks[current_block_right]
		if not block_right.moved then
			if empty_offset_end - empty_offset >= block_right.len then
				-- we can move the block
				for i = empty_offset, empty_offset + block_right.len - 1 do
					checksum2 = checksum2 + i * block_right.file_id
				end
				block_right.moved = true
				empty_offset = empty_offset + block_right.len
			end
		end
	end

	current_block_left = current_block_left + 1
end
print("checksum2", checksum2)
