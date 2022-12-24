use std::{
	io::Read,
	fmt
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
	Void,
	Ground,
	Rock,
	Cursor(Facing)
}
impl fmt::Display for Cell {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Void => write!(f, " "),
			Self::Ground => write!(f, "."),
			Self::Rock => write!(f, "#"),
			Self::Cursor(facing) => write!(f, "{}", facing)
		}
	}
}

#[derive(Debug)]
enum Command {
	Move(isize),
	RotateLeft,
	RotateRight
}
impl<'a> TryFrom<&'a str> for Command {
	type Error = anyhow::Error;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		let me = match value {
			"L" => Self::RotateLeft,
			"R" => Self::RotateRight,
			r => match r.parse::<isize>() {
				Ok(v) => Self::Move(v),
				Err(_) => anyhow::bail!("Failed to parse \"{}\" as Command", value)
			}
		};

		Ok(me)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
	Up,
	Right,
	Down,
	Left
}
type Transform = Facing;
impl Facing {
	pub fn direction(&self) -> Point2 {
		match self {
			Self::Up => Point2::new(0, -1),
			Self::Right => Point2::new(1, 0),
			Self::Down => Point2::new(0, 1),
			Self::Left => Point2::new(-1, 0)
		}
	}

	pub fn score(&self) -> usize {
		match self {
			Self::Up => 3,
			Self::Right => 0,
			Self::Down => 1,
			Self::Left => 2
		}
	}

	pub fn rotate(self, rhs: Self) -> Self {
		match (self, rhs) {
			(a, Self::Up) | (Self::Up, a) => a,
			(Self::Down, Self::Right) | (Self::Right, Self::Down) => Self::Left,
			(Self::Left, Self::Right) | (Self::Right, Self::Left) => Self::Up,
			(Self::Left, Self::Down) | (Self::Down, Self::Left) => Self::Right,
			(Self::Right, Self::Right) => Self::Down,
			(Self::Down, Self::Down) => Self::Up,
			(Self::Left, Self::Left) => Self::Down
		}
	}

	pub fn inverse(self) -> Self {
		match self {
			Self::Up => Self::Up,
			Self::Down => Self::Down,
			Self::Left => Self::Right,
			Self::Right => Self::Left
		}
	}
}
impl fmt::Display for Facing {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Up => write!(f, "^"),
			Self::Right => write!(f, ">"),
			Self::Down => write!(f, "v"),
			Self::Left => write!(f, "<")
		}
	}
}

fn row_non_void(grid: &Grid2<Cell>, y: isize) -> anyhow::Result<Rectangle2> {
	let mut start = isize::MAX;
	let mut end = isize::MIN;

	for x in grid.x_range() {
		match grid.get(Point2::new(x, y)).context("Invalid position")? {
			Cell::Void => (),
			_ => {
				start = start.min(x);
				end = end.max(x);
			}
		}
	}

	Ok(Rectangle2 { min: Point2::new(start, y), max: Point2::new(end + 1, y + 1) })
}

fn column_non_void(grid: &Grid2<Cell>, x: isize) -> anyhow::Result<Rectangle2> {
	let mut start = isize::MAX;
	let mut end = isize::MIN;

	for y in grid.y_range() {
		match grid.get(Point2::new(x, y)).context("Invalid position")? {
			Cell::Void => (),
			_ => {
				start = start.min(y);
				end = end.max(y);
			}
		}
	}

	Ok(Rectangle2 { min: Point2::new(x, start), max: Point2::new(x + 1, end + 1) })
}

