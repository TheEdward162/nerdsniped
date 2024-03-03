use std::{io::Read, collections::HashSet, ops};

use anyhow::Context;

use aoc_commons as aoc;
use aoc::{anyhow, log};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
	pub x: isize,
	pub y: isize
}
impl Vec2 {
	pub const UP: Self = Self { x: 0, y: -1 };
	pub const DOWN: Self = Self { x: 0, y: 1 };
	pub const LEFT: Self = Self { x: -1, y: 0 };
	pub const RIGHT: Self = Self { x: 1, y: 0 };

	pub fn length_diag(&self) -> isize {
		self.x.abs().max(self.y.abs())
	}
}
impl ops::Add<Self> for Vec2 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}
impl ops::AddAssign for Vec2 {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}
impl ops::Sub<Self> for Vec2 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}
impl ops::Div<isize> for Vec2 {
	type Output = Self;

	fn div(self, rhs: isize) -> Self::Output {
		Self { x: self.x / rhs, y: self.y / rhs } 
	}
}

struct Rope<const N: usize> {
	nodes: [Vec2; N]
}
impl<const N: usize> Rope<N> {
	pub fn new() -> Self {
		Self {
			nodes: [Vec2 { x: 0, y: 0 }; N]
		}
	}

	fn propagate(&mut self, from: usize, to: usize) -> anyhow::Result<()> {
		let diff = self.nodes[from] - self.nodes[to];
		let length = diff.length_diag();
		anyhow::ensure!(length <= 2, "Invalid state");

		log::trace!("from: {:?}, to: {:?}, diff: {:?}, length: {}", self.nodes[from], self.nodes[to], diff, length);

		if length == 2 {
			if diff.x == 0 || diff.y == 0 {
				self.nodes[to] += diff / 2;
			} else {
				self.nodes[to] += Vec2 { x: diff.x.signum(), y: diff.y.signum() };
			}
		}

		log::trace!("new to: {:?}", self.nodes[to]);

		Ok(())
	}

	pub fn move_head(&mut self, shift: Vec2) -> anyhow::Result<()> {
		self.nodes[0] += shift;
		
		for i in 1 .. N {
			self.propagate(i - 1, i)?;
		}
		
		Ok(())
	}

	pub fn link_pos(&self, i: usize) -> Option<Vec2> {
		if i >= N {
			None
		} else {
			Some(self.nodes[i])
		}
	}

	pub fn tail_pos(&self) -> Vec2 {
		self.nodes[N - 1]
	}
}

fn main() -> anyhow::Result<()> {
	let mut file = aoc::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut rope = Rope::<10>::new();
	
	let mut unique_tail1_positions: HashSet<Vec2, _> = HashSet::new();
	let mut unique_tail9_positions: HashSet<Vec2, _> = HashSet::new();
	unique_tail1_positions.insert(rope.link_pos(1).unwrap());
	unique_tail9_positions.insert(rope.tail_pos());

	for line in input.split("\n").filter(|s| !s.is_empty()) {
		let (direction, steps_str) = line.split_once(' ').context("Failed to split line by \\n")?;

		let shift = match direction {
			"U" => Vec2::UP,
			"D" => Vec2::DOWN,
			"L" => Vec2::LEFT,
			"R" => Vec2::RIGHT,
			_ => anyhow::bail!("Invalid direction")
		};

		let steps: usize = steps_str.parse().context("Invalid number of steps")?;
		for _ in 0 .. steps {
			rope.move_head(shift)?;
			unique_tail1_positions.insert(rope.link_pos(1).unwrap());
			unique_tail9_positions.insert(rope.tail_pos());
		}
	}

	println!("Tail1 unique: {}", unique_tail1_positions.len());
	println!("Tail9 unique: {}", unique_tail9_positions.len());

	Ok(())
}
