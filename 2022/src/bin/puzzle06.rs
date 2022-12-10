use std::{io::Read, collections::HashSet};

use anyhow::Context;

use aoc_commons as base;
use base::anyhow;

// TODO: Should recollect the input string as [char] to be utf8 correct, but we are assuming ascii
fn find_start_of(sequence: &[u8], size: usize) -> usize {
	let mut result = 0;

	for (i, window) in sequence.windows(size).enumerate() {
		let deduplicated: HashSet<&u8> = HashSet::from_iter(window);
		if deduplicated.len() == size {
			result = i + window.len();
			break;
		}
	}

	result
}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let after_header_index = find_start_of(input.as_bytes(), 4);
	let after_message_index = find_start_of(input.as_bytes(), 14);

	println!("After header: {}", after_header_index);
	println!("After message: {}", after_message_index);

	Ok(())
}
