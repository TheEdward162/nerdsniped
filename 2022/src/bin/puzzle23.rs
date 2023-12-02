use std::{
	io::Read,
	fmt
};

use anyhow::Context;

use aoc_commons as aoc;
use aoc::{
	anyhow,
	log,
	geometry::{Point, Rectangle, Grid2}
};

type Point2 = Point<2>;
type Rectangle2 = Rectangle<2>;

#[derive(Debug, Clone, Copy)]
enum Cell {
	Empty,
	Elf,
	Considered,
	Blocked
}
impl fmt::Display for Cell {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Empty => write!(f, "."),
			Self::Elf => write!(f, "#"),
			Self::Considered => write!(f, "P"),
			Self::Blocked => write!(f, "X")
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Direction {
	North,
	South,
	West,
	East
}
impl Direction {
	pub fn next(&self) -> Self {
		match self {
			Self::North => Self::South,
			Self::South => Self::West,
			Self::West => Self::East,
			Self::East => Self::North
		}
	}

	pub fn next4(&self) -> [Self; 4] {
		let a = *self;
		let b = a.next();
		let c = b.next();
		let d = c.next();

		[a, b, c, d]
	}

	pub fn directions(&self) -> [Point2; 3] {
		match self {
			Self::North => [Point2::new(-1, -1), Point2::new(0, -1), Point2::new(1, -1)],
			Self::South => [Point2::new(-1, 1), Point2::new(0, 1), Point2::new(1, 1)],
			Self::West => [Point2::new(-1, -1), Point2::new(-1, 0), Point2::new(-1, 1)],
			Self::East => [Point2::new(1, -1), Point2::new(1, 0), Point2::new(1, 1)]
		}
	}
}

struct Elf {
	position: Point2,
	start_direction: Direction,
	proposed: Option<Point2>
}
impl Elf {
	pub fn nex(position: Point2) -> Self {
		Self { position, start_direction: Direction::North, proposed: None }
	}

	pub fn step_begin(&mut self, grid: &mut Grid2<Cell>) {
		const AROUND: [Point2; 8] = [
			Point2::new(1, 0),
			Point2::new(0, 1),
			Point2::new(-1, 0),
			Point2::new(0, -1),
			Point2::new(1, 1),
			Point2::new(1, -1),
			Point2::new(-1, 1),
			Point2::new(-1, -1)
		];
		let queue = self.start_direction.next4();
		self.start_direction = queue[1];

		if !AROUND.into_iter().any(|p| matches!(grid.get(self.position + p).unwrap(), Cell::Elf)) {
			return;
		}

		for d in queue {
			let points = d.directions();

			let any_blocked = points.into_iter().any(
				|p| matches!(grid.get(self.position + p).unwrap(), Cell::Elf)
			);

			if !any_blocked {
				let considered = points[1] + self.position;
				match grid.get_mut(considered).unwrap() {
					c @ Cell::Empty => {
						*c = Cell::Considered;
						self.proposed = Some(considered);
					}
					c @ Cell::Considered => { *c = Cell::Blocked; }
					_ => ()
				}

				break;
			}
		}		
	}

	pub fn step_end(&mut self, grid: &mut Grid2<Cell>) -> bool {
		if let Some(new_position) = self.proposed.take() {
			let new_position = match grid.get_mut(new_position).unwrap() {
				c @ Cell::Considered => {
					*c = Cell::Elf;
					Some(new_position)
				}
				c @ Cell::Blocked => {
					*c = Cell::Empty;
					None
				}
				_ => None
			};

			if let Some(new_position) = new_position {
				*grid.get_mut(self.position).unwrap() = Cell::Empty;
				self.position = new_position;

				return true;
			}
		}

		false
	}
}

fn main() -> anyhow::Result<()> {
	let mut file = aoc::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let (mut grid, mut elves) = {
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
			Cell::Empty,
			Rectangle2 {
				min: Point2::new(-(width as isize), -(height as isize)),
				max: Point2::new((width * 2) as isize, (height * 2) as isize)
			}
		)?;
		log::debug!("Grid bounds: {}", grid.bounding_box());

		let mut elves = Vec::new();
		for (y, line) in input.lines().filter(|s| !s.is_empty()).enumerate() {
			for (x, ch) in line.chars().enumerate() {
				match ch {
					'.' => (),
					'#' => {
						let elf = Elf::nex(Point2::new(x as isize, y as isize));
						*grid.get_mut(elf.position).unwrap() = Cell::Elf;
						elves.push(elf);
					}
					_ => anyhow::bail!("Invalid input")
				}
			}
		}

		(grid, elves)
	};
	log::trace!("Start map:\n{}", grid);

	let mut grid1 = grid.clone();
	let mut round = 0;
	loop {
		round += 1;
		log::debug!("Round: {}", round);

		for elf in elves.iter_mut() {
			elf.step_begin(&mut grid);
		}
		log::trace!("Map:\n{}", grid);
		let mut any_moved = false;
		for elf in elves.iter_mut() {
			any_moved |= elf.step_end(&mut grid);
		}
		log::trace!("Map:\n{}", grid);

		if round == 10 {
			grid1 = grid.clone();
			log::info!("part1");
		}

		if !any_moved {
			break;
		}
	}

	let part1 = {
		let mut min_bb = Rectangle2 {
			min: Point2::MAX,
			max: Point2::MIN
		};
		for y in grid1.y_range() {
			for x in grid1.x_range() {
				let point = Point2::new(x, y);
				if let Cell::Elf = grid1.get(point).unwrap() {
					min_bb.min = min_bb.min.min(point);
					min_bb.max = min_bb.max.max(point);
				}
			}
		}
		min_bb.max = min_bb.max + Point2::new(1, 1);
		log::debug!("Minimum bb: {}", min_bb);
		
		let mut empty_tiles = 0;
		for point in min_bb.points_iter() {
			if let Cell::Empty = grid1.get(point).unwrap() {
				empty_tiles += 1;
			}
		}

		empty_tiles
	};

	println!("Empty tiles: {}", part1);
	println!("Settled after: {}", round);
	log::info!("Done");

	Ok(())
}
