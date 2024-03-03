use std::{io::Read, collections::VecDeque};

use anyhow::Context;

use aoc_commons as aoc;
use aoc::{anyhow, log, geometry::{Grid2, Point}};

type Point2 = Point<2>;

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
	pub pos: Point2,
	pub steps: u32
}
impl BfsStackEntry {
	pub fn new_start(pos: Point2) -> Self {
		Self { pos, steps: 0 }
	}

	pub fn next(&self, x: isize, y: isize) -> Self {
		Self {
			pos: self.pos + Point2::new(x, y),
			steps: self.steps + 1
		}
	}
}

fn height_value(height: char) -> anyhow::Result<u8> {
	anyhow::ensure!(height >= 'a' && height <= 'z', "Height must be between 'a' and 'z'");

	Ok(
		(height as u32 - 'a' as u32) as u8
	)
}

fn do_search(mut map: Grid2<BfsState>, start: Point2, end: Point2) -> anyhow::Result<u32> {
	log::debug!("Coordinates: {} -> {}", start, end);
	
	let mut bfs_stack = VecDeque::<BfsStackEntry>::new();
	bfs_stack.push_back(BfsStackEntry::new_start(start));

	while let Some(current) = bfs_stack.pop_front() {
		let height = match map.get_mut(current.pos) {
			None => None,
			Some(cell) => if cell.visit(current.steps) {
				log::trace!("Visited cell {} @ {} with {} steps ", current.pos, cell.height, current.steps);
				Some(cell.height)
			} else {
				None
			}
		};

		if let Some(height) = height {
			macro_rules! evaluate {
				($x: expr, $y: expr) => {
					let next = current.next($x, $y);
					match map.get(next.pos) {
						None => (),
						Some(next_cell) => {
							log::trace!("Evaluating {} @ {}", next.pos, next_cell.height);
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

	let end = map.get(end).unwrap();
	log::debug!("End: {:?}", end);
	end.visited_steps.context("Failed to visit end")
}

fn main() -> anyhow::Result<()> {
	let mut file = aoc::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let width = input.find('\n').context("Failed to find newline in input")? as isize;

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

	let map = Grid2::new_width(cells, width)?;
	let start = map.index_to_coords(start).context("Failed to parse start coordinates")?;
	let end = map.index_to_coords(end).context("Failed to parse end coordinates")?;

	let mut possible_starts = vec![start];
	for y in map.y_range() {
		for x in map.x_range() {
			let p = Point2::new(x, y);
			match map.get(p) {
				Some(cell) if cell.height == 0 => {
					possible_starts.push(p);
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
