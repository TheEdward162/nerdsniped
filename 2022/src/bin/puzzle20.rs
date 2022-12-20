use std::{
	fmt,
	io::Read
};

use anyhow::Context;

use aoc_commons as base;
use base::{anyhow, log};

struct Element {
	previous: usize,
	next: usize,
	value: isize
}
impl fmt::Debug for Element {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}[{}]{}", self.previous, self.value, self.next)
	}
}

#[derive(Debug)]
struct List {
	items: Vec<Element>
}
impl List {
	pub fn new(values: &[isize]) -> Self {
		let mut items = Vec::with_capacity(values.len());

		for &value in values {
			items.push(Element { previous: items.len().saturating_sub(1), next: items.len() + 1, value });
		}
		items.first_mut().unwrap().previous = items.len().saturating_sub(1);
		items.last_mut().unwrap().next = 0;

		Self { items }
	}

	fn swap(&mut self, ia: usize, forward: bool) {
		let (ibefore, ia, ib, iafter) = if forward {
			let ibefore = self.items[ia].previous;
			let ib = self.items[ia].next;
			assert_eq!(self.items[ib].previous, ia);
			let iafter = self.items[ib].next;

			(ibefore, ia, ib, iafter)
		} else {
			let ib = self.items[ia].previous;
			assert_eq!(self.items[ib].next, ia);
			let ibefore = self.items[ib].previous;
			let iafter = self.items[ia].next;

			(ibefore, ib, ia, iafter)
		};

		log::trace!(
			"Swapping (forward = {}): {:?}@{} - {:?}@{} - {:?}@{} - {:?}@{}",
			forward,
			self.items[ibefore], ibefore,
			self.items[ia], ia,
			self.items[ib], ib,
			self.items[iafter], iafter
		);

		self.items[ibefore].next = ib;
		self.items[ib].previous = ibefore;
		self.items[ib].next = ia;
		self.items[ia].previous = ib;
		self.items[ia].next = iafter;
		self.items[iafter].previous = ia;

		log::trace!(
			"Swapped: {:?}@{} - {:?}@{} - {:?}@{} - {:?}@{}",
			self.items[ibefore], ibefore,
			self.items[ib], ib,
			self.items[ia], ia,
			self.items[iafter], iafter
		);
	}

	pub fn len(&self) -> usize {
		self.items.len()
	}

	pub fn get(&self, i: usize) -> isize {
		self.items[i % self.items.len()].value
	}

	pub fn shift(&mut self, original_index: usize, shift: isize) {
		let mod_shift = shift % (self.items.len() as isize - 1);
		log::trace!("Shifting from {} by {} ({})", original_index, shift, mod_shift);

		for _ in 0 .. mod_shift.abs() as usize {
			self.swap(original_index, mod_shift.is_positive());
		}
	}

	pub fn in_order(&self) -> impl Iterator<Item = isize> + '_ {
		struct Iter<'a> {
			items: &'a [Element],
			index: Option<usize>
		}
		impl<'a> Iterator for Iter<'a> {
			type Item = isize;

			fn next(&mut self) -> Option<Self::Item> {
				match self.index {
					None => None,
					Some(index) => {
						let current = &self.items[index];
						if current.next == 0 {
							self.index = None;
						} else {
							self.index = Some(current.next);
						}

						Some(current.value)
					}
				}
			}
		}

		Iter { items: &self.items, index: Some(0) }
	}
}

const SUM_INDICES: [usize; 3] = [1000, 2000, 3000];

fn solve(numbers: &[isize], mix_count: usize) -> isize {
	let mut list = List::new(&numbers);

	for _ in 0 .. mix_count {
		for i in 0 .. list.len() {
			let value = list.get(i);
			list.shift(i, value);
		}
		log::trace!("List: {:?}", list);
	}
	
	let in_order: Vec<isize> = list.in_order().collect();
	let zero_index = in_order.iter().enumerate().find(|(_, &v)| v == 0).unwrap().0;
	
	let mut sum = 0;
	for i in SUM_INDICES {
		sum += in_order[(zero_index + i) % in_order.len()];
	}

	sum
}


const DECRYPTION_KEY: isize = 811589153;

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut numbers = Vec::<isize>::new();
	for line in input.lines().filter(|s| !s.is_empty()) {
		let number: isize = line.parse().context("Failed to parse line as a number")?;
		numbers.push(number);
	}
	log::trace!("Numbers: {:?}", numbers);

	// part1
	println!("Part 1: {}", solve(&numbers, 1));
	
	numbers.iter_mut().for_each(|n| *n *= DECRYPTION_KEY);
	println!("Part 2: {}", solve(&numbers, 10));
	
	log::info!("Done");

	Ok(())
}
