use std::{
	io::Read,
	fmt, collections::VecDeque
};

use anyhow::Context;

use aoc_commons as base;
use base::{
	anyhow,
	log,
	geometry::{Point, Rectangle, Grid2}
};

type Point2 = Point<2>;
type Rectangle2 = Rectangle<2>;

#[derive(Debug, Clone, Copy)]
enum Cell {
	Ground([Option<Direction>; 4]),
	Wall,
	Expedition
}
impl Cell {
	pub const EMPTY: Self = Self::Ground([None, None, None, None]);

	pub fn reachable(&self) -> bool {
		match self {
			Self::Ground([None, None, None, None]) => true,
			_ => false
		}
	}

	pub fn blizzard_count(&self) -> usize {
		match self {
			Self::Ground([a, b, c, d]) => a.is_some() as usize + b.is_some() as usize + c.is_some() as usize + d.is_some() as usize,
			_ => 0
		}
	}
}
impl TryFrom<char> for Cell {
	type Error = anyhow::Error;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		let me = match value {
			'.' => Self::Ground([None; 4]),
			'#' => Self::Wall,
			'E' => Self::Expedition,
			c @ ('<' | '>' | '^' | 'v') => {
				let direction = Direction::try_from(c)?;
				let mut blizzards = [None; 4];
				blizzards[direction.index()] = Some(direction);

				Self::Ground(blizzards)
			}
			_ => anyhow::bail!("Invalid Cell char")
		};

		Ok(me)
	}
}
impl fmt::Display for Cell {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Ground([None, None, None, None]) => write!(f, "."),
			Self::Wall => write!(f, "#"),
			Self::Expedition => write!(f, "E"),
			Self::Ground(blizzards) => {
				let count = self.blizzard_count();
				if count > 1 {
					write!(f, "{}", count)
				} else {
					match blizzards {
						[Some(d), None, None, None]
						| [None, Some(d), None, None]
						| [None, None, Some(d), None]
						| [None, None, None, Some(d)]
							=> write!(f, "{}", d),
						_ => unreachable!()
					}
				}
			}
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Direction {
	Up,
	Down,
	Left,
	Right
}
impl Direction {
	pub fn index(&self) -> usize {
		match self {
			Self::Up => 0,
			Self::Down => 1,
			Self::Left => 2,
			Self::Right => 3
		}
	}

	pub fn direction(&self) -> Point2 {
		match self {
			Self::Up => Point2::new(0, -1),
			Self::Down => Point2::new(0, 1),
			Self::Left => Point2::new(-1, 0),
			Self::Right => Point2::new(1, 0)
		}
	}
}
impl TryFrom<char> for Direction {
	type Error = anyhow::Error;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		let me = match value {
			'^' => Self::Up,
			'v' => Self::Down,
			'<' => Self::Left,
			'>' => Self::Right,
			_ => anyhow::bail!("Invalid direction char")
		};

		Ok(me)
	}
}
impl fmt::Display for Direction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Up => write!(f, "^"),
			Self::Down => write!(f, "v"),
			Self::Left => write!(f, "<"),
			Self::Right => write!(f, ">")
		}
	}
}

fn next_wrap(grid: &Grid2<Cell>, position: Point2, direction: Direction) -> Point2 {
	let check_position = position + direction.direction();

	match grid.get(check_position).unwrap() {
		Cell::Wall => grid.bounding_box().wrap_around(
			check_position + direction.direction()
		) + direction.direction(),
		_ => check_position
	}
}

struct Blizzard {
	position: Point2,
	direction: Direction
}
impl Blizzard {
	pub fn update(&mut self, grid: &Grid2<Cell>) {
		self.position = next_wrap(grid, self.position, self.direction);
	}

	pub fn project(&mut self, grid: &mut Grid2<Cell>) {
		match grid.get_mut(self.position).unwrap() {
			Cell::Ground(blizzards) => {
				blizzards[self.direction.index()] = Some(self.direction);
			}
			_ => unreachable!()
		}
	}

	pub fn unproject(&mut self, grid: &mut Grid2<Cell>) {
		match grid.get_mut(self.position).unwrap() {
			Cell::Ground(blizzards) => {
				blizzards[self.direction.index()] = None;
			}
			_ => unreachable!()
		}
	}
}

