use std::{io::Read, fmt};

use anyhow::Context;

use aoc_commons as base;
use base::{anyhow, log, geometry::{Grid2, Point2}};

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

fn evaluate_visibility(
	map: &Grid2<i8>,
	map_visibility: &mut Grid2<Visibility>,
	at: Point2
) -> anyhow::Result<()> {
	macro_rules! evaluate {
		($dir: ident; $x_diff: literal, $y_diff: literal) => {
			{
				let sat = at + Point2::new($x_diff, $y_diff);
				map_visibility.get(sat).map(|v| v.$dir).unwrap_or(-1).max(
					map.get(sat).copied().unwrap_or(-1)
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

	let vis = map_visibility.get_mut(at).context("Invalid position to evaluate")?;
	let new_vis = vis.min(around);

	log::trace!(
		"{}: {:?}: {:?} -> {:?}",
		at, around, vis, new_vis
	);
	
	*vis = new_vis;

	Ok(())
}

fn evaluate_scenic(map: &Grid2<i8>, at: Point2) -> anyhow::Result<usize> {
	let center = *map.get(at).context("Invalid center tree")?;
	
	macro_rules! evaluate {
		($x_diff: literal, $y_diff: literal) => {
			{
				let mut i: isize = 1;
				loop {
					let sat = at + Point2::new($x_diff, $y_diff) * i;
					match map.get(sat) {
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

	Ok(score as usize)
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

	let map = Grid2::new_width(trees, width as isize)?;
	log::debug!("Map:\n{:?}", map);
	
	let mut map_visibility = Grid2::new_fill(Visibility { left: 9, top: 9, right: 9, bottom: 9 }, map.bounding_box())?;
	for y in map.y_range() {
		for x in map.x_range() {
			evaluate_visibility(&map, &mut map_visibility, Point2::new(x, y))?;

			// mirror vis
			let b = map.bounding_box();
			evaluate_visibility(&map, &mut map_visibility, Point2::new(
				b.max.x - x + b.min.x - 1,
				b.max.y - y + b.min.y - 1
			))?;
		}
	}

	let mut map_visible = Grid2::new_fill(0, map.bounding_box())?;
	let mut total_visible = 0;
	for y in map.y_range() {
		for x in map.x_range() {
			let tree = map.get(Point2::new(x, y)).unwrap();
			let vis = map_visibility.get(Point2::new(x, y)).unwrap();

			if *tree > vis.total() {
				*map_visible.get_mut(Point2::new(x, y)).unwrap() = 1;
				total_visible += 1;
			}
		}
	}
	log::debug!("Map visible:\n{:?}", map_visible);

	let mut max_scenic = 0;
	for y in map.y_range() {
		for x in map.x_range() {
			let scenic = evaluate_scenic(&map, Point2::new(x, y))?;

			if scenic > max_scenic {
				max_scenic = scenic;
			}

		}
	}
	
	println!("Total visible: {}", total_visible);
	println!("Max scenic: {}", max_scenic);

	Ok(())
}
