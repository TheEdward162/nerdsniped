use std::io::Read;

use anyhow::Context;

use aoc2022 as base;

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut all_inventories = Vec::new();
	for inventory_str in input.split("\n\n") {
		let mut one_inventory = Vec::new();
		for item_str in inventory_str.split('\n').filter(|s| !s.is_empty()) {
			let calories = item_str.parse::<u32>().context("Failed to parse input item")?;

			one_inventory.push(calories);
		}

		all_inventories.push(one_inventory);
	}

	let mut inventory_sums: Vec<u32> = all_inventories.iter().map(
		|inv| -> u32 { inv.iter().sum() }
	).collect();
	inventory_sums.sort_by(|x, y| x.cmp(y).reverse());

	println!("top three calories: {:?}", &inventory_sums[..3]);
	println!("top three calories sum: {}", (&inventory_sums[..3]).iter().sum::<u32>());

	Ok(())
}
