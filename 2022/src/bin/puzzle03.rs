use std::{collections::HashSet, io::Read};

use anyhow::Context;

use aoc_commons as aoc;
use aoc::anyhow;

fn priority(item: char) -> anyhow::Result<u32> {
	anyhow::ensure!((item as u32) < 256, "Invalid item value");

	if item >= 'a' && item <= 'z' {
		Ok(item as u32 - 'a' as u32 + 1)
	} else if item >= 'A' && item <= 'Z' {
		Ok(item as u32 - 'A' as u32 + 27)
	} else {
		anyhow::bail!("Invalid item {}", item)
	}
}

fn main() -> anyhow::Result<()> {
	let mut file = aoc::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut total_priority = 0;
	let mut all_rucksacks = Vec::new();
	for contents_str in input.split('\n').filter(|s| !s.is_empty()) {
		let (left_str, right_str) = contents_str.split_at(contents_str.len() / 2);

		let left_items: HashSet<char> = left_str.chars().collect();
		let right_items: HashSet<char> = right_str.chars().collect();

		total_priority += left_items
			.intersection(&right_items)
			.map(|i| priority(*i))
			.try_fold(0, |acc, res| res.map(|curr| acc + curr))
		?;

		all_rucksacks.push(
				left_items.union(&right_items).copied().collect::<HashSet<char>>()
		);
	}
	println!("Total priority: {}", total_priority);

	let mut total_trio_priority = 0;
	for group in all_rucksacks.chunks(3) {
		let intersection = group[0].intersection(&group[1]).filter(|item| group[2].contains(item)).copied().nth(0).expect("group intersection is empty");
		total_trio_priority += priority(intersection)?;
	}
	println!("Total trio priority: {}", total_trio_priority);

	Ok(())
}