fn update_blizzards(grid: &mut Grid2<Cell>, blizzards: &mut [Blizzard]) {
	for blizzard in blizzards.iter_mut() {
		blizzard.unproject(grid);
		blizzard.update(grid);
	}

	for blizzard in blizzards.iter_mut() {
		blizzard.project(grid);
	}
}

struct SwapQueue<T> {
	from: VecDeque<T>,
	to: VecDeque<T>
}
impl<T> SwapQueue<T> {
	pub fn new() -> Self {
		Self {
			from: VecDeque::new(),
			to: VecDeque::new()
		}
	}

	pub fn push(&mut self, value: T) {
		self.to.push_back(value);
	}

	pub fn pop(&mut self) -> Option<T> {
		self.from.pop_front()
	}

	pub fn swap(&mut self) {
		std::mem::swap(&mut self.from, &mut self.to);
	}

	pub fn len(&self) -> usize {
		self.from.len()
	}

	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.from.iter()
	}
}

fn find_path(grid: &mut Grid2<Cell>, blizzards: &mut [Blizzard], start: Point2, end: Point2) -> anyhow::Result<usize> {
	let mut round = 0;

	let mut search_queue = SwapQueue::<Point2>::new();
	search_queue.push(start);

	loop {
		round += 1;
		log::trace!("Round: {}, active: {}", round, search_queue.len());

		let mut at_end = false;
		for e in search_queue.iter() {
			*grid.get_mut(*e).context("Invalid expedition position")? = Cell::EMPTY;

			if *e == end {
				at_end = true;
			}
		}
		if at_end {
			break;
		}

		update_blizzards(grid, blizzards);
		while let Some(expedition) = search_queue.pop() {
			match grid.get_mut(expedition).context("Invalid expedition position")? {
				c if c.reachable() => {
					search_queue.push(expedition);
					*c = Cell::Expedition;
				}
				_ => ()
			}

			for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
				let point = expedition + direction.direction();
				if grid.get(point).map(|c| c.reachable()).unwrap_or(false) {
					search_queue.push(point);
					*grid.get_mut(point).context("Invalid reachable position")? = Cell::Expedition;
				}
			}
		}
		search_queue.swap();

		log::trace!("Map:\n{}", grid);
	}

	Ok(round - 1)
}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let (mut grid, mut blizzards, start, end) = {
		let mut width = 0;
		let mut height = 0;
		for (i, line) in input.lines().enumerate() {
			if line.is_empty() {
				break;
			}

			width = width.max(line.len());
			height = i + 1;
		}

		let mut grid = Grid2::<Cell>::new_fill(
			Cell::EMPTY,
			Rectangle2 {
				min: Point2::new(0, 0),
				max: Point2::new(width as isize, height as isize)
			}
		)?;
		log::debug!("Grid bounds: {}", grid.bounding_box());
		let mut blizzards = Vec::new();

		for (y, line) in input.lines().filter(|s| !s.is_empty()).enumerate() {
			for (x, ch) in line.chars().enumerate() {
				let point = Point2::new(x as isize, y as isize);
				let cell = Cell::try_from(ch)?;
				
				match cell {
					Cell::Ground(
						[Some(d), None, None, None]
						| [None, Some(d), None, None]
						| [None, None, Some(d), None]
						| [None, None, None, Some(d)]
					) => {
						blizzards.push(Blizzard { position: point, direction: d });
					}
					_ => ()
				}

				*grid.get_mut(point).unwrap() = cell;
			}
		}

		let start = Point2::new(1, 0);
		let end = Point2::new(grid.bounding_box().size().x() - 2, grid.bounding_box().size().y() - 1);
		*grid.get_mut(start).unwrap() = Cell::Expedition;

		(grid, blizzards, start, end)
	};
	log::trace!("Start map:\n{}", grid);

	let to_end = find_path(&mut grid, &mut blizzards, start, end)?;
	log::info!("to_end: {}", to_end);
	let to_beginning = find_path(&mut grid, &mut blizzards, end, start)?;
	log::info!("to_beginning: {}", to_beginning);
	let to_end_again = find_path(&mut grid, &mut blizzards, start, end)?;
	log::info!("to_end_again: {}", to_end_again);
	
	println!("To end: {}", to_end);
	println!("Another one: {}", to_end + to_beginning + to_end_again);
	log::info!("Done");

	Ok(())
}
