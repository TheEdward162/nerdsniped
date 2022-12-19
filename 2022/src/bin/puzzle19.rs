use std::{
	fmt,
	io::Read,
	ops::{Add, Index, AddAssign, Mul, MulAssign, IndexMut, Sub, SubAssign}
};

use anyhow::Context;

use aoc_commons as base;
use base::{anyhow, log};

type Time = usize;

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Kind {
	Ore = 0,
	Clay = 1,
	Obsidian = 2,
	Geode = 3
}

#[derive(Debug, Clone, Copy)]
struct Minerals([usize; 4]);
impl Minerals {
	pub const ZERO: Self = Self([0; 4]);

	pub fn new(v: [usize; 4]) -> Self {
		Self(v)
	}

	pub fn checked_sub(&self, rhs: &Self) -> Option<Self> {
		match (
			self.0[0].checked_sub(rhs.0[0]),
			self.0[1].checked_sub(rhs.0[1]),
			self.0[2].checked_sub(rhs.0[2]),
			self.0[3].checked_sub(rhs.0[3])
		) {
			(Some(a), Some(b), Some(c), Some(d)) => Some(Self([a, b, c, d])),
			_ => None
		}
	}

	pub fn div_by_rate(self, rhs: Self) -> Option<Self> {
		fn div(a: usize, b: usize) -> Option<usize> {
			if a == 0 {
				return Some(0);
			}

			(a + b - 1).checked_div(b)
		}

		match (
			div(self.0[0], rhs.0[0]),
			div(self.0[1], rhs.0[1]),
			div(self.0[2], rhs.0[2]),
			div(self.0[3], rhs.0[3])
		) {
			(Some(a), Some(b), Some(c), Some(d)) => Some(Self([a, b, c, d])),
			_ => None
		}
	}

	pub fn saturating_sub(self, rhs: Self) -> Self {
		Self([
			self.0[0].saturating_sub(rhs.0[0]),
			self.0[1].saturating_sub(rhs.0[1]),
			self.0[2].saturating_sub(rhs.0[2]),
			self.0[3].saturating_sub(rhs.0[3])
		])
	}
}
impl Index<Kind> for Minerals {
	type Output = usize;

	fn index(&self, index: Kind) -> &Self::Output {
		&self.0[index as usize]
	}
}
impl IndexMut<Kind> for Minerals {
	fn index_mut(&mut self, index: Kind) -> &mut Self::Output {
		&mut self.0[index as usize]
	}
}
impl Add<Self> for Minerals {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let mut me = Self(self.0);
		me += rhs;
		me
	}
}
impl AddAssign<Self> for Minerals {
	fn add_assign(&mut self, rhs: Self) {
		self.0[0] += rhs.0[0];
		self.0[1] += rhs.0[1];
		self.0[2] += rhs.0[2];
		self.0[3] += rhs.0[3];
	}
}
impl Sub<Self> for Minerals {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		let mut me = Self(self.0);
		me -= rhs;
		me
	}
}
impl SubAssign<Self> for Minerals {
	fn sub_assign(&mut self, rhs: Self) {
		self.0[0] -= rhs.0[0];
		self.0[1] -= rhs.0[1];
		self.0[2] -= rhs.0[2];
		self.0[3] -= rhs.0[3];
	}
}
impl Mul<usize> for Minerals {
	type Output = Self;

	fn mul(self, rhs: usize) -> Self::Output {
		let mut me = Self(self.0);
		me *= rhs;
		me
	}
}
impl MulAssign<usize> for Minerals {
    fn mul_assign(&mut self, rhs: usize) {
        self.0[0] *= rhs;
		self.0[1] *= rhs;
		self.0[2] *= rhs;
		self.0[3] *= rhs;
    }
}
impl fmt::Display for Minerals {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

#[derive(Clone)]
struct RobotBlueprint {
	time: Time,
	cost: Minerals,
	production: Minerals
}
impl fmt::Display for RobotBlueprint {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({} in {} -> {})", self.cost, self.time, self.production)
	}
}

#[derive(Clone)]
struct Blueprint {
	id: usize,
	robots: [RobotBlueprint; 4]
}
impl Index<Kind> for Blueprint {
	type Output = RobotBlueprint;

	fn index(&self, index: Kind) -> &Self::Output {
		&self.robots[index as usize]
	}
}
impl fmt::Display for Blueprint {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f, "Blueprint {}:\n  {:?}{}\n  {:?}{}\n  {:?}{}\n  {:?}{}",
			self.id,
			Kind::Ore, self.robots[Kind::Ore as usize],
			Kind::Clay, self.robots[Kind::Clay as usize],
			Kind::Obsidian, self.robots[Kind::Obsidian as usize],
			Kind::Geode, self.robots[Kind::Geode as usize]
		)
	}
}

struct Factory {
	blueprint: Blueprint,
	max_time: Time,
	best: SearchValue,
	total_explored: usize
}
struct SearchValue {
	time: Time,
	minerals: Minerals,
	robots: Minerals
}
impl SearchValue {
	pub fn until(&self, target: Minerals) -> Option<Self> {
		target.saturating_sub(self.minerals).div_by_rate(self.robots).map(
			|required| {
				let elapsed = required[Kind::Ore].max(required[Kind::Clay]).max(required[Kind::Obsidian]).max(required[Kind::Geode]);

				Self {
					time: self.time + elapsed,
					minerals: self.minerals + self.robots * elapsed,
					robots: self.robots
				}
			}
		)
	}
}
impl fmt::Debug for SearchValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {		
		write!(f, "{}+{}@{}", self.minerals, self.robots, self.time)
	}
}
impl Factory {
	pub fn new(blueprint: Blueprint, max_time: Time) -> Self {
		Self {
			blueprint,
			max_time,
			best: SearchValue { time: 0, minerals: Minerals::ZERO, robots: Minerals::ZERO },
			total_explored: 0
		}
	}

