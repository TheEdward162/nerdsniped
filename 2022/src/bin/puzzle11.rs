use std::{io::Read, iter::Peekable, collections::VecDeque};

use anyhow::Context;

use aoc_commons as base;
use base::{anyhow, log, macros::FromStrToTryFromAdapter};

type WorryLevel = u64;

#[derive(Debug, Clone)]
enum Operand {
	Old,
	Value(WorryLevel)
}
impl<'a> TryFrom<&'a str> for Operand {
	type Error = anyhow::Error;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		match value {
			"old" => Ok(Self::Old),
			value => match value.parse::<WorryLevel>() {
				Ok(v) => Ok(Self::Value(v)),
				Err(_) => anyhow::bail!("Invalid operation target value: \"{}\"", value)
			}
		}
	}
}

#[derive(Debug, Clone)]
enum Operation {
	Add,
	Mul
}
impl<'a> TryFrom<&'a str> for Operation {
	type Error = anyhow::Error;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		match value {
			"+" => Ok(Self::Add),
			"*" => Ok(Self::Mul),
			v => anyhow::bail!("Invalid operation op value: \"{}\"", v)
		}
	}
}

#[derive(Debug, Clone)]
struct Expression {
	left: Operand,
	operation: Operation,
	right: Operand,
}
impl<'a> TryFrom<&'a str> for Expression {
	type Error = anyhow::Error;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		let (left, operation, right) = base::split_match_tokens!(value, ' '; "new", "=", left: Operand, op: Operation, right: Operand)?;

		Ok(Self { left, operation, right })
	}
}
impl Expression {
	fn resolve_target(target: &Operand, old_value: WorryLevel) -> WorryLevel {
		match target {
			Operand::Old => old_value,
			Operand::Value(v) => *v
		}
	}

	pub fn apply(&self, old_value: WorryLevel) -> WorryLevel {
		let left = Self::resolve_target(&self.left, old_value);
		let right = Self::resolve_target(&self.right, old_value);

		match self.operation {
			Operation::Add => left + right,
			Operation::Mul => left * right
		}
	}
}

#[derive(Debug, Clone)]
struct MonkeyTest {
	divisible_by: WorryLevel,
	monkey_true: usize,
	monkey_false: usize
}
impl<'a> TryFrom<&'a InputStructure<'a>> for MonkeyTest {
	type Error = anyhow::Error;

	fn try_from(value: &'a InputStructure<'a>) -> Result<Self, Self::Error> {
		anyhow::ensure!(value.key == "Test", "Top level key must be Test");

		let value_true = value.get("If true").context("Missing If true")?.value;
		let value_false = value.get("If false").context("Missing If false")?.value;

		let divisible_by = base::split_match_tokens!(value.value, ' '; "divisible", "by", divisible_by: FromStrToTryFromAdapter<WorryLevel>)?.0;
		let monkey_true = base::split_match_tokens!(value_true, ' '; "throw", "to", "monkey", monkey_true: FromStrToTryFromAdapter<usize>)?.0;
		let monkey_false = base::split_match_tokens!(value_false, ' '; "throw", "to", "monkey", monkey_false: FromStrToTryFromAdapter<usize>)?.0;

		Ok(Self { divisible_by, monkey_true, monkey_false })
	}
}
impl MonkeyTest {
	pub fn test(&self, value: WorryLevel) -> usize {
		if value % self.divisible_by == 0 {
			self.monkey_true
		} else {
			self.monkey_false
		}
	}
}

#[derive(Debug, Clone)]
struct Monkey {
	items: VecDeque<WorryLevel>,
	expression: Expression,
	test: MonkeyTest
}
impl Monkey {
	pub fn new(items: Vec<WorryLevel>, expression: Expression, test: MonkeyTest) -> Self {
		Self {
			items: VecDeque::from(items),
			expression,
			test
		}
	}

	pub fn tick(&mut self, reducer: impl Fn(WorryLevel) -> WorryLevel) -> Option<(usize, WorryLevel)> {
		let old_level = self.items.pop_front()?;
		let new_level = self.expression.apply(old_level);
		let new_level = reducer(new_level);
		log::trace!("Updated item: {} -> {}", old_level, new_level);
		
		let new_monkey = self.test.test(new_level);

		Some((new_monkey, new_level))
	}