fn simulate_2d(grid: &mut Grid2<Cell>, commands: &[Command], start: Point2) -> anyhow::Result<(Point2, Facing)> {
	let mut position = start;
	let mut facing = Facing::Right;
	for command in commands {
		match command {
			Command::RotateLeft => { facing = facing.rotate(Facing::Left); },
			Command::RotateRight => { facing = facing.rotate(Facing::Right); },
			Command::Move(steps) => {
				let boundaries = match facing {
					Facing::Left | Facing::Right => row_non_void(&grid, position.y()),
					Facing::Up | Facing::Down => column_non_void(&grid, position.x())
				}?;
				let direction = facing.direction();

				for _ in 0 .. *steps {
					let new_position = boundaries.wrap_around(position + direction);
					match grid.get(new_position).context("Invalid new position")? {
						Cell::Ground => {
							position = new_position;

							if log::log_enabled!(log::Level::Trace) {
								*grid.get_mut(position).unwrap() = Cell::Cursor(facing);
								log::trace!("Map:\n{}", grid);
								*grid.get_mut(position).unwrap() = Cell::Ground;
							}
						}
						Cell::Rock => (),
						Cell::Void | Cell::Cursor(_) => unreachable!()
					};
				}
			}
		}
	}
	log::debug!("Final position: {}, facing: {:?}", position, facing);

	Ok((position, facing))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CubeSideKind {
	Top,
	Bottom,
	Left,
	Right,
	Far,
	Near
}
impl CubeSideKind {
	pub fn transition(&self, facing: Facing) -> (CubeSideKind, Transform) {
		match (self, facing) {
			(Self::Top, Facing::Up) => (Self::Far, Transform::Up),
			(Self::Top, Facing::Right) => (Self::Right, Transform::Up),
			(Self::Top, Facing::Down) => (Self::Near, Transform::Up),
			(Self::Top, Facing::Left) => (Self::Left, Transform::Up),

			(Self::Near, Facing::Up) => (Self::Top, Transform::Up),
			(Self::Near, Facing::Right) => (Self::Right, Transform::Left),
			(Self::Near, Facing::Down) => (Self::Bottom, Transform::Up),
			(Self::Near, Facing::Left) => (Self::Left, Transform::Right),

			(Self::Far, Facing::Up) => (Self::Bottom, Transform::Up),
			(Self::Far, Facing::Right) => (Self::Right, Transform::Right),
			(Self::Far, Facing::Down) => (Self::Top, Transform::Up),
			(Self::Far, Facing::Left) => (Self::Left, Transform::Left),

			(Self::Left, Facing::Up) => (Self::Far, Transform::Right),
			(Self::Left, Facing::Right) => (Self::Top, Transform::Up),
			(Self::Left, Facing::Down) => (Self::Near, Transform::Left),
			(Self::Left, Facing::Left) => (Self::Bottom, Transform::Down),

			(Self::Right, Facing::Up) => (Self::Far, Transform::Left),
			(Self::Right, Facing::Right) => (Self::Bottom, Transform::Down),
			(Self::Right, Facing::Down) => (Self::Near, Transform::Right),
			(Self::Right, Facing::Left) => (Self::Top, Transform::Up),

			(Self::Bottom, Facing::Up) => (Self::Near, Transform::Up),
			(Self::Bottom, Facing::Right) => (Self::Right, Transform::Down),
			(Self::Bottom, Facing::Down) => (Self::Far, Transform::Up),
			(Self::Bottom, Facing::Left) => (Self::Left, Transform::Down)
		}
	}
}

#[derive(Debug, Clone)]
struct CubeSide {
	kind: CubeSideKind,
	bounding_box: Rectangle2,
	transform: Facing
}
impl fmt::Display for CubeSide {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}({} * {:?})", self.kind, self.bounding_box, self.transform)
	}
}

#[derive(Debug)]
struct Cube {
	top: CubeSide,
	bottom: CubeSide,
	left: CubeSide,
	right: CubeSide,
	far: CubeSide,
	near: CubeSide
}
impl Cube {
	fn get_side(&self, side: CubeSideKind) -> &CubeSide {
		match side {
			CubeSideKind::Top => &self.top,
			CubeSideKind::Bottom => &self.bottom,
			CubeSideKind::Left => &self.left,
			CubeSideKind::Right => &self.right,
			CubeSideKind::Far => &self.far,
			CubeSideKind::Near => &self.near
		}
	}
}
impl fmt::Display for Cube {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "{}\n{}\n{}\n{}\n{}\n{}", self.top, self.far, self.right, self.near, self.left, self.bottom)
	}
}

fn modulo_point(point: Point2, size: isize) -> Point2 {
	Point2::new(
		(size + point.x()) % size,
		(size + point.y()) % size,
	)
}

fn rotate_point(point: Point2, transform: Transform, size: isize) -> Point2 {
	let s = size - 1;
	let p = match transform {
		Transform::Up => point,
		Transform::Left => Point2::new(point.y(), s - point.x()),
		Transform::Down => Point2::new(s - point.x(), s - point.y()),
		Transform::Right => Point2::new(s - point.y(), point.x())
	};

	modulo_point(p, size)
}

