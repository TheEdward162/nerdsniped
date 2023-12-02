use std::{
	io::Read, collections::HashMap
};

use anyhow::Context;

use aoc_commons as aoc;
use aoc::{anyhow, log};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct MonkeyName([u8; 4]);
impl<'a> TryFrom<&'a str> for MonkeyName {
	type Error = anyhow::Error;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		let value = value.as_bytes();
		anyhow::ensure!(value.len() == 4, "Monkey name must have exactly 4 bytes");

		Ok(Self(
			[value[0], value[1], value[2], value[3]]
		))
	}
}

#[derive(Clone, Copy)]
enum Operation {
	Add,
	Sub,
	Mul,
	Div,
	Eq
}
impl<'a> TryFrom<&'a str> for Operation {
	type Error = anyhow::Error;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		match value {
			"+" => Ok(Self::Add),
			"-" => Ok(Self::Sub),
			"*" => Ok(Self::Mul),
			"/" => Ok(Self::Div),
			_ => anyhow::bail!("Invalid Operation: \"{}\"", value)
		}
	}
}

enum Monkey {
	Value(isize),
	Variable(MonkeyName),
	Compute {
		left: MonkeyName,
		op: Operation,
		right: MonkeyName
	}
}

fn evaluate_monkey(monkeys: &HashMap<MonkeyName, Monkey>, start: &MonkeyName) -> Option<isize> {
	match monkeys.get(&start).unwrap() {
		Monkey::Value(value) => Some(*value),
		Monkey::Variable(_) => None,
		Monkey::Compute { left, op, right } => {			
			let left = evaluate_monkey(monkeys, left);
			let right = evaluate_monkey(monkeys, right);

			match (left, right) {
				(Some(left), Some(right)) => {
					let value = match op {
						Operation::Add => left + right,
						Operation::Sub => left - right,
						Operation::Mul => left * right,
						Operation::Div => left / right,
						Operation::Eq => unreachable!()
					};
					Some(value)
				},
				_ => None
			}
		}
	}
}

fn evaluate_reverse(monkeys: &HashMap<MonkeyName, Monkey>, start: &MonkeyName, target: isize) -> isize {
	match monkeys.get(start).unwrap() {
		Monkey::Variable(_) => target,
		Monkey::Compute { left, op, right } => {
			let left_value = evaluate_monkey(monkeys, left);
			let right_value = evaluate_monkey(monkeys, right);

			match (left_value, op, right_value) {
				// eq
				(None, Operation::Eq, Some(right_value)) => evaluate_reverse(monkeys, left, right_value),
				(Some(left_value), Operation::Eq, None) => evaluate_reverse(monkeys, right, left_value),
				// add
				(None, Operation::Add, Some(right_value)) => evaluate_reverse(monkeys, left, target - right_value),
				(Some(left_value), Operation::Add, None) => evaluate_reverse(monkeys, right, target - left_value),
				// sub
				(None, Operation::Sub, Some(right_value)) => evaluate_reverse(monkeys, left, target + right_value),
				(Some(left_value), Operation::Sub, None) => evaluate_reverse(monkeys, right, -(target - left_value)),
				// mul
				(None, Operation::Mul, Some(right_value)) => evaluate_reverse(monkeys, left, target / right_value),
				(Some(left_value), Operation::Mul, None) => evaluate_reverse(monkeys, right, target / left_value),
				// div
				(None, Operation::Div, Some(right_value)) => evaluate_reverse(monkeys, left, target * right_value),
				(Some(left_value), Operation::Div, None) => evaluate_reverse(monkeys, right, left_value / target),
				// (Some(left_value), Operation::Eq, Some(right_value)) => (left_value == right_value) as isize,
				// (Some(left_value), Operation::Add, Some(right_value)) => left_value + right_value,
				// (Some(left_value), Operation::Sub, Some(right_value)) => left_value - right_value,
				// (Some(left_value), Operation::Mul, Some(right_value)) => left_value * right_value,
				// (Some(left_value), Operation::Div, Some(right_value)) => left_value / right_value,
				(None, _, None) => unreachable!(),
				(Some(_), _, Some(_)) => unreachable!()
			}
		}
		_ => unreachable!()
	}
}

const ROOT_MONKEY: MonkeyName = MonkeyName([b'r', b'o', b'o', b't']);
const HUMAN_MONKEY: MonkeyName = MonkeyName([b'h', b'u', b'm', b'n']);

fn main() -> anyhow::Result<()> {
	let mut file = aoc::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut monkeys: HashMap<MonkeyName, Monkey> = HashMap::new();
	for line in input.lines().filter(|s| !s.is_empty()) {
		if let Ok((name, left, op, right)) = aoc::match_tokens!(
			line.split([' ', ':']).filter(|s| !s.is_empty()); name: MonkeyName, left: MonkeyName, op: Operation, right: MonkeyName
		) {
			monkeys.insert(name, Monkey::Compute { left, op, right });
			continue;
		}

		if let Ok((name, value)) = aoc::match_tokens!(
			line.split([' ', ':']).filter(|s| !s.is_empty()); name: MonkeyName, value: aoc::macros::FromStrToTryFromAdapter<isize> {.0}
		) {
			monkeys.insert(name, Monkey::Value(value));
			continue;
		}

		anyhow::bail!("Invalid input line \"{}\"", line);
	}
	
	let root_value = evaluate_monkey(&monkeys, &ROOT_MONKEY).unwrap();

	*monkeys.get_mut(&HUMAN_MONKEY).unwrap() = Monkey::Variable(HUMAN_MONKEY);
	match monkeys.get_mut(&ROOT_MONKEY).unwrap() {
		Monkey::Compute { op, .. } => { *op = Operation::Eq; }
		_ => unimplemented!()
	}
	let human_value = evaluate_reverse(&monkeys, &ROOT_MONKEY, 0);
	
	println!("Root value: {}", root_value);
	println!("Human value: {}", human_value);
	log::info!("Done");

	Ok(())
}
