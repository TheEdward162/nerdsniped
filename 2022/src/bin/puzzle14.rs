use std::{io::Read, fmt};

use anyhow::Context;

use aoc_commons as base;
use base::{anyhow, log};

use base::{
	geometry::{Grid2, Point2, Rectangle2},
	macros::FromStrToTryFromAdapter
};

#[derive(Clone, Copy)]
enum Cell {
	Source,
	Air,
	Rock,
	Sand,
	Void
}
impl fmt::Display for Cell {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Source => write!(f, "+"),
			Self::Air => write!(f, "."),
			Self::Rock => write!(f, "#"),
			Self::Sand => write!(f, "o"),
			Self::Void => write!(f, "~")
		}
	}
}

fn map_fill_line(map: &mut Grid2<Cell>, from: Point2, to: Point2, fill: Cell) -> anyhow::Result<()> {
	let min = from.min(to);
	let max = from.max(to);

	for y in min.y ..= max.y {
		for x in min.x ..= max.x {
			*map.get_mut(Point2::new(x, y)).context("Invalid coordinates")? = fill;
		}
	}

	Ok(())
}

enum TickResult {
	Continue,
	Rest,
	Void
}
fn map_sand_tick(map: &mut Grid2<Cell>, cursor: &mut Point2) -> anyhow::Result<TickResult> {
	macro_rules! check {
		($x: expr, $y: expr) => {
			{
				let check_pos = *cursor + Point2::new($x, $y);
				match map.get_mut(check_pos) {
					Some(Cell::Source | Cell::Air) => {
						*cursor = check_pos;
						return Ok(TickResult::Continue);
					}
					Some(Cell::Void) => {
						return Ok(TickResult::Void)
					}
					Some(Cell::Rock | Cell::Sand) => (),
					None => {
						log::debug!("Map error:\n{}", map);
						anyhow::bail!("Invalid collision check coordinates {}", check_pos)
					}
				}
			}
		};
	}

	check!(0, 1);
	check!(-1, 1);
	check!(1, 1);

	*map.get_mut(*cursor).context("Cursor out of bounds")? = Cell::Sand;

	Ok(TickResult::Rest)
}

const SAND_SOURCE: Point2 = Point2::new(500, 0);

fn simulate(mut map: Grid2<Cell>) -> anyhow::Result<()> {
	log::debug!("Map start:\n{}", map);
	
	let mut cursor = SAND_SOURCE;
	let mut sand_at_rest = 0;
	loop {
		match map_sand_tick(&mut map, &mut cursor)? {
			TickResult::Continue => (),
			TickResult::Rest => {
				sand_at_rest += 1;

				if cursor == SAND_SOURCE {
					break
				}
				cursor = SAND_SOURCE;

				log::trace!("Map:\n{}", map);
			}
			TickResult::Void => break
		}
	}
	log::debug!("Map rest:\n{}", map);
	println!("Sand at rest: {}", sand_at_rest);

	Ok(())
}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut lines: Vec<Vec<Point2>> = Vec::new();
	for line in input.split('\n').filter(|s| !s.is_empty()) {
		let mut points = Vec::new();
		
		for point_str in line.split(" -> ") {
			let (x, y) = base::split_match_tokens!(point_str, ','; x: FromStrToTryFromAdapter<isize>, y: FromStrToTryFromAdapter<isize>)?;
			points.push(Point2::new(x.0, y.0));
		}

		anyhow::ensure!(points.len() > 1, "Line has less than 2 points");
		lines.push(points);
	}

	let bounding_box = lines.iter().flat_map(|p| p.iter()).fold(
		Rectangle2 { min: Point2::new(isize::MAX, 0), max: Point2::new(0, 0) }, |acc, p| Rectangle2 {
			min: Point2::new(acc.min.x.min(p.x), acc.min.y.min(p.y)),
			max: Point2::new(acc.max.x.max(p.x), acc.max.y.max(p.y))
		}
	);
	let bounding_box = Rectangle2 {
		min: bounding_box.min,
		max: bounding_box.max + Point2::new(1, 1),
	};

	// part1
	let b1 = Rectangle2 {
		min: bounding_box.min - Point2::new(1, 0),
		max: bounding_box.max + Point2::new(1, 1)
	};
	log::debug!("Boundaries: {:?}", b1);

	let mut map1 = Grid2::new_fill(Cell::Air, b1)?;
	*map1.get_mut(SAND_SOURCE).unwrap() = Cell::Source;
	for points in lines.iter() {
		for window in points.windows(2) {
			map_fill_line(&mut map1, window[0], window[1], Cell::Rock)?;
		}
	}
	map_fill_line(&mut map1, b1.min, Point2::new(b1.min.x, b1.max.y - 1), Cell::Void)?;
	map_fill_line(&mut map1, Point2::new(b1.max.x - 1, b1.min.y), Point2::new(b1.max.x - 1, b1.max.y - 1), Cell::Void)?;
	map_fill_line(&mut map1, Point2::new(b1.min.x, b1.max.y - 1), Point2::new(b1.max.x - 1, b1.max.y - 1), Cell::Void)?;
	simulate(map1)?;

	// part 2
	let b2 = Rectangle2 {
		min: bounding_box.min - Point2::new(bounding_box.size().y, 0),
		max: bounding_box.max + Point2::new(bounding_box.size().y, 2)
	};
	log::debug!("Boundaries: {:?}", b2);

	let mut map2 = Grid2::new_fill(Cell::Air, b2)?;
	*map2.get_mut(SAND_SOURCE).unwrap() = Cell::Source;
	for points in lines.iter() {
		for window in points.windows(2) {
			map_fill_line(&mut map2, window[0], window[1], Cell::Rock)?;
		}
	}
	map_fill_line(&mut map2, Point2::new(b2.min.x, b2.max.y - 1), Point2::new(b2.max.x - 1, b2.max.y - 1), Cell::Rock)?;
	simulate(map2)?;

	Ok(())
}