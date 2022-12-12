use std::{io::Read, collections::VecDeque};

use anyhow::Context;

use aoc_commons as base;
use base::{anyhow, log};

#[derive(Debug, Clone)]
struct BfsState {
	pub height: u8,
	pub visited_steps: Option<u32>
}
impl BfsState {
	pub fn new(height: u8) -> Self {
		Self { height, visited_steps: None }
	}

	pub fn visit(&mut self, steps: u32) -> bool {
		if self.visited_steps.is_none() {
			self.visited_steps = Some(steps);
			true
		} else {
			false
		}
	}
}

#[derive(Debug)]
struct BfsStackEntry {
	pub x: isize,
	pub y: isize,
	pub steps: u32
}
impl BfsStackEntry {
	pub fn new_start(x: isize, y: isize) -> Self {
		Self { x, y, steps: 0 }
	}

	pub fn next(&self, x: isize, y: isize) -> Self {
		Self {
			x: self.x + x,
			y: self.y + y,
			steps: self.steps + 1
		}
	}
}

#[derive(Clone)]
struct Map<T> {
	cells: Vec<T>,
	width: usize
}
impl<T> Map<T> {
	pub fn new(cells: Vec<T>, width: usize) -> Self {
		Self { cells, width }
	}

	pub fn width(&self) -> isize {
		self.width as isize
	}

	pub fn height(&self) -> isize {
		self.cells.len() as isize / self.width()
	}

	pub fn index_to_coords(&self, index: usize) -> Option<[isize; 2]> {
		if index >= self.cells.len() {
			None
		} else {
			let x = index % self.width;
			let y = index / self.width;
			Some([x as isize, y as isize])
		}
	}

	pub fn get(&self, x: isize, y: isize) -> Option<&T> {
		if x < 0 || x >= self.width() || y < 0 || y >= self.height() {
			return None;
		}

		self.cells.get(x as usize + y as usize * self.width)
	}

	// Initially copied from puzzle08 but not sure which approach is better
	pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
		if x < 0 || x >= self.width() || y < 0 || y >= self.height() {
			return None;
		}

		self.cells.get_mut(x as usize + y as usize * self.width)
	}
}

fn height_value(height: char) -> anyhow::Result<u8> {
	anyhow::ensure!(height >= 'a' && height <= 'z', "Height must be between 'a' and 'z'");

	Ok(
		(height as u32 - 'a' as u32) as u8
	)
}

fn do_search(mut map: Map<BfsState>, start: [isize; 2], end: [isize; 2]) -> anyhow::Result<u32> {
	log::debug!("Coordinates: {:?} -> {:?}", start, end);
	
	let mut bfs_stack = VecDeque::<BfsStackEntry>::new();
	bfs_stack.push_back(BfsStackEntry::new_start(start[0], start[1]));

	while let Some(current) = bfs_stack.pop_front() {
		let height = match map.get_mut(current.x, current.y) {
			None => None,
			Some(cell) => if cell.visit(current.steps) {
				log::trace!("Visited cell ({}, {}) @ {} with {} steps ", current.x, current.y, cell.height, current.steps);
				Some(cell.height)
			} else {
				None
			}
		};

		if let Some(height) = height {
			macro_rules! evaluate {
				($x: expr, $y: expr) => {
					let next = current.next($x, $y);
					match map.get(next.x, next.y) {
						None => (),
						Some(next_cell) => {
							log::trace!("Evaluating ({}, {}) @ {}", next.x, next.y, next_cell.height);
							if height + 1 >= next_cell.height {
								bfs_stack.push_back(next);
							}
						}
					}
				};
			}

			evaluate!(-1, 0);
			evaluate!(0, -1);
			evaluate!(1, 0);
			evaluate!(0, 1);
		}
	}

	let end = map.get(end[0], end[1]).unwrap();
	log::debug!("End: {:?}", end);
	end.visited_steps.context("Failed to visit end")
}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let width = input.find('\n').context("Failed to find newline in input")?;

	let mut cells = Vec::<BfsState>::new();
	let mut start = 0;
	let mut end = 0;
	for height_char in input.chars() {
		if height_char == 'S' {
			start = cells.len();
			cells.push(
				BfsState::new(height_value('a').unwrap())
			);
		} else if height_char == 'E' {
			end = cells.len();
			cells.push(
				BfsState::new(height_value('z').unwrap())
			);
		} else {
			match height_value(height_char) {
				Err(_) => (),
				Ok(height) => cells.push(BfsState::new(height as u8))
			}
		}
	}

	let map = Map::new(cells, width);
	let start = map.index_to_coords(start).context("Failed to parse start coordinates")?;
	let end = map.index_to_coords(end).context("Failed to parse end coordinates")?;

	let mut possible_starts = vec![start];
	for y in 0 .. map.height() as isize {
		for x in 0 .. map.width() as isize {
			match map.get(x, y) {
				Some(cell) if cell.height == 0 => {
					possible_starts.push([x, y]);
				},
				_ => ()
			}
		}
	}

	let mut ends = Vec::new();
	for start in possible_starts {
		match do_search(map.clone(), start, end) {
			Ok(end) => ends.push(end),
			Err(err) => log::warn!("Failed to reach end: {}", err)
		}
	}

	println!("Min steps: {}", ends[0]);
	println!("Min steps from any: {}", ends.iter().copied().min().unwrap());

	Ok(())
}