fn simulate_cube(grid: &mut Grid2<Cell>, commands: &[Command], cube: &Cube, start: (Point2, CubeSideKind)) -> anyhow::Result<(Point2, Facing)> {
	let mut cube_side = start.1;
	let mut position = start.0;
	let mut facing = Facing::Right;
	
	for command in commands {
		match command {
			Command::RotateLeft => { facing = facing.rotate(Facing::Left); },
			Command::RotateRight => { facing = facing.rotate(Facing::Right); },
			Command::Move(steps) => {
				for _ in 0 .. *steps {
					let side = cube.get_side(cube_side);
					let step_position = position + facing.direction();
					anyhow::ensure!(side.bounding_box.contains(&position), "Invalid side-position value pair");
					
					let mut new_position = step_position;
					let mut new_facing = facing;
					let mut new_side = cube_side;
					if !side.bounding_box.contains(&new_position) {
						let transition_facing = facing.rotate(side.transform.inverse());

						let (ncs, transition_transform) = cube_side.transition(transition_facing);
						new_side = ncs;
						
						let side2 = cube.get_side(new_side);
						
						let compound_transform = side.transform.inverse().rotate(transition_transform).rotate(side2.transform);
						new_facing = facing.rotate(compound_transform);
						
						new_position = side2.bounding_box.min + rotate_point(
							new_position - side.bounding_box.min,
							compound_transform,
							side.bounding_box.size().x()
						);

						log::trace!("side: {:?}", side.kind);
						log::trace!("position: {}", step_position - side.bounding_box.min);
						log::trace!("facing: {:?}", facing);
						log::trace!("side_transform: {:?}", side.transform);
						log::trace!("transition_facing: {:?}", transition_facing);

						log::trace!("new_side: {:?}", side2.kind);
						log::trace!("new_side_transform: {:?}", side2.transform);
						log::trace!("transition_transform: {:?}", transition_transform);

						log::trace!("new_facing: {:?}", new_facing);
						log::trace!("new_position: {}", new_position - side2.bounding_box.min);
					}

					match grid.get(new_position).context("Invalid new position")? {
						Cell::Ground => {
							position = new_position;
							facing = new_facing;
							cube_side = new_side;

							if log::log_enabled!(log::Level::Trace) {
								*grid.get_mut(position).unwrap() = Cell::Cursor(facing);
								log::trace!("Map:\n{}", grid);
								// *grid.get_mut(position).unwrap() = Cell::Ground;
							}
						}
						Cell::Rock => (),
						Cell::Void | Cell::Cursor(_) => unreachable!()
					};
				}
			}
		}
	}
	log::debug!("Final position: {}, facing: {:?}", position, facing);

	Ok((position, facing))
}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut grid = {
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
			Cell::Void,
			Rectangle2 {
				min: Point2::new(0, 0),
				max: Point2::new(width as isize, height as isize)
			}
		)?;
		log::debug!("Grid bounds: {}", grid.bounding_box());

		for (y, line) in input.lines().take(grid.bounding_box().size().y() as usize).enumerate() {
			for (x, ch) in line.chars().enumerate() {
				match ch {
					'.' => { *grid.get_mut(Point2::new(x as isize, y as isize)).unwrap() = Cell::Ground; }
					'#' => { *grid.get_mut(Point2::new(x as isize, y as isize)).unwrap() = Cell::Rock; }
					_ => ()
				}
			}
		}

		grid
	};
	log::trace!("Map:\n{}", grid);

	let commands = {
		let input_line = input.lines().skip(grid.bounding_box().size().y() as usize + 1).next().context("Invalid input")?;

		let mut commands = Vec::<Command>::new();

		let mut start_index = 0;
		for (index, ch) in input_line.char_indices() {
			if !ch.is_numeric() {
				commands.push(
					Command::try_from(&input_line[start_index .. index])?
				);
				commands.push(
					Command::try_from(&input_line[index .. index + 1])?
				);

				start_index = index + 1;
			}
		}
		if start_index != input_line.len() {
			commands.push(
				Command::try_from(&input_line[start_index ..])?
			);
		}

		commands
	};
	log::trace!("Commands: {:?}", commands);

	let cube_size = grid.bounding_box().size().x().max(grid.bounding_box().size().y()) / 4;
	let start = row_non_void(&grid, 0)?.min;
	// let start = Point2::new(1 * cube_size, 1 * cube_size);
	let cube = {
		anyhow::ensure!(grid.bounding_box().size().x().min(grid.bounding_box().size().y()) / 3 == cube_size, "Cube size sanity check failed");
		
		let side_offset = Point2::new(cube_size, cube_size);
		let cube_side = Rectangle2 { min: Point2::new(0, 0), max: side_offset };
		log::debug!("Cube side: {}", cube_side);

		let top = CubeSide {
			kind: CubeSideKind::Top,
			bounding_box: cube_side + start,
			transform: Transform::Up
		};

		macro_rules! find_side {
			(
				$kind: ident:
				$(
					($offset_x: literal, $offset_y: literal) $transform: ident
				),+
			) => {
				None
				$(
					.or_else(|| {
						let bounding_box = top.bounding_box + Point2::new($offset_x, $offset_y) * cube_size;
						match grid.get(bounding_box.min) {
							Some(Cell::Ground | Cell::Rock) => Some(CubeSide {
								kind: CubeSideKind::$kind,
								bounding_box,
								transform: Transform::$transform
							}),
							_ => None
						}
					})
				)+
			};
		}

		Cube {
			left: find_side!(
				Left:
				(-1, 1) Left,
				(-1, 2) Down
			).unwrap(),
			right: find_side!(
				Right:
				(1, 0) Up,
				(1, 2) Down
			).unwrap(),
			far: find_side!(
				Far:
				(-2, 1) Down,
				(-1, 3) Right
			).unwrap(),
			near: find_side!(
				Near:
				(0, 1) Up
			).unwrap(),
			bottom: find_side!(
				Bottom:
				(0, 2) Up
			).unwrap(),
			top
		}
	};
	log::debug!("Cube:\n{}", cube);
	
	let part1 = {
		let (position, facing) = simulate_2d(&mut grid, &commands, start)?;

		(position.x() + 1) * 4 + (position.y() + 1) * 1000 + facing.score() as isize
	};
	log::info!("part1: {}", part1);

	let part2 = {
		let (position, facing) = simulate_cube(&mut grid, &commands, &cube, (start, CubeSideKind::Top))?;

		(position.x() + 1) * 4 + (position.y() + 1) * 1000 + facing.score() as isize
	};
	log::info!("part2: {}", part2);

	println!("Final password 2: {}", part2);
	println!("Final password: {}", part1);
	log::info!("Done");

	Ok(())
}
