use std::{io::Read, str::FromStr, fmt::Debug};

use anyhow::Context;

use aoc_commons as aoc;
use aoc::{anyhow, log, macros::FromStrToTryFromAdapter};

struct MoveCommand {
	count: usize,
	from: usize,
	to: usize
}
impl FromStr for MoveCommand {
	type Err = anyhow::Error;

	fn from_str(value: &str) -> anyhow::Result<Self> {
		let mut it = value.split(' ');
		match it.next().context("Invalid command")? {
			"move" => {
				let (count, from, to) = aoc::match_tokens!(value.split(' '); "move", count: FromStrToTryFromAdapter<usize>, "from", from: FromStrToTryFromAdapter<usize>, "to", to: FromStrToTryFromAdapter<usize>)?;

				Ok(Self { count: count.0, from: from.0, to: to.0 })
			}
			_ => anyhow::bail!("Invalid command")
		}
	}
}
impl MoveCommand {
	pub fn apply(&self, state: &mut State) -> anyhow::Result<()> {
		let (from, to) = state.get_two_mut(self.from - 1, self.to - 1)?;
		
		for _ in 0 .. self.count {
			to.push(from.pop().context("cannot pop from empty stack")?);
		}

		Ok(())
	}
}

struct MoveCommand2 {
	count: usize,
	from: usize,
	to: usize
}
impl FromStr for MoveCommand2 {
	type Err = anyhow::Error;

	fn from_str(value: &str) -> anyhow::Result<Self> {
		let other: MoveCommand = value.parse()?;
		Ok(Self {
			count: other.count,
			from: other.from,
			to: other.to
		})
	}
}
impl MoveCommand2 {
	pub fn apply(&self, state: &mut State) -> anyhow::Result<()> {
		let (from, to) = state.get_two_mut(self.from - 1, self.to - 1)?;

		to.extend(from.drain(from.len() - self.count ..));

		Ok(())
	}
}


#[derive(Clone)]
struct State {
	stacks: Vec<Vec<char>>
}
impl State {
	pub fn new(count: usize) -> Self {
		State {
			stacks: vec![Vec::new(); count]
		}
	}

	pub fn get_stacks(&self) -> &[Vec<char>] {
		&self.stacks
	}

	pub fn get_two_mut(&mut self, first: usize, second: usize) -> anyhow::Result<(&mut Vec<char>, &mut Vec<char>)> {
		anyhow::ensure!(first < self.stacks.len(), "first index out of bounds");
		anyhow::ensure!(second < self.stacks.len(), "second index out of bounds");
		anyhow::ensure!(first != second, "indices cannot be equal");

		let res = if first < second {
			let (left, right) = self.stacks.split_at_mut(second);
			(&mut left[first], &mut right[0])
		} else {
			let (left, right) = self.stacks.split_at_mut(first);
			(&mut right[0], &mut left[second])
		};

		Ok(res)
	}

	pub fn push(&mut self, which: usize, value: char) -> anyhow::Result<()> {
		self.stacks.get_mut(which).context("invalid stack index")?.push(value);

		Ok(())
	}
}
impl Debug for State {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let height = self.stacks.iter().map(|s| s.len()).max().unwrap();
		let width = self.stacks.len();

		for y_inv in 1 ..= height {
			for x in 0 .. width {
				if let Some(value) = self.stacks[x].get(height - y_inv) {
					write!(f, "[{}] ", value)?;
				} else {
					write!(f, "    ")?;
				}
			}
			writeln!(f)?;
		}

		for c in 1 ..= width {
			write!(f, " {}  ", c)?;
		}
		writeln!(f)?;

		Ok(())
	}
}

fn main() -> anyhow::Result<()> {
	let mut file = aoc::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let (initial_state_str, procedure_str) = input.split_once("\n\n").context("Failed to split input by \\n\\n")?;

	let (initial_state_str, initial_state_indices_str) = initial_state_str.rsplit_once('\n').context("Failed to split initial state by \\n")?;
	let last_index: usize = initial_state_indices_str.rsplit_once(' ').context("Failed to split initial state indices by <space>")?.1.parse().context("Failed to parse last column index")?;

	let mut state = State::new(last_index);
	for state_str in initial_state_str.split('\n').rev() {
		for x in 0 .. state.get_stacks().len() {
			let crate_str = &state_str[(x * 4)..][..3];
			let crate_char = crate_str.chars().nth(1).context("Failed to parse initial state crate")?;

			if crate_char != ' ' {
				state.push(x, crate_char)?;
			}
		}
	}
	let mut state2 = state.clone();
	log::debug!("State:\n{:?}", state);

	for command_str in procedure_str.split("\n").filter(|s| !s.is_empty()) {
		let command: MoveCommand = command_str.parse()?;
		command.apply(&mut state).context("Failed to apply command")?;

		let command2: MoveCommand2 = command_str.parse()?;
		command2.apply(&mut state2).context("Failed to apply command")?;
	}

	let top_sequence: String = state.get_stacks().iter().map(|s| s.last().copied().unwrap_or(' ')).collect();
	println!("Top sequence: {}", top_sequence);

	let top_sequence: String = state2.get_stacks().iter().map(|s| s.last().copied().unwrap_or(' ')).collect();
	println!("Top sequence2: {}", top_sequence);

	Ok(())
}
