use std::{io::Read, fmt};

use anyhow::Context;

use aoc2022 as base;

#[derive(Clone, Copy)]
struct Visibility {
	pub left: i8,
	pub top: i8,
	pub right: i8,
	pub bottom: i8
}
impl Visibility {
	pub fn min(self, other: Self) -> Self {
		Self {
			left: self.left.min(other.left),
			top: self.top.min(other.top),
			right: self.right.min(other.right),
			bottom: self.bottom.min(other.bottom)
		}
	}

	pub fn total(&self) -> i8 {
		self.left.min(self.top).min(self.right).min(self.bottom)
	}
}
impl fmt::Debug for Visibility {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[L{} T{} R{} B{}]", self.left, self.top, self.right, self.bottom)
	}
}

struct Map<T> {
	trees: Vec<T>,
	width: usize
}
impl<T> Map<T> {
	pub fn new(trees: Vec<T>, width: usize) -> Self {
		Self { trees, width }
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.trees.len() / self.width
	}

	pub fn get(&self, x: usize, y: usize) -> Option<&T> {
		self.trees.get(x + y * self.width)
	}

	pub fn get_offset(&self, x: isize, y: isize) -> Option<&T> {
		if x < 0 || (x as usize) >= self.width || y < 0 || (y as usize) >= self.height() {
			return None;
		}
		
		self.get(x as usize, y as usize)
	}

	pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
		if x >= self.width || y >= self.height() {
			return None;
		}
		
		self.trees.get_mut(x + y * self.width)
	}
}
impl<T: Copy> Map<T> {
	pub fn new_fill(fill: T, width: usize, height: usize) -> Self {
		Self::new(vec![fill; width * height], width)
	}
}
impl<T: fmt::Debug> fmt::Debug for Map<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for y in 0 .. self.height() {
			for x in 0 .. self.width {
				write!(f, "{:?}", self.get(x, y).unwrap())?;
			}
			writeln!(f)?;
		}

		Ok(())
	}
}

fn evaluate_visibility(
	map: &Map<i8>,
	map_visibility: &mut Map<Visibility>,
	x: usize,
	y: usize
) -> anyhow::Result<()> {
	macro_rules! evaluate {
		($dir: ident; $x_diff: literal, $y_diff: literal) => {
			{
				let sx = (x as isize) + $x_diff;
				let sy = (y as isize) + $y_diff;
				map_visibility.get_offset(sx, sy).map(|v| v.$dir).unwrap_or(-1).max(
					map.get_offset(sx, sy).copied().unwrap_or(-1)
				)
			}
		}
	}

	let around = Visibility {
		left: evaluate!(left; -1, 0),
		top: evaluate!(top; 0, -1),
		right: evaluate!(right; 1, 0),
		bottom: evaluate!(bottom; 0, 1)
	};
	
	let vis = map_visibility.get_mut(x, y).context("Invalid position to evaluate")?;
	let new_vis = vis.min(around);

	log::trace!(
		"({}, {}): {:?}: {:?} -> {:?}",
		x, y, around, vis, new_vis
	);
	
	*vis = new_vis;

	Ok(())
}

fn evaluate_scenic(map: &Map<i8>, x: usize, y: usize) -> anyhow::Result<usize> {
	let center = *map.get(x, y).context("Invalid center tree")?;
	
	macro_rules! evaluate {
		($x_diff: literal, $y_diff: literal) => {
			{
				let mut i: usize = 1;
				loop {
					let sx = x as isize + $x_diff * i as isize;
					let sy = y as isize + $y_diff * i as isize;

					match map.get_offset(sx, sy) {
						None => break i - 1,
						Some(&tree) if tree >= center => break i,
						_ => ()
					}

					i += 1;
				}
			}
		};
	}

	let score = evaluate!(-1, 0) * evaluate!(0, -1) * evaluate!(1, 0) * evaluate!(0, 1);

	Ok(score)
}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let width = input.find('\n').context("Failed to find newline in input")?;
	let mut trees = Vec::new();
	for tree_char in input.chars() {
		if let Some(height) = tree_char.to_digit(10) {
			trees.push(height as i8);
		}
	}

	let map = Map::new(trees, width);
	log::debug!("Map:\n{:?}", map);
	
	let mut map_visibility = Map::new_fill(Visibility { left: 9, top: 9, right: 9, bottom: 9 }, map.width(), map.height());
	for y in 0 .. map.height() {
		for x in 0 .. map.width() {
			evaluate_visibility(&map, &mut map_visibility, x, y)?;

			// mirror vis
			let mirror_x = map.width() - x - 1;
			let mirror_y = map.height() - y - 1;
			evaluate_visibility(&map, &mut map_visibility, mirror_x, mirror_y)?;
		}
	}

	let mut map_visible = Map::new_fill(0, map.width(), map.height());
	let mut total_visible = 0;
	for y in 0 .. map.height() {
		for x in 0 .. map.width() {
			let tree = map.get(x, y).unwrap();
			let vis = map_visibility.get(x, y).unwrap();

			if *tree > vis.total() {
				*map_visible.get_mut(x, y).unwrap() = 1;
				total_visible += 1;
			}
		}
	}
	log::debug!("Map visible:\n{:?}", map_visible);

	let mut max_scenic = 0;
	for y in 0 .. map.height() {
		for x in 0 .. map.width() {
			let scenic = evaluate_scenic(&map, x, y)?;

			if scenic > max_scenic {
				max_scenic = scenic;
			}

		}
	}
	
	println!("Total visible: {}", total_visible);
	println!("Max scenic: {}", max_scenic);

	Ok(())
}
