use std::{fmt, io::Read};

use anyhow::Context;

use aoc_commons as base;
use base::{anyhow, log};

use base::geometry::Point;

type Point2 = Point<2>;

#[derive(Debug)]
struct Intervals {
	intervals: Vec<[isize; 2]>
}
impl Intervals {
	pub fn new() -> Self {
		Self { intervals: vec![[0, 1]] }
	}

	pub fn add(&mut self, interval: [isize; 2]) {
		for i in 0 .. self.intervals.len() {
			if interval[0] <= self.intervals[i][1] + 1 && interval[1] >= self.intervals[i][0] {
				self.intervals[i][0] = self.intervals[i][0].min(interval[0]);
				self.intervals[i][1] = self.intervals[i][1].max(interval[1]);

				return;
			}
		}
		
		self.intervals.push(interval);
	}

	pub fn contains(&self, v: isize) -> bool {
		for interval in self.intervals.iter().rev() {
			if v >= interval[0] && v < interval[1] {
				return true;
			}
		}

		false
	}

	pub fn lowest(&self) -> isize {
		self.intervals.iter().map(|[l, _]| *l).min().unwrap()
	}
}

#[derive(Debug)]
struct Heights {
	columns: [Intervals; Self::WIDTH]
}
impl Heights {
	const WIDTH: usize = 7;

	pub fn new() -> Self {
		Self { columns: [(); Self::WIDTH].map(|_| Intervals::new()) }
	}

	pub fn shift(&mut self, rock: &mut RockFormation, shift: RockShift) -> bool {
		let shift_point = shift.shift();

		let mut to_move = true;
		for point in rock.points_iter() {
			let point = point + shift_point;

			if point.x() < 0 || point.x() >= Self::WIDTH as isize {
				to_move = false;
				break;
			}

			if self.columns[point.x() as usize].contains(point.y()) {
				to_move = false;
				break;
			}
		}
		
		if to_move {
			rock.position = rock.position + shift_point;
			log::trace!("Shifted: {} (shift: {})", rock, shift_point);

			false
		} else if matches!(shift, RockShift::Down) {
			let mut intervals = [None::<[isize; 2]>; Self::WIDTH];
			for point in rock.points_iter() {
				let interval = &mut intervals[point.x() as usize];

				*interval = match interval {
					None => Some([point.y(), point.y() + 1]),
					Some([l, h]) => Some([(*l).min(point.y()), (*h).max(point.y() + 1)])
				};
			}
			for i in 0 .. Self::WIDTH {
				if let Some(interval) = intervals[i] {
					self.columns[i].add(interval);
				}
			}
			log::trace!("At rest: {}", rock);

			true
		} else {
			log::trace!("Blocked: {} (shift: {})", rock, shift_point);
			false
		}
	}

	fn height_profile(&self) -> [isize; Self::WIDTH] {
		let mut result = [0isize; Self::WIDTH];

		for i in 0 .. Self::WIDTH {
			result[i] = self.columns[i].lowest();
		}

		result
	}

	fn all_same(&self) -> Option<isize> {
		let profile = self.height_profile();
		if profile.iter().all(|&h| h == profile[0]) {
			Some(profile[0])
		} else {
			None
		}
	}

	pub fn lowest(&self) -> isize {
		self.height_profile().into_iter().min().unwrap()
	}
}
impl fmt::Display for Heights {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut heights = [0; Self::WIDTH];
		for (i, height) in self.columns.iter().enumerate() {
			heights[i] = height.lowest();
		}
		
		write!(f, "{:?}", heights)
	}
}

