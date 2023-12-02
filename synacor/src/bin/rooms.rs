use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Cell {
	Number(isize),
	Add,
	Mul,
	Sub
}

#[derive(Debug, Clone)]
enum Direction {
	North,
	West,
	South,
	East
}

const MAP: [Cell; 16] = [
	Cell::Number(22), Cell::Sub, Cell::Number(9), Cell::Mul,
	Cell::Add, Cell::Number(4), Cell::Sub, Cell::Number(18),
	Cell::Number(4), Cell::Mul, Cell::Number(11), Cell::Mul,
	Cell::Mul, Cell::Number(8), Cell::Sub, Cell::Number(1),
];
const MAP_SIZE: [usize; 2] = [4, 4];
const DESIRED_VALUE: isize = 30;

#[derive(Debug)]
struct SearchState {
	position: [usize; 2],
	steps: usize,
	value: isize,
	last_cell: Cell,
	path: Vec<Direction>
}
impl SearchState {
	pub fn new(value: isize) -> Self {
		Self {
			position: [0, 0],
			steps: 0,
			value,
			last_cell: Cell::Number(0),
			path: vec![]
		}
	}

	pub fn enter(&mut self) {
		let cell = MAP[self.position[0] + self.position[1] * MAP_SIZE[1]].clone();
		
		match (&self.last_cell, &cell) {
			(Cell::Add, Cell::Number(x)) => { self.value += x; }
			(Cell::Mul, Cell::Number(x)) => { self.value *= x; }
			(Cell::Sub, Cell::Number(x)) => { self.value -= x; }
			_ => ()
		}

		self.last_cell = cell;
		self.steps += 1;
	}

	pub fn next(&self, direction: Direction) -> Option<Self> {
		let new_pos = match direction {
			Direction::West => match self.position[0].checked_sub(1) {
				None => None,
				Some(x) => Some([x, self.position[1]])
			},
			Direction::East => match self.position[0] + 1 {
				x if x < MAP_SIZE[0] => Some([x, self.position[1]]),
				_ => None
			},
			Direction::South => match self.position[1].checked_sub(1) {
				None => None,
				Some(y) => Some([self.position[0], y])
			},
			Direction::North => match self.position[1] + 1 {
				y if y < MAP_SIZE[1] => Some([self.position[0], y]),
				_ => None
			}
		};

		match new_pos {
			None | Some([0, 0]) => None,
			Some(position) => Some(Self {
				position,
				steps: self.steps,
				value: self.value,
				last_cell: self.last_cell.clone(),
				path: {
					let mut path = self.path.clone();
					path.push(direction);
					path
				}
			})
		}
	}
}

fn main() {
	let mut queue = VecDeque::new();
	queue.push_back(SearchState::new(
		match &MAP[0] {
			Cell::Number(v) => *v,
			_ => unreachable!()
		}
	));

	while let Some(mut current) = queue.pop_front() {
		current.enter();

		if current.position == [3, 3] {
			if current.value == DESIRED_VALUE {
				eprintln!("Reached door with: {:?}", current);
			}
			continue;
		}

		if current.steps > 15 {
			continue;
		}

		if let Some(next) = current.next(Direction::West) {
			queue.push_back(next);
		}
		if let Some(next) = current.next(Direction::East) {
			queue.push_back(next);
		}
		if let Some(next) = current.next(Direction::South) {
			queue.push_back(next);
		}
		if let Some(next) = current.next(Direction::North) {
			queue.push_back(next);
		}
	}
}