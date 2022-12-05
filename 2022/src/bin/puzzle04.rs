use std::{io::Read, ops::RangeInclusive};

use anyhow::Context;

use aoc2022 as base;

fn parse_range(value: &str) -> anyhow::Result<RangeInclusive<u32>> {
	let (start_str, end_str) = value.split_once("-").context("Failed to split range by -")?;

	let start: u32 = start_str.parse().context("Failed to parse start number")?;
	let end: u32 = end_str.parse().context("Failed to parse end number")?;

	Ok(start ..= end)
}

fn partial_overlap(left: &RangeInclusive<u32>, right: &RangeInclusive<u32>) -> bool {
	(left.start() >= right.start() && left.start() <= right.end())
	|| (right.start() >= left.start() && right.start() <= left.end())
}

fn full_overlap(left: &RangeInclusive<u32>, right: &RangeInclusive<u32>) -> bool {
	(left.start() >= right.start() && left.end() <= right.end())
	|| (right.start() >= left.start() && right.end() <= left.end())
}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut full_overlap_count = 0;
	let mut partial_overlap_count = 0;
	for pair_str in input.split('\n').filter(|s| !s.is_empty()) {
		let (left_str, right_str) = pair_str.split_once(',').context("Failed to split pair by ,")?;

		let left = parse_range(left_str)?;
		let right = parse_range(right_str)?;

		if full_overlap(&left, &right) {
			full_overlap_count += 1;
		} else if partial_overlap(&left, &right) {
			partial_overlap_count += 1;
		}
	}

	println!("Full overlap: {}", full_overlap_count);
	println!("Partial overlap: {}", partial_overlap_count);
	println!("Total overlap: {}", full_overlap_count + partial_overlap_count);

	Ok(())
}