	pub fn receive_item(&mut self, item: WorryLevel) {
		self.items.push_back(item);
	}
}

#[derive(Debug)]
struct InputStructure<'a> {
	pub key: &'a str,
	pub value: &'a str,
	pub subsections: Vec<InputStructure<'a>>
}
impl<'a> InputStructure<'a> {
	fn indent_level(line: &str) -> anyhow::Result<usize> {
		line.find(|ch: char| !ch.is_whitespace()).context("Empty line cannot be parsed as input structure")
	}

	pub fn parse<I: Iterator<Item = &'a str>>(lines: &mut Peekable<I>) -> anyhow::Result<Self> {
		let first_line = lines.next().context("Input structure requires at least one line")?;
		let indent_level = Self::indent_level(first_line)?;

		let (key, value) = first_line.split_once(':').context("Cannot parse line without ':' as input structure")?;

		let mut subsections = Vec::new();
		while let Some(next_line) = lines.peek() {
			match Self::indent_level(next_line) {
				Ok(next_line_indent) if next_line_indent > indent_level => {
					subsections.push(
						Self::parse(lines).context("Failed to parse subsection")?
					);
				}
				_ => break
			}
		}

		Ok(Self {
			key: key.trim(),
			value: value.trim(),
			subsections
		})
	}

	pub fn get<'me>(&'me self, subkey: &str) -> Option<&'me InputStructure<'a>> {
		self.subsections.iter().find(|s| s.key == subkey)
	}
}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut monkeys: Vec<Monkey> = Vec::new();
	for monkey_str in input.split("\n\n").filter(|s| !s.is_empty()) {
		let mut sublines = monkey_str.split('\n').peekable();
		let input_structure = InputStructure::parse(&mut sublines).context("Failed to parse monkey input section")?;
		anyhow::ensure!(input_structure.key.starts_with("Monkey "), "Invalid monkey input");
		log::trace!("input structure: {:#?}", input_structure);

		let items = {
			let value = input_structure.get("Starting items").context("Missing Starting items")?.value;

			let mut starting_items = Vec::new();
			for item_str in value.split(',') {
				starting_items.push(item_str.trim().parse::<WorryLevel>().context("Invalid starting item value")?);
			}

			starting_items
		};
		let expression = Expression::try_from(input_structure.get("Operation").context("Missing Operation")?.value)?;
		let test = MonkeyTest::try_from(input_structure.get("Test").context("Missing Test")?)?;

		monkeys.push(Monkey::new(items, expression, test));
	}
	log::debug!("Monkeys: {:#?}", monkeys);

	let mut monkeys2 = monkeys.clone();

	// part 1
	let mut inspected = vec![0; monkeys.len()];
	for _round in 0 .. 20 {
		for i in 0 .. monkeys.len() {
			while let Some((receiver, item)) = monkeys[i].tick(|x| x / 3) {
				inspected[i] += 1;
				monkeys[receiver].receive_item(item);
			}
		}
	}
	let monkey_business = {
		inspected.sort_by(|a, b| a.cmp(b).reverse());
		
		inspected[0] * inspected[1]
	};
	println!("Monkey business: {}", monkey_business);

	// part 2
	let test_modulo: WorryLevel = monkeys2.iter().map(|m| m.test.divisible_by).product();
	let mut inspected2 = vec![0usize; monkeys2.len()];
	for _round in 0 .. 10_000 {
		for i in 0 .. monkeys2.len() {
			while let Some((receiver, item)) = monkeys2[i].tick(|x| x % test_modulo) {
				inspected2[i] += 1;
				monkeys2[receiver].receive_item(item);
			}
		}
	}
	let monkey_business2 = {
		inspected2.sort_by(|a, b| a.cmp(b).reverse());
		
		inspected2[0] * inspected2[1]
	};
	println!("Monkey business 2: {}", monkey_business2);

	Ok(())
}