#[derive(Debug, Clone, Copy)]
enum RockShift {
	Left,
	Right,
	Down
}
impl RockShift {
	pub fn shift(&self) -> Point2 {
		match self {
			Self::Left => Point2::new(-1, 0),
			Self::Right => Point2::new(1, 0),
			Self::Down => Point2::new(0, 1)
		}
	}
}
impl TryFrom<char> for RockShift {
	type Error = anyhow::Error;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'<' => Ok(Self::Left),
			'>' => Ok(Self::Right),
			c => Err(anyhow::anyhow!("Invalid char '{}'", c))
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Shape {
	Hline,
	Plus,
	RevL,
	VLine,
	Square
}
impl Shape {
	const SHAPE_HLINE: &'static [Point2] = &[
		Point2::new(0, 0), Point2::new(1, 0), Point2::new(2, 0), Point2::new(3, 0)
	];
	const SHAPE_PLUS: &'static [Point2] = &[
		Point2::new(1, 0),
		Point2::new(0, 1), Point2::new(1, 1), Point2::new(2, 1),
		Point2::new(1, 2)
	];
	const SHAPE_REVL: &'static [Point2] = &[
		Point2::new(2, 0),
		Point2::new(2, 1),
		Point2::new(0, 2), Point2::new(1, 2), Point2::new(2, 2)
	];
	const SHAPE_VLINE: &'static [Point2] = &[
		Point2::new(0, 0),
		Point2::new(0, 1),
		Point2::new(0, 2),
		Point2::new(0, 3)
	];
	const SHAPE_SQUARE: &'static [Point2] = &[
		Point2::new(0, 0), Point2::new(1, 0),
		Point2::new(0, 1), Point2::new(1, 1)
	];

	pub fn next(self) -> Self {
		match self {
			Self::Hline => Self::Plus,
			Self::Plus => Self::RevL,
			Self::RevL => Self::VLine,
			Self::VLine => Self::Square,
			Self::Square => Self::Hline
		}
	}

	pub fn points_iter(&self) -> impl Iterator<Item = Point2> {
		let points = match self {
			Self::Hline => Self::SHAPE_HLINE,
			Self::Plus => Self::SHAPE_PLUS,
			Self::RevL => Self::SHAPE_REVL,
			Self::VLine => Self::SHAPE_VLINE,
			Self::Square => Self::SHAPE_SQUARE
		};

		points.into_iter().copied()
	}

	pub fn height(&self) -> isize {
		self.points_iter().fold(isize::MIN, |acc, p| acc.max(p.y())) + 1
	}
}

#[derive(Debug)]
struct RockFormation {
	position: Point2,
	shape: Shape
}
impl RockFormation {
	pub fn points_iter(&self) -> impl Iterator<Item = Point2> {
		let pos = self.position;
		self.shape.points_iter().map(move |p| p + pos)
	}
}
impl fmt::Display for RockFormation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}@{}", self.shape, self.position)
	}
}

fn simulate(
	streams: &[RockShift],
	rounds: usize,
	mut jet_i: usize,
	mut current_shape: Shape,
	periods: &mut Vec<(usize, Shape, usize, isize)>
) -> anyhow::Result<usize> {
	let mut heights = Heights::new();
	log::debug!("Heights: {}", heights);

	for round in 0 .. rounds {
		if let Some(height) = heights.all_same() {
			if round > 0 {
				periods.push((jet_i, current_shape, round, height));
			}
		}

		let mut current = RockFormation {
			shape: current_shape,
			position: Point2::new(2, heights.lowest() - 3 - current_shape.height())
		};
		current_shape = current_shape.next();
		log::debug!("Spawned: {}", current);

		loop {
			let stream = streams[jet_i];
			jet_i = (jet_i + 1) % streams.len();
			heights.shift(&mut current, stream);

			if heights.shift(&mut current, RockShift::Down) {
				break;
			}
		}
	}

	log::info!("total intervals: {}", heights.columns.iter().map(|i| i.intervals.len()).sum::<usize>());

	Ok(heights.lowest().abs() as usize)
}

const MAX_ROUNDS: usize = 2022;
const MAX_ROUNDS2: usize = 1000000000000;

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut streams = Vec::<RockShift>::new();
	for ch in input.chars().filter(|c| !c.is_whitespace()) {
		streams.push(RockShift::try_from(ch).context("Invalid input")?);
	}

	let mut periods = Vec::new();

	let part1 = simulate(
		&streams,
		MAX_ROUNDS,
		0,
		Shape::Hline,
		&mut periods
	)?;
	if periods.len() < 2 {
		anyhow::bail!("Not enough periods :(");
	}
	log::info!("Periods: {:?}", periods);
	
	let start_round = periods[0].2;
	let start_height = periods[0].3;
	let period_rounds = periods[1].2 - start_round;
	let period_height = periods[1].3 - start_height;
	let skipped_periods = (MAX_ROUNDS2 - start_round) / period_rounds;
	let skipped_rounds = skipped_periods * period_rounds;

	let part2 = simulate(
		&streams,
		MAX_ROUNDS2 - skipped_rounds - start_round,
		periods[0].0,
		periods[0].1,
		&mut periods
	)?;

	println!("Tower height: {}", part1);
	println!("Tower height 2: {}", (-start_height as usize) + (-period_height as usize) * skipped_periods + part2);

	log::info!("Done");

	Ok(())
}