	pub fn simulate(&mut self, start_minerals: Minerals, start_robots: Minerals) -> usize {
		let start = SearchValue { time: 0, minerals: start_minerals, robots: start_robots };
		self.explore(start);
		
		self.best.minerals[Kind::Geode]
	}

	fn until_build(&self, start: &SearchValue, kind: Kind) -> Option<SearchValue> {
		let blueprint = &self.blueprint[kind];
		match start.until(blueprint.cost) {
			None => None,
			Some(mut until) => {
				until.time += blueprint.time;
				until.minerals = until.minerals.checked_sub(&blueprint.cost).unwrap() + until.robots * blueprint.time;
				until.robots += blueprint.production;

				Some(until)
			}
		}
	}

	fn definitely_better(&self, left: &SearchValue, right: &SearchValue) -> bool {
		let result = if right.time >= left.time && left.minerals[Kind::Geode] > right.minerals[Kind::Geode] {
			true
		} else {
			let left_projection = left.minerals[Kind::Geode] + left.robots[Kind::Geode] * (self.max_time - left.time);

			let x = right.minerals.checked_sub(&self.blueprint[Kind::Geode].cost).map(|_| 1).unwrap_or(2);
			let right_projection = right.minerals[Kind::Geode] + right.robots[Kind::Geode] * (self.max_time - right.time) + (self.max_time - right.time).saturating_sub(x).pow(2);

			left_projection > right_projection
		};

		log::trace!("{:?} vs {:?} = {}", left, right, result);
		result
	}

	fn explore(&mut self, start: SearchValue) {
		if self.total_explored % 10000000 == 0 {
			log::debug!("explore({:?})#{}", start, self.total_explored);
		} else {
			log::trace!("explore({:?})#{}", start, self.total_explored);
		}
		self.total_explored += 1;
		
		if start.time > self.max_time {
			log::trace!("Over time");
			return;
		}

		if self.definitely_better(&self.best, &start) {
			log::trace!("Culled");
			return;
		}

		if let Some(until) = self.until_build(&start, Kind::Geode) {
			self.explore(until);
		}
		if let Some(until) = self.until_build(&start, Kind::Obsidian) {
			self.explore(until);
		}
		if let Some(until) = self.until_build(&start, Kind::Clay) {
			self.explore(until);
		}
		if let Some(until) = self.until_build(&start, Kind::Ore) {
			self.explore(until);
		}

		let until_end = self.max_time - start.time;
		let until_end = SearchValue {
			time: self.max_time,
			minerals: start.minerals + start.robots * until_end,
			robots: start.robots
		};
		if until_end.minerals[Kind::Geode] > self.best.minerals[Kind::Geode] {
			log::debug!("New best: {:?}", until_end);
			self.best = until_end;
		}
	}
}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut blueprints = Vec::<Blueprint>::new();
	for line in input.lines().filter(|s| !s.is_empty()) {
		type Num = base::macros::FromStrToTryFromAdapter<usize>;

		let (id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) = base::match_tokens!(
			line.split([' ', ':', '.']).filter(|s| !s.is_empty());
			"Blueprint", id: Num {.0},
			"Each", "ore", "robot", "costs", ore_ore: Num {.0}, "ore",
			"Each", "clay", "robot", "costs", clay_ore: Num {.0}, "ore",
			"Each", "obsidian", "robot", "costs", obsidian_ore: Num {.0}, "ore", "and", obsidian_clay: Num {.0}, "clay",
			"Each", "geode", "robot", "costs", geode_ore: Num {.0}, "ore", "and", geode_obsidian: Num {.0}, "obsidian",
		)?;

		blueprints.push(
			Blueprint {
				id,
				robots: [
					RobotBlueprint { time: 1, cost: Minerals::new([ore_ore, 0, 0, 0]), production: Minerals::new([1, 0, 0, 0]) },
					RobotBlueprint { time: 1, cost: Minerals::new([clay_ore, 0, 0, 0]), production: Minerals::new([0, 1, 0, 0]) },
					RobotBlueprint { time: 1, cost: Minerals::new([obsidian_ore, obsidian_clay, 0, 0]), production: Minerals::new([0, 0, 1, 0]) },
					RobotBlueprint { time: 1, cost: Minerals::new([geode_ore, 0, geode_obsidian, 0]), production: Minerals::new([0, 0, 0, 1]) }
				]
			}
		);
	}

	let mut quality_score = 0;
	for blueprint in blueprints.clone().into_iter() {
		let blueprint_id = blueprint.id;
		
		let mut factory = Factory::new(blueprint, 24);
		let geodes = factory.simulate(Minerals::ZERO, Minerals::new([1, 0, 0, 0]));

		log::info!("Blueprint: {} = {}", blueprint_id, geodes);

		quality_score += blueprint_id * geodes;
	}
	println!("Total quality: {}", quality_score);

	let mut geodes_product = 1;
	for blueprint in blueprints.into_iter().take(3) {
		let blueprint_id = blueprint.id;
		
		let mut factory = Factory::new(blueprint, 32);
		let geodes = factory.simulate(Minerals::ZERO, Minerals::new([1, 0, 0, 0]));

		log::info!("Blueprint: {} = {}", blueprint_id, geodes);

		geodes_product *= geodes;
	}
	println!("Geodes product: {}", geodes_product);

	log::info!("Done");

	Ok(())
}
