require("aoc_2024/libaoc")

input_reports = {}

input_file = io.open(arg[1], "r")
while true do
	line = input_file:read("l")
	if line == nil then
		break
	end

	report_items_str = aoc_string_split_space(line)
	report_items = aoc_map(tonumber, report_items_str)

	table.insert(input_reports, report_items)
end
input_file:close()

-- print(aoc_dump(input_reports))

-- Part 1
function list_diff(list)
	local res = {}
	local prev = nil
	for _, v in pairs(list) do
		if prev ~= nil then
			table.insert(res, v - prev)
		end
		prev = v
	end
	return res
end

function math_sign(number)
	return number > 0 and 1 or (number == 0 and 0 or -1)
end

function is_report_safe(report)
	local report_diff = list_diff(report)
	local report_safe = true
	local diff_direction = 0
	for _, diff in pairs(report_diff) do
		if math.abs(diff) <= 0 or math.abs(diff) > 3 then
			report_safe = false
			break
		end

		if diff_direction ~= 0 and math_sign(diff) ~= diff_direction then
			report_safe = false
			break
		end
		diff_direction = math_sign(diff)
	end

	return report_safe
end

safe_reports = 0
for _, report in pairs(input_reports) do
	if is_report_safe(report) then
		safe_reports = safe_reports + 1
	end
end
print("safe_reports", safe_reports)

-- Part 2
safe_reports_dampened = 0
for _, report in pairs(input_reports) do
	if is_report_safe(report) then
		safe_reports_dampened = safe_reports_dampened + 1
	else
		for i, _ in ipairs(report) do
			dampened_report = aoc_splice(report, i, i + 1)
			if is_report_safe(dampened_report) then
				safe_reports_dampened = safe_reports_dampened + 1
				break
			end
		end
	end
end
print("safe_reports_dampened", safe_reports_dampened)